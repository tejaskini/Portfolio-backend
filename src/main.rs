use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env;

mod routes;
mod utils;
mod config;
mod models;
mod middleware;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db: mongodb::Database = config::db::connect_db().await;
        println!("✅ MongoDB connected successfully");

    let port = env::var("PORT").unwrap_or("8080".to_string());

    println!("🚀 Server running on port {}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5174")
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://127.0.0.1:5174")
            .allowed_origin("http://127.0.0.1:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .service(routes::auth::health_check)
            // Development endpoints (password utilities)
            .service(
                web::scope("/dev")
                    .service(routes::password_utils::encode_password)
                    .service(routes::password_utils::decode_password)
            )
            .service(
                web::scope("/api/v1")
                    .service(routes::auth::login)
                    .service(routes::auth::register)
                    .service(routes::auth::verify_reset)
                    .service(routes::auth::reset_password)
                    // Projects
                    .service(routes::projects::create_project)
                    .service(routes::projects::get_projects)
                    .service(routes::projects::update_project)
                    .service(routes::projects::delete_project)
                    // Experience
                    .service(routes::experience::create_experience)
                    .service(routes::experience::get_experience)
                    .service(routes::experience::update_experience)
                    .service(routes::experience::delete_experience)
                    // Education
                    .service(routes::education::create_education)
                    .service(routes::education::get_education)
                    .service(routes::education::update_education)
                    .service(routes::education::delete_education)
                    // Skills
                    .service(routes::skills::create_skill)
                    .service(routes::skills::get_skills)
                    .service(routes::skills::update_skill)
                    .service(routes::skills::delete_skill)
            )
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}