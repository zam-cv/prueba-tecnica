use crate::{
    config, controllers,
    database::Database,
    middlewares,
    socket::{self, server::Server},
};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use std::env;

pub async fn app() -> std::io::Result<()> {
    // Create Database instance
    let database = Database::new();
    log::info!("Database connection established");

    // Configure the database
    config::configure_database(&database).await;
    log::info!("Database configured");

    // Create the socket server
    let (mut socket_server, server_tx) = Server::new(database.clone());
    tokio::spawn(async move { socket_server.run().await });
    log::info!("Socket server started");

    HttpServer::new(move || {
        App::new()
            // Enable the logger and CORS middleware
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            // Resources
            .app_data(web::Data::new(server_tx.clone()))
            .app_data(web::Data::new(database.clone()))
            // Routes
            .route(
                "/ws/{id}",
                web::get()
                    .to(socket::server_index)
                    // Wrap the websocket route with the user_auth middleware
                    .wrap(from_fn(middlewares::auth)),
            )
            .service(
                web::scope("/api")
                    .service(controllers::auth::routes())
                    .service(
                        web::scope("")
                            .wrap(from_fn(middlewares::auth))
                            .service(controllers::rooms::routes()),
                    ),
            )
            .service(
                // Static files
                fs::Files::new("/data/", "./data/").show_files_listing(),
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
