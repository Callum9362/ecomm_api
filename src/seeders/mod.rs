use sqlx::{Pool, Sqlite};

pub async fn seed_products(pool: &Pool<Sqlite>) {
    let products = vec![
        (
            "Laptop",
            "A high-performance laptop",
            1200.00,
            10,
            "tx123laptop",
            false,
            "Warehouse A",
        ),
        (
            "Phone",
            "A smartphone with excellent camera quality",
            800.00,
            25,
            "tx123phone",
            true,
            "Store B",
        ),
        (
            "Headphones",
            "Noise-cancelling headphones",
            150.00,
            50,
            "tx123headphones",
            false,
            "Warehouse C",
        ),
        (
            "Tablet",
            "A lightweight and compact tablet",
            300.00,
            15,
            "tx123tablet",
            true,
            "Store A",
        ),
    ];

    for (name, description, price, stock, blockchain_tx_id, is_verified, current_location) in products {
        sqlx::query!(
            "INSERT INTO products (name, description, price, stock, blockchain_tx_id, is_verified, current_location)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            name,
            description,
            price,
            stock,
            blockchain_tx_id,
            is_verified,
            current_location,
        )
            .execute(pool)
            .await
            .expect("Failed to seed products");
    }

    println!("Products seeded successfully");
}

