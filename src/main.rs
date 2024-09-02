extern crate actix_web;
extern crate env_logger;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use handlers::{generate::handle_generate, home::handler_home, upload::handle_mscz_upload};

mod handlers;
mod templates;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger for capturing and displaying log messages
    env_logger::init();

    // Start an Actix web server on port 8080
    HttpServer::new(|| {
        App::new()
            // Define the home page route, mapped to `handler_home`
            .route("/", web::get().to(handler_home))
            // Route for handling MSCZ file uploads, mapped to `handle_mscz_upload`
            .service(web::resource("/upload").route(web::post().to(handle_mscz_upload)))
            // Route for generating content from uploaded files, mapped to `handle_generate`
            .service(web::resource("/generate").route(web::post().to(handle_generate)))
            // Serve static files from the "static" directory with directory listing enabled
            .service(Files::new("/static", "static").show_files_listing())
    })
    // Bind the server to 0.0.0.0:8080 and start it
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
