use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::models::item::{item, Category, DatabaseObject, Db_Name, JsonEntryObject};
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

    let conn = pool.get().await?;
    let affected = conn
        .interact(move |conn| {
            diesel::update(item::table)
                .filter(item::category.eq(category2))
                .filter(item::dbName.eq(dbname2))
                .filter(item::id.eq(id))
                .set(dbitem)
                .returning(item::all_columns)
                .get_result(conn)
        })
        .await?
        .map_err(AppError::from)?;

    Ok(affected)
}

pub async fn delete_record_by_id(
    pool: &DbPool,
    db_name: Db_Name,
    category: Category,
    id: i32,
) -> Result<Option<()>, AppError> {
    let conn = pool.get().await?;
    let affected = conn
        .interact(move |conn| {
            diesel::delete(item::table)
                .filter(item::id.eq(id))
                .filter(item::dbName.eq(db_name))
                .filter(item::category.eq(category))
                .execute(conn)
        })
        .await?
        .map_err(AppError::from)?;

    Ok((affected == 1).then_some(()))
}
