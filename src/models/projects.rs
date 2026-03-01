use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String, // Markdown supported
    pub tech_stack: Vec<String>,
    pub image_url: String,
    pub live_link: Option<String>,
    pub repo_link: Option<String>,
    pub is_featured: bool,
    pub order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// This struct is what the frontend will send us (without IDs or timestamps)
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub image_url: String,
    pub live_link: Option<String>,
    pub repo_link: Option<String>,
    pub is_featured: bool,
    pub order: i32,
}