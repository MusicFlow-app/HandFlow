use crate::templates::html::load_header_content;
use crate::utils::file::clean_old_uploads;
use actix_web::{Error, HttpRequest, HttpResponse};
use std::time::Duration;
use tokio::fs;

/// Handles GET requests to the home page of the web application.
///
/// This function:
///
/// 1. **Cleans Up Old Uploads**: Asynchronously deletes files in the "uploads" directory that are older than 600 seconds. If the cleanup fails, it logs the error and returns a `500 Internal Server Error` response with the message "Server error".
///
/// 2. **Reads HTML Template**: Asynchronously reads the `main_tmpl.html` file, which serves as the main HTML template for the home page. If reading the file fails, it logs the error and returns a `500 Internal Server Error` response with the message "Server error".
///
/// 3. **Loads Header Content**: Asynchronously loads the header content by calling the `load_header_content` function.
///
/// 4. **Inserts Body Content**: Replaces the `{{body}}` placeholder in the header content with the content from `main_tmpl.html`.
///
/// 5. **Returns Response**: Constructs and returns an HTTP response with the final HTML content, setting the content type to `text/html; charset=utf-8` and returning it as a `200 OK` response.
///
/// # Parameters
/// - `_req`: The incoming `HttpRequest`.
///
/// # Returns
/// - `Result<HttpResponse, Error>`: The final HTML response or an error if any step fails.
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

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response))
}
