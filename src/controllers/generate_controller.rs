#[macro_export]
macro_rules! generate_controller {
    ($category:expr, $db_name:expr, $attributes:expr, $controllername:expr) => {
        use rocket::fairing::AdHoc;
        use rocket::http::Status;
        use rocket::serde::json::Json;

        use $crate::services::itemstore::Result;
        use $crate::Db;

        use $crate::helper::get_attributes;
        use $crate::models::item::{Category, DatabaseObject, Db_Name, JsonEntryObject};
        use $crate::services::itemstore::{
            delete_record_by_id, get_record_by_id, insert_record, modify_record_by_id,
            print_all_values,
        };

        #[get("/")]
        async fn get(db: Db) -> Result<Json<Vec<DatabaseObject>>, Status> {
            match print_all_values(&db, $db_name, $category, false).await {
                Ok(result) => Ok(Json(result)),
                Err(_) => Err(Status::InternalServerError),
            }
        }

        #[get("/<id>")]
        async fn get_by_id(db: Db, id: i32) -> Result<Json<DatabaseObject>, Status> {
            match get_record_by_id(&db, $db_name, $category, id).await {
                Ok(v) => Ok(Json(v)),
                Err(_) => Err(Status::NotFound),
            }
        }

        #[post("/", format = "json", data = "<obj>")]
        async fn post(db: Db, obj: Json<JsonEntryObject>) -> Result<Json<DatabaseObject>, Status> {
            match insert_record(&db, $db_name, $category, obj.0, get_attributes($attributes)).await
            {
                Ok(result) => Ok(Json(result)),
                Err(_) => Err(Status::InternalServerError),
            }
        }

        #[put("/<id>", format = "json", data = "<obj>")]
        async fn put(
            db: Db,
            id: i32,
            obj: Json<JsonEntryObject>,
        ) -> Result<Json<DatabaseObject>, Status> {
            match modify_record_by_id(
                &db,
                $db_name,
                $category,
                get_attributes($attributes),
                id,
                obj.0,
            )
            .await
            {
                Ok(result) => Ok(Json(result)),
                Err(_) => Err(Status::InternalServerError),
            }
        }

        #[delete("/<id>")]
        async fn delete(db: Db, id: i32) -> Result<Status, Status> {
            match delete_record_by_id(&db, $db_name, $category, id).await {
                Ok(_) => Ok(Status::NoContent),
                Err(_) => Err(Status::InternalServerError),
            }
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
