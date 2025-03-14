use actix_files::Files;
use actix_web::{web, App, HttpServer};
use handlers::{generate::handle_generate, home::handler_home, upload::handle_mscz_upload};

mod db;
mod handlers;
mod state;
mod templates;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger for capturing and displaying log messages
    env_logger::init();

    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    // Initialize application state with database connection
    let state = state::AppState::new().await.map_err(|e| {
        log::error!("Failed to initialize application state: {:?}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to initialize application state")
    })?;

    // Get port from environment variable or use default
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    log::info!("Starting server on {}", bind_addr);

    // Start an Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            // Define the home page route, mapped to `handler_home`
            .route("/", web::get().to(handler_home))
            // Route for handling MSCZ file uploads, mapped to `handle_mscz_upload`
            .service(web::resource("/upload").route(web::post().to(handle_mscz_upload)))
            // Route for generating content from uploaded files, mapped to `handle_generate`
            .service(web::resource("/generate").route(web::post().to(handle_generate)))
            // Serve static files from the "static" directory with directory listing enabled
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind(&bind_addr)?
    .run()
    .await
}
