// Database configuration module
use mongodb::{Client, Database};
use std::env;

pub async fn connect_db() -> Database {
    let uri = env::var("MONGO_URI").expect("MONGO_URI not set");
    let client = Client::with_uri_str(uri).await.unwrap();
    client.database("portfolio_db")
}
