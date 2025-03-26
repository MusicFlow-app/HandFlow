use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};

mod db;
mod routes;
mod state;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger for capturing and displaying log messages
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    // Initialize database connection
    let db = db::Database::new().await.map_err(|e| {
        log::error!("Failed to initialize database: {:?}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to initialize database")
    })?;

    // Get port from environment variable or use default
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    log::info!("Starting server on {}", bind_addr);

    // Start an Actix web server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173") // Frontend dev server
            .allowed_origin("http://localhost:8080") // Backend URL
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec!["Authorization", "Content-Type", "Accept"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .service(Files::new("/static", "static").show_files_listing())
            .configure(routes::api::configure)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
