-- up
CREATE TABLE supply_chain_events (
     id TEXT PRIMARY KEY,
     product_id TEXT NOT NULL,
     event TEXT NOT NULL,
     location TEXT NOT NULL,
     timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
     blockchain_tx_id TEXT NOT NULL,
     CONSTRAINT fk_product FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
);

-- down
DROP TABLE IF EXISTS supply_chain_events;
