use axum::{
    routing::{get, post},
    Router,
};

use sqlx::SqlitePool;
use crate::controllers::product_controller::{get_all};

pub fn create_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route(
            "/products",
            get(get_all),
        )
        .with_state(pool)
}