// Database configuration module
use mongodb::{Client, Database};
use std::env;

pub async fn connect_db() -> Database {
    let uri = env::var("MONGO_URI").expect("MONGO_URI not set");

    println!("Connecting to MongoDB...");

    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to MongoDB");

    let db = client.database("portfolio_db");

    println!("Successfully connected to MongoDB database: portfolio_db");

    db
}
