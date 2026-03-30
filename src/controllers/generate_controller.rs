#[macro_export]
macro_rules! generate_controller {
    ($category:expr, $db_name:expr, $attributes:expr) => {
        use axum::{
            extract::{Path, State},
            http::StatusCode,
            Json,
        };

        use $crate::AppError;
        use $crate::DbPool;

        use $crate::helper::get_attributes;
        use $crate::models::item::{Category, DatabaseObject, Db_Name, JsonEntryObject};
        use $crate::services::itemstore::{
            delete_record_by_id, get_record_by_id, insert_record, modify_record_by_id,
            print_all_values,
        };

        async fn get_handler(
            State(pool): State<DbPool>,
        ) -> Result<Json<Vec<DatabaseObject>>, AppError> {
            let result = print_all_values(&pool, $db_name, $category, false).await?;
            Ok(Json(result))
        }

        async fn get_by_id_handler(
            State(pool): State<DbPool>,
            Path(id): Path<i32>,
        ) -> Result<Json<DatabaseObject>, AppError> {
            let result = get_record_by_id(&pool, $db_name, $category, id).await?;
            Ok(Json(result))
        }

        async fn post_handler(
            State(pool): State<DbPool>,
            Json(obj): Json<JsonEntryObject>,
        ) -> Result<Json<DatabaseObject>, AppError> {
            let result =
                insert_record(&pool, $db_name, $category, obj, get_attributes($attributes))
                    .await?;
            Ok(Json(result))
        }

        async fn put_handler(
            State(pool): State<DbPool>,
            Path(id): Path<i32>,
            Json(obj): Json<JsonEntryObject>,
        ) -> Result<Json<DatabaseObject>, AppError> {
            let result = modify_record_by_id(
                &pool,
                $db_name,
                $category,
                get_attributes($attributes),
                id,
                obj,
            )
            .await?;
            Ok(Json(result))
        }

        async fn delete_handler(
            State(pool): State<DbPool>,
            Path(id): Path<i32>,
        ) -> Result<StatusCode, AppError> {
            delete_record_by_id(&pool, $db_name, $category, id).await?;
            Ok(StatusCode::NO_CONTENT)
        }

        pub fn router() -> axum::Router<DbPool> {
            axum::Router::new()
                .route("/", axum::routing::get(get_handler).post(post_handler))
                .route(
                    "/{id}",
                    axum::routing::get(get_by_id_handler)
                        .put(put_handler)
                        .delete(delete_handler),
                )
        }
    };
}
