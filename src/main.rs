mod helper;
mod controllers;
mod models;
mod services;

use std::sync::Arc;

use axum::Router;
use deadpool_diesel::sqlite::{Manager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tower_http::services::{ServeDir, ServeFile};

pub type DbPool = Arc<Pool>;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Internal(String),
}

impl From<deadpool_diesel::PoolError> for AppError {
    fn from(e: deadpool_diesel::PoolError) -> Self {
        AppError::Internal(e.to_string())
    }
}

impl From<deadpool_diesel::InteractError> for AppError {
    fn from(e: deadpool_diesel::InteractError) -> Self {
        AppError::Internal(e.to_string())
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => AppError::NotFound,
            _ => AppError::Internal(e.to_string()),
        }
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found").into_response(),
            AppError::Internal(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[tokio::main]
async fn main() {
    let db_url = "storage/database.sqlite";
    let manager = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool: DbPool = Arc::new(Pool::builder(manager).build().expect("Failed to build pool"));

    {
        let conn = pool
            .get()
            .await
            .expect("Failed to get connection for migrations");
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .expect("Interact error during migrations")
            .expect("Failed to run migrations");
    }

    let app = build_app(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5540")
        .await
        .expect("Failed to bind");
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Server error");
}

fn build_app(pool: DbPool) -> Router {
    use controllers::*;

    let router: Router<DbPool> = Router::new()
        .nest("/api/bank", bank::router())
        .nest("/api/regularcredit", regularcredit::router())
        .nest("/api/cardbalance", cardbalance::router())
        .nest("/api/uncleared", uncleared::router())
        .nest("/api/regularpayment", regularpayment::router())
        .nest("/api/miscdebit", miscdebit::router())
        .nest("/api/misccredit", misccredit::router())
        .nest("/api/debtto", debtto::router())
        .nest("/api/debt", debt::router())
        .nest("/api/cash", cash::router())
        .nest("/api/cardheld", cardheld::router())
        .nest("/api/settings", settings::router())
        .nest("/api", api::router())
        .fallback_service(ServeDir::new("wwwroot").fallback(ServeFile::new("wwwroot/index.html")));

    let router = attach_dev_cors(router);
    router.with_state(pool)
}

#[cfg(debug_assertions)]
fn attach_dev_cors(router: Router<DbPool>) -> Router<DbPool> {
    use axum::http::{HeaderValue, Method};
    use tower_http::cors::CorsLayer;

    let cors = CorsLayer::new()
        .allow_origin([
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
        .allow_credentials(true);

    router.layer(cors)
}

#[cfg(not(debug_assertions))]
fn attach_dev_cors(router: Router<DbPool>) -> Router<DbPool> {
    router
}
