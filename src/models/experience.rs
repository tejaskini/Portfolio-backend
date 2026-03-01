// Experience model
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experience {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub company: String,
    pub role: String,
    pub description: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub is_current: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExperienceRequest {
    pub company: String,
    pub role: String,
    pub description: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub is_current: bool,
}
