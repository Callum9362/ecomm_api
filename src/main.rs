mod database;
mod seeders;

use crate::database::connect;
use crate::seeders::seed_products;

#[tokio::main]
async fn main() {

    let pool = connect().await;

    println!("Connected to the database successfully!");

    seed_products(&pool).await;

    println!("Database seeded successfully!");
}
