use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};

mod api;
mod config;
mod db;
mod error;
mod models;
mod parsers;
mod state;
mod utils;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration first
    let config = config::Config::new().map_err(|e| {
        log::error!("Failed to load config: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;

    // Initialize the logger with configured level
    std::env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();

    // Initialize application state
    let state = state::AppState::new().await.map_err(|e| {
        log::error!("Failed to initialize state: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;

    let bind_addr = format!("0.0.0.0:{}", config.port);

    log::info!("Starting server on {}", bind_addr);
    log::info!("CORS origin: {}", config.cors_origin);
    log::info!("Upload limit: {} bytes", config.upload_limit);

    // Start the Actix web server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config.cors_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Authorization", "Content-Type", "Accept"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(state.db.clone()))
            .service(Files::new("/static", "static").show_files_listing())
            .configure(api::configure)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
