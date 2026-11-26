use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::models::item::{item, Category, DatabaseObject, Db_Name, JsonEntryObject};
use crate::services::txlog::{self, Action};
use crate::{AppError, DbPool};

pub async fn get_collection(pool: &DbPool) -> Vec<DatabaseObject> {
    let conn = pool.get().await.expect("pool connection");
    conn.interact(move |conn| item::table.load(conn))
        .await
        .expect("interact error")
        .expect("diesel error")
}

pub async fn print_all_values(
    pool: &DbPool,
    db_name: Db_Name,
    category: Category,
    sort_by_day: bool,
) -> Result<Vec<DatabaseObject>, AppError> {
    let conn = pool.get().await?;
    if sort_by_day {
        conn.interact(move |conn| {
            item::table
                .filter(item::category.eq(category))
                .filter(item::dbName.eq(db_name))
                .order(item::day.asc())
                .load(conn)
        })
        .await?
        .map_err(AppError::from)
    } else {
        conn.interact(move |conn| {
            item::table
                .filter(item::category.eq(category))
                .filter(item::dbName.eq(db_name))
                .load(conn)
        })
        .await?
        .map_err(AppError::from)
    }
}

pub async fn get_record_by_id(
    pool: &DbPool,
    db_name: Db_Name,
    category: Category,
    id: i32,
) -> Result<DatabaseObject, AppError> {
    let conn = pool.get().await?;
    conn.interact(move |conn| {
        item::table
            .filter(item::category.eq(category))
            .filter(item::dbName.eq(db_name))
            .filter(item::id.eq(id))
            .first(conn)
    })
    .await?
    .map_err(AppError::from)
}

pub async fn insert_record(
    pool: &DbPool,
    db_name: Db_Name,
    category: Category,
    new_db_obj: JsonEntryObject,
    attributes: Vec<&str>,
) -> Result<DatabaseObject, AppError> {
    //TODO: std::io::Result<Created<DBObj>>
    //TODO: verify attributes
    let _ = attributes;

    let mut new_obj = DatabaseObject {
        id: None,
        dbName: db_name,
        oldId: None,
        category,
        name: new_db_obj.name.unwrap(),
        day: new_db_obj.day.map(|x| x.parse::<i32>().unwrap()),
        amount: new_db_obj.amount.unwrap().to_string(),
        cardid: new_db_obj.cardid,
        currency_id: new_db_obj.currency_id,
        person: new_db_obj.person.and_then(normalise_person),
    };

    let post_value = new_obj.clone();
    let conn = pool.get().await?;
    let id: Option<i32> = conn
        .interact(move |conn| {
            diesel::insert_into(item::table)
                .values(&post_value)
                .returning(item::id)
                .get_result(conn)
        })
        .await?
        .map_err(AppError::from)?;

    new_obj.id = id;
    txlog::record(Action::Create, "item", &new_obj);
    Ok(new_obj)
}

pub async fn modify_record_by_id(
    pool: &DbPool,
    db_name: Db_Name,
    category: Category,
    attributes: Vec<&str>,
    id: i32,
    new_db_obj: JsonEntryObject,
) -> Result<DatabaseObject, AppError> {
    //TODO: verify attributes
    let _ = attributes;

    let category2 = category.clone();
    let dbname2 = db_name.clone();

    let conn = pool.get().await?;
    let ids: DatabaseObject = conn
        .interact(move |conn| {
            item::table
                .filter(item::category.eq(category))
                .filter(item::dbName.eq(db_name))
                .filter(item::id.eq(id))
                .first(conn)
        })
        .await?
        .map_err(AppError::from)?;

    let mut dbitem = ids.clone();

    if let Some(d) = new_db_obj.name {
        dbitem.name = d;
    }
    if let Some(d) = new_db_obj.amount {
        dbitem.amount = d.to_string();
    }
    dbitem.day = new_db_obj.day.map(|x| x.parse::<i32>().unwrap());
    dbitem.cardid = new_db_obj.cardid;
    if new_db_obj.currency_id.is_some() {
        dbitem.currency_id = new_db_obj.currency_id;
    }
    // The changeset below skips None fields, so person is set explicitly to allow clearing it.
    let person = match new_db_obj.person {
        Some(p) => normalise_person(p),
        None => dbitem.person.clone(),
    };
    dbitem.person = None;

    let conn = pool.get().await?;
    let affected = conn
        .interact(move |conn| {
            diesel::update(item::table)
                .filter(item::category.eq(category2))
                .filter(item::dbName.eq(dbname2))
                .filter(item::id.eq(id))
                .set((dbitem, item::person.eq(person)))
                .returning(item::all_columns)
                .get_result(conn)
        })
        .await?
        .map_err(AppError::from)?;

    txlog::record(Action::Update, "item", &affected);
    Ok(affected)
}

pub async fn delete_record_by_id(
    pool: &DbPool,
    db_name: Db_Name,
    category: Category,
    id: i32,
) -> Result<Option<()>, AppError> {
    let conn = pool.get().await?;
    let deleted: Option<DatabaseObject> = conn
        .interact(move |conn| {
            diesel::delete(item::table)
                .filter(item::id.eq(id))
                .filter(item::dbName.eq(db_name))
                .filter(item::category.eq(category))
                .returning(item::all_columns)
                .get_result(conn)
                .optional()
        })
        .await?
        .map_err(AppError::from)?;

    if let Some(obj) = &deleted {
        txlog::record(Action::Delete, "item", obj);
    }

    Ok(deleted.map(|_| ()))
}

/// Trims a person's name; blank means "no person".
fn normalise_person(person: String) -> Option<String> {
    let trimmed = person.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

/// Distinct people named on debt items (both lists), alphabetically, for suggestions.
pub async fn get_debt_people(pool: &DbPool) -> Result<Vec<String>, AppError> {
    let conn = pool.get().await?;
    let people: Vec<Option<String>> = conn
        .interact(move |conn| {
            item::table
                .filter(item::category.eq(Category::debt))
                .filter(item::person.is_not_null())
                .select(item::person)
                .distinct()
                .load(conn)
        })
        .await?
        .map_err(AppError::from)?;

    let mut people: Vec<String> = people.into_iter().flatten().collect();
    people.sort_by_key(|p| p.to_lowercase());
    people.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    Ok(people)
}
