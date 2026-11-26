use axum::{extract::State, Json};

use crate::{services::itemstore::get_debt_people, AppError, DbPool};

/// GET /api/people: names already used on the debt pages, for the person dropdown.
async fn get(State(pool): State<DbPool>) -> Result<Json<Vec<String>>, AppError> {
    Ok(Json(get_debt_people(&pool).await?))
}

pub fn router() -> axum::Router<DbPool> {
    axum::Router::new().route("/", axum::routing::get(get))
}
