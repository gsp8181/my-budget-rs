use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::models::item::{item, Category, DatabaseObject, Db_Name, JsonEntryObject};
use crate::Db;

pub type Result<T, E = rocket::response::Debug<diesel::result::Error>> = std::result::Result<T, E>;

pub async fn get_collection(db: &Db) -> Vec<DatabaseObject> {
    let ids: Vec<DatabaseObject> = db.run(move |conn| item::table.load(conn)).await.unwrap();

    ids
}

pub async fn print_all_values(
    db: &Db,
    db_name: Db_Name,
    category: Category,
    sort_by_day: bool,
) -> Result<Vec<DatabaseObject>> {
    if sort_by_day {
        let ids: Vec<DatabaseObject> = db
            .run(move |conn| {
                item::table
                    .filter(item::category.eq(category))
                    .filter(item::dbName.eq(db_name))
                    .order(item::day.asc())
                    .load(conn)
            })
            .await?;

        Ok(ids)
    } else {
        let ids: Vec<DatabaseObject> = db
            .run(move |conn| {
                item::table
                    .filter(item::category.eq(category))
                    .filter(item::dbName.eq(db_name))
                    .load(conn)
            })
            .await?;

        Ok(ids)
    }
}

pub async fn get_record_by_id(
    db: &Db,
    db_name: Db_Name,
    category: Category,
    id: i32,
) -> Result<DatabaseObject> {
    let ids: DatabaseObject = db
        .run(move |conn| {
            item::table
                .filter(item::category.eq(category))
                .filter(item::dbName.eq(db_name))
                .filter(item::id.eq(id))
                .first(conn)
        })
        .await?;

    Ok(ids)
}

pub async fn insert_record(
    db: &Db,
    db_name: Db_Name,
    category: Category,
    new_db_obj: JsonEntryObject,
    attributes: Vec<&str>,
) -> Result<DatabaseObject> {
    //TODO: std::io::Result<Created<DBObj>>
    //TODO: verify attributes

    let mut new_obj = DatabaseObject {
        id: None,
        dbName: db_name,
        oldId: None,
        category,
        name: new_db_obj.name.unwrap(),
        day: new_db_obj.day.map(|x| x.parse::<i32>().unwrap()),
        amount: new_db_obj.amount.unwrap().to_string(),
        cardid: new_db_obj.cardid,
    };

    let post_value = new_obj.clone();
    let id: Option<i32> = db
        .run(move |conn| {
            diesel::insert_into(item::table)
                .values(&post_value)
                .returning(item::id)
                .get_result(conn)
        })
        .await?;

    new_obj.id = id;

    //new_obj = Some(id.expect("returning guarantees id present"));
    Ok(new_obj) //Ok(Created::new("").body(new_obj))
}

pub async fn modify_record_by_id(
    db: &Db,
    db_name: Db_Name,
    category: Category,
    attributes: Vec<&str>,
    id: i32,
    new_db_obj: JsonEntryObject,
) -> Result<DatabaseObject> {
    //TODO: verify attributes

    let category2 = category.clone();
    let dbname2 = db_name.clone();

    let ids: DatabaseObject = db
        .run(move |conn| {
            item::table
                .filter(item::category.eq(category))
                .filter(item::dbName.eq(db_name))
                .filter(item::id.eq(id))
                .first(conn)
        })
        .await?;

    let mut dbitem = ids.clone();

    if let Some(d) = new_db_obj.name {
        dbitem.name = d;
    }

    if let Some(d) = new_db_obj.amount {
        dbitem.amount = d.to_string();
    }

    dbitem.day = new_db_obj.day.map(|x| x.parse::<i32>().unwrap());
    dbitem.cardid = new_db_obj.cardid;

    let affected = db
        .run(move |conn| {
            diesel::update(item::table)
                .filter(item::category.eq(category2))
                .filter(item::dbName.eq(dbname2))
                .filter(item::id.eq(id))
                .set(dbitem)
                .returning(item::all_columns)
                .get_result(conn)
        })
        .await?;

    Ok(affected)
}

pub async fn delete_record_by_id(
    db: &Db,
    db_name: Db_Name,
    category: Category,
    id: i32,
) -> Result<Option<()>> {
    let affected = db
        .run(move |conn| {
            diesel::delete(item::table)
                .filter(item::id.eq(id))
                .filter(item::dbName.eq(db_name))
                .filter(item::category.eq(category))
                .execute(conn)
        })
        .await?;

    Ok((affected == 1).then_some(()))
}
