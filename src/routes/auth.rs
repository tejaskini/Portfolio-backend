// Authentication routes
use actix_web::{post, web, HttpResponse};
use mongodb::bson::oid::ObjectId;
use crate::models::admin::{Admin, LoginRequest, RegisterRequest, PasswordResetRequest, PasswordResetVerifyRequest};
use crate::utils::jwt::create_jwt;
use crate::error::MyError;
use mongodb::bson::doc;
use base64::{engine::general_purpose, Engine as _};
use crate::utils::collections::ADMIN_CL;
use mongodb::bson::{Document};

#[post("/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

// fetch admin rules for new admin registration
pub async fn fetch_admin_rules(db: &mongodb::Database) -> Result<ObjectId, MyError> {
    let collection = db.collection::<Document>("permissions");

    let filter = doc! {
         "admin_type": "web_user" ,
         "is_admin": false
        };

    let doc = collection
        .find_one(filter, None)
        .await?
        .ok_or_else(|| MyError::NotFound("No admin rules found".to_string()))?;

    let id = doc.get_object_id("_id")
        .map_err(|_| MyError::NotFound("Admin rules ID not found".to_string()))?;

    Ok(id)
}

// #[post("/login")]
// pub async fn login(
//     db: web::Data<mongodb::Database>,
//     payload: web::Json<LoginRequest>,
// ) -> Result<HttpResponse, MyError> {
//     let collection = db.collection::<Admin>(ADMIN_CL);
    
//     // 1. Find user in DB
//     let admin = collection
//         .find_one(doc! { "username": &payload.username }, None)
//         .await?
//         .ok_or_else(|| MyError::NotFound("Admin not found".to_string()))?;

//     // 2. Verify password using base64 decoding
//     let decoded_password = general_purpose::STANDARD
//         .decode(&admin.password_hash)
//         .ok()
//         .and_then(|bytes| String::from_utf8(bytes).ok())
//         .unwrap_or_default();

//     if decoded_password != payload.password {
//         return Err(MyError::AuthError("Invalid credentials".to_string()));
//     }

//     // 3. Generate Token
//     let token = create_jwt(&admin.username)?;

//     Ok(HttpResponse::Ok().json(serde_json::json!({
//          "token": token,
//          "message": "Login successful"
//         })))
// }


#[post("/login")]
pub async fn login(
    db: web::Data<mongodb::Database>,
    payload: web::Json<LoginRequest>,
) -> Result<HttpResponse, MyError> {
    let admin_coll = db.collection::<Admin>(ADMIN_CL);
    // Note: We use a generic Document collection to handle dynamic permission fields
    let perm_coll = db.collection::<mongodb::bson::Document>("permissions");

    // 1. Find user in DB
    let admin = admin_coll
        .find_one(doc! { "username": &payload.username }, None)
        .await?
        .ok_or_else(|| MyError::NotFound("Admin not found".to_string()))?;

    // 2. Verify password (Base64 check as per your current logic)
    let decoded_password = general_purpose::STANDARD
        .decode(&admin.password_hash)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .unwrap_or_default();

    if decoded_password != payload.password {
        return Err(MyError::AuthError("Invalid credentials".to_string()));
    }


    let user_id = admin.id.ok_or_else(|| MyError::NotFound("Admin ID not found".to_string()))?;

    // 3. Fetch permissions using the admin_type ObjectId
    // Assuming admin.admin_type is a mongodb::bson::oid::ObjectId
    let permission_doc = perm_coll
        .find_one(doc! { "_id": &admin.admin_type }, None)
        .await?
        .ok_or_else(|| MyError::AuthError("Permission configuration missing".to_string()))?;

    // Extract user_type (e.g., "super_admin") and is_admin boolean
    let user_type = permission_doc
        .get_str("admin_type")
        .unwrap_or("web_user")
        .to_string();

    let is_admin = permission_doc
        .get_bool("is_admin")
        .unwrap_or(false);

    // 4. Generate Token with new claims
    // Update your create_jwt signature to: create_jwt(username, user_type, is_admin)
    let token = create_jwt(&admin.username,&admin.id.as_ref().unwrap().to_hex(),&user_type, is_admin)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
         "token": token,
         "message": "Login successful",
         "role": user_type
    })))
}

#[post("/register")]
pub async fn register(
    db: web::Data<mongodb::Database>,
    payload: web::Json<RegisterRequest>,
) -> Result<HttpResponse, MyError> {
    let collection = db.collection::<Admin>(ADMIN_CL);
    
    // Check if username already exists
    if let Ok(Some(_)) = collection.find_one(doc! { "username": &payload.username }, None).await {
        return Err(MyError::AuthError("Username already exists".to_string()));
    }

    // Check if email already exists
    if let Ok(Some(_)) = collection.find_one(doc! { "email": &payload.email }, None).await {
        return Err(MyError::AuthError("Email already registered".to_string()));
    }

    // Check if phone already exists
    if let Ok(Some(_)) = collection.find_one(doc! { "phone": &payload.phone }, None).await {
        return Err(MyError::AuthError("Phone already registered".to_string()));
    }
    
    // 1. Encode the password using base64
    let password_hash = general_purpose::STANDARD.encode(payload.password.as_bytes());

    let now = chrono::Utc::now().timestamp_millis();

    let admin_rules_id = fetch_admin_rules(&db).await?;

    let new_admin = Admin {
        id: None,
        username: payload.username.to_string(),
        email: payload.email.to_string(),
        phone: payload.phone.to_string(),
        password_hash,
        created_at: now,
        admin_type: Some(admin_rules_id)
    };

    // 2. Save to DB
    collection.insert_one(&new_admin, None).await?;
    Ok(HttpResponse::Created().json(serde_json::json!({ 
        "message": "Admin registered successfully",
        "username": payload.username
    })))
}

#[post("/verify-reset")]
pub async fn verify_reset(
    db: web::Data<mongodb::Database>,
    payload: web::Json<PasswordResetVerifyRequest>,
) -> Result<HttpResponse, MyError> {
    let collection = db.collection::<Admin>(ADMIN_CL);
    
    // Find admin by email or phone
    let admin = collection
        .find_one(
            doc! {
                "$or": [
                    { "email": &payload.email_or_phone },
                    { "phone": &payload.email_or_phone }
                ]
            },
            None,
        )
        .await?
        .ok_or_else(|| MyError::NotFound("User not found with provided email or phone".to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "User found",
        "username": admin.username
    })))
}

#[post("/reset-password")]
pub async fn reset_password(
    db: web::Data<mongodb::Database>,
    payload: web::Json<PasswordResetRequest>,
) -> Result<HttpResponse, MyError> {
    let collection = db.collection::<Admin>(ADMIN_CL);
    
    // Find admin by email or phone
    let admin = collection
        .find_one(
            doc! {
                "$or": [
                    { "email": &payload.email_or_phone },
                    { "phone": &payload.email_or_phone }
                ]
            },
            None,
        )
        .await?
        .ok_or_else(|| MyError::NotFound("User not found with provided email or phone".to_string()))?;

    // Encode new password using base64
    let new_password_hash = general_purpose::STANDARD.encode(payload.new_password.as_bytes());

    // Update password in database
    let update_result = collection
        .update_one(
            doc! { "_id": admin.id },
            doc! { "$set": { "password_hash": new_password_hash } },
            None,
        )
        .await?;

    if update_result.modified_count == 0 {
        return Err(MyError::InternalError);
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Password reset successfully"
    })))
}