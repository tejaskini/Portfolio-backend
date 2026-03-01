// API Response utilities
use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use serde_json::json;

pub struct ApiResponse;

impl ApiResponse {
    pub fn created<T: serde::Serialize>(message: &str, data: T) -> HttpResponse {
        HttpResponse::Created().json(json!({
            "status": "success",
            "message": message,
            "data": data
        }))
    }

    pub fn ok<T: serde::Serialize>(message: &str, data: T) -> HttpResponse {
        HttpResponse::Ok().json(json!({
            "status": "success",
            "message": message,
            "data": data
        }))
    }

    pub fn success<T: serde::Serialize>(message: &str, data: T) -> HttpResponse {
        Self::ok(message, data)
    }

    pub fn error(message: &str) -> HttpResponse {
        HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": message
        }))
    }

    pub fn message_only(status: StatusCode, status_text: &str, message: &str) -> HttpResponse {
        HttpResponse::build(status).json(json!({
            "status": status_text,
            "message": message
        }))
    }
}

