use axum::{
    extract::State,
    response::Json,
};

use sqlx::SqlitePool;

use crate::models::product::{Product};

pub async fn get_all(State(pool): State<SqlitePool>) -> Json<Vec<Product>> {
    let products = sqlx::query_as!(
        Product,
        r#"
            SELECT id, name, description, price, stock
            FROM products
        "#
    )
        .fetch_all(&pool)
        .await
        .expect("Failed fetching todos from the database");

    Json(products)
}