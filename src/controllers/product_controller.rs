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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;
    use sqlx::{SqlitePool};

    async fn setup_test_database() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();

        sqlx::query(
            r#"
            CREATE TABLE products (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                price REAL NOT NULL,
                stock INTEGER NOT NULL
            )
            "#
        )
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query(
            r#"
            INSERT INTO products (name, description, price, stock)
            VALUES
                ('Product1', 'Description1', 10.0, 100),
                ('Product2', 'Description2', 20.0, 200)
            "#
        )
            .execute(&pool)
            .await
            .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_get_all() {
        let pool = setup_test_database().await;

        let response = get_all(State(pool)).await;

        let expected = serde_json::json!([
            {
                "id": 1,
                "name": "Product1",
                "description": "Description1",
                "price": 10.0,
                "stock": 100
            },
            {
                "id": 2,
                "name": "Product2",
                "description": "Description2",
                "price": 20.0,
                "stock": 200
            }
        ]);

        let actual: serde_json::Value = serde_json::to_value(response.0).unwrap();
        let expected: serde_json::Value = serde_json::to_value(expected).unwrap();

        assert_eq!(actual, expected);
    }
}
