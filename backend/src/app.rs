use crate::{controllers, database::Database};
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::env;

pub async fn app() -> std::io::Result<()> {
    // Create Database instance
    let db = Database::new();

    HttpServer::new(move || {
        App::new()
            // Enable the logger and CORS middleware
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            // Resources
            .app_data(web::Data::new(db.clone()))
            // Routes
            .service(web::scope("/api").service(controllers::auth::routes()))
    })
    // Use the HOST and PORT environment variables to bind the server
    .bind(format!(
        "{}:{}",
        env::var("HOST").unwrap_or("0.0.0.0".to_string()),
        env::var("PORT").unwrap_or("8080".to_string())
    ))?
    .run()
    .await
}
