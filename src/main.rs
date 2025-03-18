mod database;

use crate::database::connect;

#[tokio::main]
async fn main() {

    connect().await;

    println!("Connected to the database successfully!");
}
