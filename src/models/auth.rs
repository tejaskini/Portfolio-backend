use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub struct AuthenticatedUser {
    pub user_id: ObjectId,
    pub role: String,
    pub permissions: Vec<String>,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Role {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_type: String,                // "super_admin", "admin", "web_user"
    pub permissions: Vec<String>,    // ["project:create", "project:delete"]
}