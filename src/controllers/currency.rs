use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::{
    models::currency::Currency,
    services::currencystore::{
        delete_currency, get_all_currencies, insert_currency, update_currency,
    },
    AppError, DbPool,
};

#[derive(Deserialize)]
pub struct CurrencyBody {
    pub rate: String,
    pub symbol: String,
    pub name: String,
}

async fn get_handler(State(pool): State<DbPool>) -> Result<Json<Vec<Currency>>, AppError> {
    Ok(Json(get_all_currencies(&pool).await?))
}

async fn post_handler(
    State(pool): State<DbPool>,
    Json(body): Json<CurrencyBody>,
) -> Result<Json<Currency>, AppError> {
    Ok(Json(
        insert_currency(&pool, body.rate, body.symbol, body.name).await?,
    ))
}

async fn put_handler(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(body): Json<CurrencyBody>,
) -> Result<Json<Currency>, AppError> {
    Ok(Json(
        update_currency(&pool, id, body.rate, body.symbol, body.name).await?,
    ))
}

async fn delete_handler(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    delete_currency(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> axum::Router<DbPool> {
    axum::Router::new()
        .route("/", axum::routing::get(get_handler).post(post_handler))
        .route(
            "/{id}",
            axum::routing::put(put_handler).delete(delete_handler),
        )
}
