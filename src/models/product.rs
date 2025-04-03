use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: i64,            // This represents the primary key (auto-incrementing ID in your database)
    pub name: String,       // Name of the product
    pub description: String, // Description of the product
    pub price: f64,         // Price of the product
    pub stock: i32,         // Stock count
}
