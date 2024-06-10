use crate::{
    config, controllers,
    database::Database,
    middlewares,
    socket::{self, server::Server},
};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer, Responder, Result};
use actix_web_lab::middleware::from_fn;
use std::env;

async fn index() -> Result<impl Responder> {
    Ok(fs::NamedFile::open("./page/index.html")?.customize())
}

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

    // Get the HOST and PORT environment variables
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());

    // Start the HTTP server
    log::info!("Starting the server");
    log::info!("Listening on http://{}:{}", host, port);

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
            // Static files
            .service(fs::Files::new("/data/", "./data/").show_files_listing())
            .service(
                fs::Files::new("/", "./page/")
                    .show_files_listing()
                    .index_file("index.html"),
            )
            // if it doesn't find the route
            .default_service(web::get().to(index))
    })
    // Use the HOST and PORT environment variables to bind the server
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
