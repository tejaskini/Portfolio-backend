// Education routes
use actix_web::{get, post, put, delete, web, HttpResponse};
use mongodb::{Database, bson::{doc, oid::ObjectId}};
use futures_util::stream::TryStreamExt;

use crate::models::education::{Education, EducationRequest};
use crate::middleware::auth_middleware::AuthenticatedAdmin;
use crate::utils::response::ApiResponse;
use crate::utils::collections::EDU_CL;



#[post("/education")]
pub async fn create_education(
    _admin: AuthenticatedAdmin,
    db: web::Data<Database>,
    payload: web::Json<EducationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let collection = db.collection::<Education>(EDU_CL);
    let mut new_edu = Education {
        id: None,
        institution: payload.institution.clone(),
        degree: payload.degree.clone(),
        field_of_study: payload.field_of_study.clone(),
        start_year: payload.start_year,
        end_year: payload.end_year,
    };

    let result = collection.insert_one(&new_edu, None).await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    new_edu.id = result.inserted_id.as_object_id();
    
    Ok(ApiResponse::created("Education created", new_edu))
}

#[get("/education")]
pub async fn get_education(db: web::Data<Database>) -> Result<HttpResponse, actix_web::Error> {
    let collection = db.collection::<Education>(EDU_CL);
    let find_options = mongodb::options::FindOptions::builder().sort(doc! { "start_year": -1 }).build();
    let mut cursor = collection.find(None, find_options).await
        .map_err(actix_web::error::ErrorInternalServerError)?;
        
    let mut education_list: Vec<Education> = Vec::new();
    while let Some(edu) = cursor.try_next().await.map_err(actix_web::error::ErrorInternalServerError)? {
        education_list.push(edu);
    }
    Ok(ApiResponse::success("Education retrieved", education_list))
}

#[put("/education/{id}")]
pub async fn update_education(
    _admin: AuthenticatedAdmin,
    db: web::Data<Database>,
    path: web::Path<String>,
    payload: web::Json<EducationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let obj_id = ObjectId::parse_str(path.into_inner()).map_err(actix_web::error::ErrorBadRequest)?;
    let collection = db.collection::<Education>(EDU_CL);
    
    let end_year_bson = match payload.end_year {
        Some(year) => mongodb::bson::Bson::Int32(year),
        None => mongodb::bson::Bson::Null,
    };

    let update = doc! {
        "$set": {
            "institution": &payload.institution,
            "degree": &payload.degree,
            "field_of_study": &payload.field_of_study,
            "start_year": payload.start_year,
            "end_year": end_year_bson,
        }
    };

    let result = collection.update_one(doc! { "_id": obj_id }, update, None).await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if result.matched_count == 0 { return Ok(ApiResponse::message_only(actix_web::http::StatusCode::NOT_FOUND, "error", "Not found")); }
    Ok(ApiResponse::message_only(actix_web::http::StatusCode::OK, "success", "Education updated"))
}

#[delete("/education/{id}")]
pub async fn delete_education(
    _admin: AuthenticatedAdmin,
    db: web::Data<Database>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let obj_id = ObjectId::parse_str(path.into_inner()).map_err(actix_web::error::ErrorBadRequest)?;
    let result = db.collection::<Education>(EDU_CL).delete_one(doc! { "_id": obj_id }, None).await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if result.deleted_count == 0 { return Ok(ApiResponse::message_only(actix_web::http::StatusCode::NOT_FOUND, "error", "Not found")); }
    Ok(ApiResponse::message_only(actix_web::http::StatusCode::OK, "success", "Education deleted"))
}
