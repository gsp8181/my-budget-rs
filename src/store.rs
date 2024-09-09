use rocket::fairing::AdHoc;
use rocket::response::status::Created;
use rocket::{Build, Rocket};

use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::structs::{item, Category, DBObjDBIntermediate, DBObjIn, Db_Name};
use crate::Db;

use lazy_static::lazy_static;

use std::sync::Mutex;

pub type Result<T, E = rocket::response::Debug<diesel::result::Error>> = std::result::Result<T, E>;

lazy_static! {
    static ref my_mutex: Mutex<i32> = Mutex::new(0i32);
}

pub async fn get_collection(db: Db) -> Vec<DBObjDBIntermediate> {
    let ids: Vec<DBObjDBIntermediate> = db.run(move |conn| item::table.load(conn)).await.unwrap();

    ids
}

pub async fn print_all_values(
    db: Db,
    db_name: Db_Name,
    category: Category,
    sort_by_day: bool,
) -> Result<Vec<DBObjDBIntermediate>> {
    if sort_by_day {
        let ids: Vec<DBObjDBIntermediate> = db
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
        let ids: Vec<DBObjDBIntermediate> = db
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
    db: Db,
    db_name: Db_Name,
    category: Category,
    id: i32,
) -> Result<DBObjDBIntermediate> {
    let ids: DBObjDBIntermediate = db
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
    db: Db,
    db_name: Db_Name,
    category: Category,
    new_db_obj: DBObjIn,
    attributes: Vec<&str>,
) -> Result<DBObjDBIntermediate> {
    //TODO: std::io::Result<Created<DBObj>>
    //TODO: verify attributes

    let mut new_obj = DBObjDBIntermediate {
        id: None,
        dbName: db_name,
        oldId: None,
        category: category,
        name: new_db_obj.name.unwrap(),
        day: match new_db_obj.day {
            //TODO: handle this with an impl or fromtype
            Some(x) => Some(x.parse::<i32>().unwrap()),
            None => None,
        },
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
    db: Db,
    db_name: Db_Name,
    category: Category,
    attributes: Vec<&str>,
    id: i32,
    new_db_obj: DBObjIn,
) -> Result<DBObjDBIntermediate> {
    let category2 = category.clone();
    let dbname2 = db_name.clone();

    let ids: DBObjDBIntermediate = db
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
        dbitem.name = d
    }

    if let Some(d) = new_db_obj.amount {
        dbitem.amount = d.to_string()
    }

    dbitem.day = match new_db_obj.day {
        Some(x) => Some(x.parse::<i32>().unwrap()),
        None => None,
    };
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
    db: Db,
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

    Ok((affected == 1).then(|| ()))
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    Db::get_one(&rocket)
        .await
        .expect("database connection")
        .run(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("diesel migrations");
        })
        .await;

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}
