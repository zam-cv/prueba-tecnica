use crate::controllers;
use actix_web::{get, middleware::Logger, web, App, HttpServer};
use std::env;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

pub async fn app() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
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
