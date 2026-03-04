// // Authentication middleware
// use actix_web::{dev::Payload, Error as ActixError, FromRequest, HttpRequest};
// use futures_util::future::{ready, Ready};
// use mongodb::bson::Document;
// use crate::utils::jwt::Claims;
// use jsonwebtoken::{decode, DecodingKey, Validation};

// pub struct AuthenticatedAdmin(pub String);

// impl FromRequest for AuthenticatedAdmin {
//     type Error = ActixError;
//     type Future = Ready<Result<Self, Self::Error>>;

//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         let auth = req.headers().get("Authorization");
        
//         match auth {
//             Some(value) => {
//                 let token = value.to_str().unwrap_or("").replace("Bearer ", "");
//                 let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
                
//                 match decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()) {
//                     Ok(token_data) => ready(Ok(AuthenticatedAdmin(token_data.claims.sub))),
//                     Err(_) => ready(Err(actix_web::error::ErrorUnauthorized("Invalid token"))),
//                 }
//             },
//             None => ready(Err(actix_web::error::ErrorUnauthorized("No token provided"))),
//         }
//     }
// }




use actix_web::{dev::Payload, Error as ActixError, FromRequest, HttpRequest, web};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::bson::{doc, oid::ObjectId};
use crate::utils::jwt::Claims;
use crate::models::admin::Admin;
use crate:: models::auth::{AuthenticatedUser, Role};




impl FromRequest for AuthenticatedUser {
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {

        let db = req.app_data::<web::Data<mongodb::Database>>().cloned();

        let auth_header = req.headers().get("Authorization").cloned();

        Box::pin(async move {

            let db = db.ok_or_else(|| actix_web::error::ErrorInternalServerError("DB missing"))?;

            let token = auth_header
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("No token"))?
                .to_str()
                .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid header"))?
                .replace("Bearer ", "");

            let secret = std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "secret".to_string());

            let decoded = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            )
            .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

        println!("Decoded JWT Claims: {:?}", decoded.claims);

            let user_id = ObjectId::parse_str(&decoded.claims.user_id)
                .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid user id"))?;

             println!("Parsed User ID: {:?}", user_id);

            // 🔹 Fetch user
           let user_coll = db.collection::<Admin>("admins");

        let user = user_coll
                .find_one(doc! { 
                    "_id": &user_id 
                },
                 None)
                .await
                .map_err(|_| actix_web::error::ErrorUnauthorized("User not found"))?
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("User not found"))?;

            println!("Fetched User: {:?}", user);

            // Fetch role
            let role_coll = db.collection::<Role>("roles");

let role = role_coll
    .find_one(doc! { "_id": &user.role_id }, None)
    .await
    .map_err(|_| actix_web::error::ErrorUnauthorized("Role not found"))?
    .ok_or_else(|| actix_web::error::ErrorUnauthorized("Role not found"))?;

// Merge permissions
let mut final_permissions = role.permissions.clone();

if !final_permissions.contains(&"*".to_string()) {

    for perm in &user.custom_permissions {
        if !final_permissions.contains(perm) {
            final_permissions.push(perm.clone());
        }
    }

    final_permissions.retain(|p| !user.denied_permissions.contains(p));
}

            Ok(AuthenticatedUser {
                user_id,
                role: role.user_type.to_string(),
                permissions: final_permissions,
            })
        })
    }
}