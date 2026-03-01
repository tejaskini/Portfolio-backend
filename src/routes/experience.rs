// Experience routes
use actix_web::{get, post, put, delete, web, HttpResponse};
use futures_util::stream::TryStreamExt;
use mongodb::bson::doc;

use crate::models::experience::{Experience, ExperienceRequest};
use crate::error::MyError;
use crate::utils::response::ApiResponse;
use crate::middleware::auth_middleware::AuthenticatedAdmin;

const COLLECTION_NAME: &str = "experience";

#[post("/experience")]
pub async fn create_experience(
    _admin: AuthenticatedAdmin,
    db: web::Data<mongodb::Database>,
    payload: web::Json<ExperienceRequest>,
) -> Result<HttpResponse, MyError> {
    let collection = db.collection::<Experience>(COLLECTION_NAME);
    let new_exp = Experience {
        id: None,
        company: payload.company.clone(),
        role: payload.role.clone(),
        description: payload.description.clone(),
        start_date: payload.start_date.clone(),
        end_date: payload.end_date.clone(),
        is_current: payload.is_current,
    };

    let result = collection.insert_one(&new_exp, None).await?;
    let mut created_exp = new_exp;
    created_exp.id = result.inserted_id.as_object_id().map(|id| id.to_owned());

    Ok(ApiResponse::created("Experience created", created_exp))
}

#[get("/experience")]
pub async fn get_experience(db: web::Data<mongodb::Database>) -> Result<HttpResponse, MyError> {
    let collection = db.collection::<Experience>(COLLECTION_NAME);
    let mut cursor = collection.find(None, None).await?;
    let mut experiences = Vec::new();

    while let Some(exp) = cursor.try_next().await? {
        experiences.push(exp);
    }

    Ok(ApiResponse::ok("Experience retrieved", experiences))
}

#[put("/experience/{id}")]
pub async fn update_experience(
    _admin: AuthenticatedAdmin,
    db: web::Data<mongodb::Database>,
    id: web::Path<String>,
    payload: web::Json<ExperienceRequest>,
) -> Result<HttpResponse, MyError> {
    let collection = db.collection::<Experience>(COLLECTION_NAME);
    let obj_id = mongodb::bson::oid::ObjectId::parse_str(id.as_str())
        .map_err(|_| MyError::NotFound("Invalid experience ID".to_string()))?;

    let updated_exp = Experience {
        id: Some(obj_id),
        company: payload.company.clone(),
        role: payload.role.clone(),
        description: payload.description.clone(),
        start_date: payload.start_date.clone(),
        end_date: payload.end_date.clone(),
        is_current: payload.is_current,
    };

    collection
        .replace_one(doc! { "_id": obj_id }, &updated_exp, None)
        .await?;

    Ok(ApiResponse::ok("Experience updated", updated_exp))
}

#[delete("/experience/{id}")]
pub async fn delete_experience(
    _admin: AuthenticatedAdmin,
    db: web::Data<mongodb::Database>,
    id: web::Path<String>,
) -> Result<HttpResponse, MyError> {
    let collection = db.collection::<Experience>(COLLECTION_NAME);
    let obj_id = mongodb::bson::oid::ObjectId::parse_str(id.as_str())
        .map_err(|_| MyError::NotFound("Invalid experience ID".to_string()))?;

    collection.delete_one(doc! { "_id": obj_id }, None).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Experience deleted" })))
}
