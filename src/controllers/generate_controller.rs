#[macro_export]
macro_rules! generate_controller {
    ($category:expr, $db_name:expr, $attributes:expr, $controllername:expr) => {
        use rocket::fairing::AdHoc;
        use rocket::serde::json::Json;

        use crate::services::itemstore::Result;

        use crate::Db;

        use crate::helper::get_attributes;
        use crate::models::item::{Category, DatabaseObject, Db_Name, JsonEntryObject};
        use crate::services::itemstore::{
            delete_record_by_id, get_record_by_id, insert_record, modify_record_by_id,
            print_all_values,
        };

        #[get("/")]
        async fn get(db: Db) -> Json<Vec<DatabaseObject>> {
            let result: Vec<DatabaseObject> = print_all_values(&db, $db_name, $category, false)
                .await
                .unwrap();

            Json(result)
        }

        #[get("/<id>")]
        async fn get_by_id(db: Db, id: i32) -> Option<Json<DatabaseObject>> {
            let result = get_record_by_id(&db, $db_name, $category, id).await;

            match result {
                Ok(v) => Some(Json(v)),
                //TODO: err handling
                Err(_e) => None,
            }
        }

        #[post("/", format = "json", data = "<obj>")]
        async fn post(db: Db, obj: Json<JsonEntryObject>) -> Json<DatabaseObject> {
            let result =
                insert_record(&db, $db_name, $category, obj.0, get_attributes($attributes));

            Json(result.await.unwrap())
        }

        #[put("/<id>", format = "json", data = "<obj>")]
        async fn put(db: Db, id: i32, obj: Json<JsonEntryObject>) -> Result<Json<DatabaseObject>> {
            let result = modify_record_by_id(
                &db,
                $db_name,
                $category,
                get_attributes($attributes),
                id,
                obj.0,
            );

            match result.await {
                Ok(v) => Ok(Json(v)),
                Err(e) => Err(e),
            }
        }

        #[delete("/<id>")]
        async fn delete(db: Db, id: i32) -> Result<Option<()>> {
            delete_record_by_id(&db, $db_name, $category, id).await
        }

        pub fn stage() -> AdHoc {
            AdHoc::on_ignite($controllername, |rocket| async {
                rocket.mount(
                    format!("/api/{}", $controllername),
                    routes![get, get_by_id, post, put, delete],
                )
            })
        }
    };
}
