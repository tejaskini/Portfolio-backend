// Projects routes
use actix_web::{get, post, put, delete, web, HttpResponse};
use futures_util::stream::TryStreamExt;
use mongodb::bson::doc;
use chrono::Utc;
use crate::models::project::{Project, CreateProjectRequest};
use crate::error::MyError;
use crate::utils::response::ApiResponse;
use crate::utils::collections::PROJECTS_CL;
use crate::models::auth::AuthenticatedUser;
use crate::utils::access::check_permission;
use mongodb::options::FindOptions;
use crate::utils::constant::{READ, UPDATE, DELETE, CREATE};


#[post("/projects")]
    pub async fn create_project(
        _admin: AuthenticatedUser,
        db: web::Data<mongodb::Database>,
        payload: web::Json<CreateProjectRequest>,
    ) -> Result<HttpResponse, MyError> {
        
         if !check_permission(&_admin, CREATE) {
            println!("User does not have permission to create projects");
            return Err(MyError::AuthError("Permission denied".to_string()));
        }

        let collection = db.collection::<Project>(PROJECTS_CL);
        let new_project = Project {
            id: None,
            title: payload.title.to_string(),
            description: payload.description.to_string(),
            tech_stack: payload.tech_stack.to_vec(),
            image_url: payload.image_url.to_string(),
            live_link: Some(payload.live_link.as_ref().unwrap_or(&"".to_string()).to_string()),
            repo_link: Some(payload.repo_link.as_ref().unwrap_or(&"".to_string()).to_string()),
            created_at: Utc::now().timestamp_millis(),
            updated_at: Utc::now().timestamp_millis(),
        };

        let result = collection.insert_one(&new_project, None).await?;
        let mut created_project = new_project;
        created_project.id = result.inserted_id.as_object_id().map(|id| id.to_owned());

        Ok(ApiResponse::created("Project inserted", created_project))
    }

#[get("/projects")]
pub async fn get_projects(db: web::Data<mongodb::Database>) -> Result<HttpResponse, MyError> {
    let collection = db.collection::<Project>(PROJECTS_CL);

        let find_options = FindOptions::builder()
        .sort(doc! { "_id": -1 })
        .build();
    
    let mut cursor = collection.find(None, find_options).await?;
    let mut projects = Vec::new();

    while let Some(project) = cursor.try_next().await? {
        projects.push(project);
    }

    Ok(ApiResponse::ok("Projects retrieved", projects))
}

#[put("/projects/{id}")]
pub async fn update_project(
    _admin: AuthenticatedUser,
    db: web::Data<mongodb::Database>,
    id: web::Path<String>,
    // user_type: web::Data<String>,
    // is_permitted: web::Data<bool>,
    // user_id: web::Data<String>,
    payload: web::Json<CreateProjectRequest>,
) -> Result<HttpResponse, MyError> {


   if !check_permission(&_admin, UPDATE) {
            return Err(MyError::AuthError("Permission denied".to_string()));
        }


    let collection = db.collection::<Project>(PROJECTS_CL);
    let obj_id = mongodb::bson::oid::ObjectId::parse_str(id.as_str())
        .map_err(|_| MyError::NotFound("Invalid project ID".to_string()))?;

    let updated_project = Project {
        id: Some(obj_id),
        title: payload.title.clone(),
        description: payload.description.clone(),
        tech_stack: payload.tech_stack.clone(),
        image_url: payload.image_url.clone(),
        live_link: payload.live_link.clone(),
        repo_link: payload.repo_link.clone(),
        created_at: Utc::now().timestamp_millis(),
        updated_at: Utc::now().timestamp_millis(),
    };

    collection
        .replace_one(doc! { "_id": obj_id }, &updated_project, None)
        .await?;

    Ok(ApiResponse::ok("Project updated", updated_project))
}

#[delete("/projects/{id}")]
pub async fn delete_project(
    _admin: AuthenticatedUser,
    db: web::Data<mongodb::Database>,
    id: web::Path<String>,
) -> Result<HttpResponse, MyError> {


     if !check_permission(&_admin, DELETE) {
            return Err(MyError::AuthError("Permission denied".to_string()));
        }

    let collection = db.collection::<Project>(PROJECTS_CL);
    let obj_id = mongodb::bson::oid::ObjectId::parse_str(id.as_str())
        .map_err(|_| MyError::NotFound("Invalid project ID".to_string()))?;

    collection.delete_one(doc! { "_id": obj_id }, None).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Project deleted" })))
}
