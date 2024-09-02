use crate::templates::html::load_header_content;
use crate::utils::file::clean_old_uploads;
use actix_web::{Error, HttpRequest, HttpResponse};
use std::time::Duration;
use tokio::fs;

/// The `handler_home` function handles the GET requests to the home page of the web application.
///
/// This function performs the following steps:
/// 1. **Cleanup Old Uploads**: It asynchronously cleans up old files in the "uploads" directory that
///    are older than a specified duration (600 seconds in this case). If the cleanup fails, it logs the error
///    and returns a `500 Internal Server Error` response with the message "Server error".
///
/// 2. **Read HTML Template**: It asynchronously reads the `main_tmpl.html` file, which serves as the main
///    HTML template for the home page. If reading the file fails, it logs the error and returns a
///    `500 Internal Server Error` response with the message "Server error".
///
/// 3. **Load Header Content**: It loads the header content of the web page asynchronously by calling
///    the `load_header_content` function.
///
/// 4. **Replace Placeholder with Body Content**: It inserts the body content from the template into the
///    header content by replacing the `{{body}}` placeholder with the content from `main_tmpl.html`.
///
/// 5. **Return Response**: It constructs an HTTP response with the final HTML content, setting the
///    content type to `text/html` with UTF-8 encoding, and returns it as a successful `200 OK` response.
///
/// This function is designed to be used as a handler for the home route ("/") in an Actix-web application.
pub async fn handler_home(_req: HttpRequest) -> Result<HttpResponse, Error> {
    if let Err(e) = clean_old_uploads("uploads", Duration::from_secs(600)).await {
        log::error!("Failed to clean old uploads: {}", e);
        return Ok(HttpResponse::InternalServerError().body("Server error"));
    }

    let body_content = match fs::read_to_string("src/html/main_tmpl.html").await {
        Ok(content) => content,
        Err(e) => {
            log::error!("Failed to read main_tmpl.html: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Server error"));
        }
    };

    let header_content = load_header_content().await;
    let response = header_content.replace("{{body}}", &body_content);

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(response))
}
