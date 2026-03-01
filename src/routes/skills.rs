// Skills routes
use actix_web::{get, post, put, delete, web, HttpResponse};
use futures_util::stream::TryStreamExt;
use mongodb::bson::doc;

use crate::models::skill::{Skill, SkillRequest};
use crate::error::MyError;
use crate::utils::response::ApiResponse;
use crate::middleware::auth_middleware::AuthenticatedAdmin;

const COLLECTION_NAME: &str = "skills";

#[post("/skills")]
pub async fn create_skill(
    _admin: AuthenticatedAdmin,
    db: web::Data<mongodb::Database>,
    payload: web::Json<SkillRequest>,
) -> Result<HttpResponse, MyError> {
    let collection = db.collection::<Skill>(COLLECTION_NAME);
    let new_skill = Skill {
        id: None,
        name: payload.name.clone(),
        proficiency: payload.proficiency.clone(),
        category: payload.category.clone(),
    };

    let result = collection.insert_one(&new_skill, None).await?;
    let mut created_skill = new_skill;
    created_skill.id = result.inserted_id.as_object_id().map(|id| id.to_owned());

    Ok(ApiResponse::created("Skill created", created_skill))
}

#[get("/skills")]
pub async fn get_skills(db: web::Data<mongodb::Database>) -> Result<HttpResponse, MyError> {
    let collection = db.collection::<Skill>(COLLECTION_NAME);
    let mut cursor = collection.find(None, None).await?;
    let mut skills = Vec::new();

    while let Some(skill) = cursor.try_next().await? {
        skills.push(skill);
    }

    Ok(ApiResponse::ok("Skills retrieved", skills))
}

#[put("/skills/{id}")]
pub async fn update_skill(
    _admin: AuthenticatedAdmin,
    db: web::Data<mongodb::Database>,
    id: web::Path<String>,
    payload: web::Json<SkillRequest>,
) -> Result<HttpResponse, MyError> {
    let collection = db.collection::<Skill>(COLLECTION_NAME);
    let obj_id = mongodb::bson::oid::ObjectId::parse_str(id.as_str())
        .map_err(|_| MyError::NotFound("Invalid skill ID".to_string()))?;

    let updated_skill = Skill {
        id: Some(obj_id),
        name: payload.name.clone(),
        proficiency: payload.proficiency.clone(),
        category: payload.category.clone(),
    };

    collection
        .replace_one(doc! { "_id": obj_id }, &updated_skill, None)
        .await?;

    Ok(ApiResponse::ok("Skill updated", updated_skill))
}

#[delete("/skills/{id}")]
pub async fn delete_skill(
    _admin: AuthenticatedAdmin,
    db: web::Data<mongodb::Database>,
    id: web::Path<String>,
) -> Result<HttpResponse, MyError> {
    let collection = db.collection::<Skill>(COLLECTION_NAME);
    let obj_id = mongodb::bson::oid::ObjectId::parse_str(id.as_str())
        .map_err(|_| MyError::NotFound("Invalid skill ID".to_string()))?;

    collection.delete_one(doc! { "_id": obj_id }, None).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Skill deleted" })))
}
