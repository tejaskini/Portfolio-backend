// Authentication middleware
use actix_web::{dev::Payload, Error as ActixError, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use crate::utils::jwt::Claims;
use jsonwebtoken::{decode, DecodingKey, Validation};

pub struct AuthenticatedAdmin(pub String);

impl FromRequest for AuthenticatedAdmin {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth = req.headers().get("Authorization");
        
        match auth {
            Some(value) => {
                let token = value.to_str().unwrap_or("").replace("Bearer ", "");
                let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
                
                match decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()) {
                    Ok(token_data) => ready(Ok(AuthenticatedAdmin(token_data.claims.sub))),
                    Err(_) => ready(Err(actix_web::error::ErrorUnauthorized("Invalid token"))),
                }
            },
            None => ready(Err(actix_web::error::ErrorUnauthorized("No token provided"))),
        }
    }
}
