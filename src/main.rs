mod database;
mod seeders;
mod controllers;
mod routes;
mod models;

use std::env;
use crate::database::connect;
use crate::seeders::seed_products;
use crate::{routes::create_routes};

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();
    let pool = connect().await;

    println!("Connected to the database successfully!");

    if args.len() > 1 && args[1] == "seed" {
        seed_products(&pool).await;
        println!("Database seeded successfully!");
    }

    let app = create_routes(pool);

    println!("Server running on http://127.0.0.1:3000");

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
