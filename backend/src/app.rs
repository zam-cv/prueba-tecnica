use crate::{config, controllers, database::Database, middlewares};
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use std::env;

pub async fn app() -> std::io::Result<()> {
    // Create Database instance
    let database = Database::new();

    // Configure the database
    config::configure_database(&database).await;

    HttpServer::new(move || {
        App::new()
            // Enable the logger and CORS middleware
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            // Resources
            .app_data(web::Data::new(database.clone()))
            // Routes
            .service(
                web::scope("/api")
                    .service(controllers::auth::routes())
                    .service(
                        web::scope("")
                            .wrap(from_fn(middlewares::auth))
                            .service(controllers::rooms::routes()),
                    ),
            )
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
