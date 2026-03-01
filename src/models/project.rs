// Project model
use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub image_url: String,
    pub live_link: Option<String>,
    pub repo_link: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateProjectRequest {
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub image_url: String,
    pub live_link: Option<String>,
    pub repo_link: Option<String>,
}
