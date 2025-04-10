# **MVP Requirements for Supply Chain Transparency Using Blockchain**

## **1. Define the Use Case**
The goal is to track a product's supply chain in a transparent and secure way. Each step in the supply chain will be recorded as an immutable event on the blockchain. Key objectives include:
- Track and verify the origin, status, and location of a product.
- Record each interaction (e.g., manufactured, shipped, inspected, delivered) on the blockchain.
- Allow end-users (e.g., customers) to view the supply chain history and confirm the authenticity of the product.

---

## **2. Extend the Database Schema**

~~### **Products Table**
~~Add new fields to the existing `Products` table~~
`blockchainTxId` (String): The transaction ID for this product's creation or tracking.
`isVerified` (Boolean): Whether the product has been verified via blockchain.
`currentLocation` (String): The current location/state of the product in the supply chain.~~~~

### **Supply Chain Events Table**

~~Create a new table to store product events in the supply chain:~~
~~- `id` (UUID): Unique ID for the supply chain event.~~
~~- `productId` (Foreign Key): Reference to the product being tracked.~~
~~- `event` (String): Description of the event (e.g., "Manufactured", "Shipped", "Delivered").~~
~~- `location` (String): Location where the event occurred.~~
~~- `timestamp` (DateTime): When the event was recorded.~~
~~- `blockchainTxId` (String): Blockchain transaction ID for this specific event.~~

---

## **3. Blockchain Integration**

### **Libraries**
Identify and integrate a blockchain library for Rust:
- **[`web3`](https://github.com/tomusdrw/rust-web3)**: For Ethereum blockchain interactions.
- **[`hyperledger/iroha`](https://hyperledger.github.io/iroha/)**: A blockchain tailored for supply chain use cases.

### **Operations to Implement**
1. **Record Supply Chain Events on Blockchain**:
  - For every supply chain event, record an immutable log on the blockchain that includes:
    - Product ID.
    - Event description.
    - Timestamp.
    - Location.
  - Store the transaction ID (`blockchainTxId`) in the database after recording.

2. **Retrieve Supply Chain History**:
  - Use the blockchain transaction ID (`blockchainTxId`) to query and retrieve the supply chain history for a product.

---

## **4. Add Routes for Supply Chain Tracking**

### **API Endpoints**

#### **`POST /products/:id/events`**
- **Purpose**: Record a new supply chain event for a product.
- **Request Payload**:
  ```json
  {
    "event": "Shipped",
    "location": "Los Angeles, CA"
  }
  ```
- **Actions**:
  - Record the event in the `Supply Chain Events` table.
  - Write event data to the blockchain.
  - Save the resulting `blockchainTxId` to the database.
- **Response**:
  ```json
  {
    "status": "success",
    "blockchainTxId": "0xabc123..."
  }
  ``` 

---

#### **`GET /products/:id/events`**
- **Purpose**: Retrieve the supply chain history for a specific product.
- **Actions**:
  - Fetch all supply chain events from the database.
  - Optionally verify these events by querying the blockchain.
- **Response**:
  ```json
  {
    "productId": "123",
    "events": [
      {
        "event": "Manufactured",
        "location": "Beijing, China",
        "timestamp": "2023-10-01T10:15:00Z",
        "blockchainTxId": "0xabc123..."
      },
      {
        "event": "Shipped",
        "location": "Los Angeles, CA",
        "timestamp": "2023-10-05T14:30:00Z",
        "blockchainTxId": "0xdef456..."
      }
    ]
  }
  ```

---

## **5. Functionality Implementation**

### **Supply Chain Event Recording**
Write a function to record supply chain events on the blockchain. For example:

```rust
async fn record_event_on_blockchain(
    product_id: &str,
    event: &str,
    location: &str,
    timestamp: &str
) -> Result<String, BlockchainError> {
    // Use a library like web3 to send data to the blockchain
    let data = format!(
        "Product: {}, Event: {}, Location: {}, Timestamp: {}",
        product_id, event, location, timestamp
    );

    // Send transaction to the blockchain (example)
    let blockchain_tx_id = send_transaction_to_blockchain(data).await?;

    Ok(blockchain_tx_id)
}
```

---

### **Supply Chain Event Retrieval**
Write a function to verify or retrieve events from the blockchain. For example:

```rust
async fn verify_event_on_blockchain(tx_id: &str) -> Result<SupplyChainEvent, BlockchainError> {
    // Query the blockchain using the transaction ID
    let event_data = query_blockchain_transaction(tx_id).await?;
    // Parse the data into a structured event
    parse_event_data(event_data)
}
```

---

## **6. Blockchain Interaction Examples**

### Record an Event
Every supply chain event (e.g., "Shipped") follows these steps:
1. Add the event to the database (`Supply Chain Events` table).
2. Use a blockchain integration library to send the event data to the blockchain.
3. Save the resulting transaction ID (`blockchainTxId`) in the database.

---

### Retrieve Supply Chain History
1. Fetch the supply chain event records from the database.
2. Optionally, verify events by querying the blockchain, using the stored transaction IDs (`blockchainTxId`).

---

## **7. Testing and Security**

### **Testing**
- Use blockchain testnets (e.g., Ethereum Goerli or Ropsten) for development and testing.
- Simulate the end-to-end functionality by:
  1. Recording multiple events for a product.
  2. Retrieving and verifying the product's event history.

### **Security**
- Ensure blockchain data is sanitized to avoid vulnerabilities.
- Secure sensitive data (e.g., private blockchain keys, API credentials) using environment variables or key management services.
- Limit write access to blockchain operations to authorized users or services only.

---

## **8. Example Roadmap**

### **Week 1: Research and Database Schema**
- Test blockchain libraries compatible with Rust for supply chain use cases.
- Extend the database schema with blockchain-related fields and a supply chain events table.

---

### **Week 2: API Endpoints and Database Integration**
- Create API endpoints to:
  - Record supply chain events (`POST /products/:id/events`).
  - Retrieve supply chain history (`GET /products/:id/events`).
- Implement database functions to handle events.

---

### **Week 3: Blockchain Integration**
- Write functions to interact with the blockchain:
  - Record supply chain events immutably.
  - Verify recorded events using transaction IDs.
- Test blockchain integration on testnets like Ethereum Ropsten.

---

### **Week 4: Testing and Deployment**
- Simulate end-to-end functionality:
  - Record supply chain events.
  - Retrieve supply chain history, verifying data from the blockchain.
- Deploy to production with a sample product to showcase transparency.

---

This plan creates a robust system for blockchain-based supply chain transparency. Each step uses blockchain's immutability and decentralization to build trust with your users. Let me know if you’d like specific help implementing parts of this plan!