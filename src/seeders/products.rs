use sqlx::{Pool, Postgres};

pub async fn seed_products(pool: &Pool<Postgres>) {
    // Sample product data
    let products = vec![
        ("Laptop", "A high-performance laptop", 1200.00, 10),
        ("Phone", "A smartphone with excellent camera quality", 800.00, 25),
        ("Headphones", "Noise-cancelling headphones", 150.00, 50),
        ("Tablet", "A lightweight and compact tablet", 300.00, 15),
    ];

    for (name, description, price, stock) in products {
        sqlx::query!(
            "INSERT INTO products (name, description, price, stock)
             VALUES ($1, $2, $3, $4)",
            name,
            description,
            price,
            stock
        )
            .execute(pool)
            .await
            .expect("Failed to seed products");
    }

    println!("Products seeded successfully");
}
