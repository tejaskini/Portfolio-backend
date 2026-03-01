// Education model
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Education {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub institution: String,
    pub degree: String,
    pub field_of_study: String,
    pub start_year: i32,
    pub end_year: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EducationRequest {
    pub institution: String,
    pub degree: String,
    pub field_of_study: String,
    pub start_year: i32,
    pub end_year: Option<i32>,
}