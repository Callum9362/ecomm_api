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

    let app = Router::new()
        .route("/products", get(get_all_products))
        .layer(axum::extract::Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
