// Skill model
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Skill {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub proficiency: String, // Beginner, Intermediate, Advanced, Expert
    pub category: String, // Frontend, Backend, DevOps, etc.
    pub month_of_experience: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SkillRequest {
    pub name: String,
    pub proficiency: String,
    pub category: String,
    pub month_of_experience: Option<u32>,
}
