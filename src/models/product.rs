use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: i64,
    pub name:  Option<String>,
    pub description:  Option<String>,
    pub price: f64,
    pub stock: i64,
    pub blockchain_tx_id: Option<String>,
    pub is_verified: bool,
    pub current_location: Option<String>,
}
