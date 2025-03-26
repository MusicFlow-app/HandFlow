use crate::templates::html::load_header_content;
use crate::utils::file::clean_old_uploads;
use crate::state::AppState;
use actix_web::{web, Error, HttpResponse};
use handlebars::Handlebars;
use serde::Serialize;
use std::time::Duration;
use tokio::fs;

/// Handles GET requests to the home page of the web application.
///
/// This function:
///
/// 1. **Cleans Up Old Uploads**: Asynchronously deletes files in the "uploads" directory that are older than 600 seconds. If the cleanup fails, it logs the error and returns a `500 Internal Server Error` response with the message "Server error".
///
/// 2. **Reads HTML Template**: Asynchronously reads the `home.html` file, which serves as the main HTML template for the home page. If reading the file fails, it logs the error and returns a `500 Internal Server Error` response with the message "Server error".
///
/// 3. **Loads Header Content**: Asynchronously loads the header content by calling the `load_header_content` function.
///
/// 4. **Inserts Body Content**: Replaces the `{{body}}` placeholder in the header content with the content from `home.html`.
///
/// 5. **Returns Response**: Constructs and returns an HTTP response with the final HTML content, setting the content type to `text/html; charset=utf-8` and returning it as a `200 OK` response.
///
/// # Parameters
/// - `_req`: The incoming `HttpRequest`.
///
/// # Returns
/// - `Result<HttpResponse, Error>`: The final HTML response or an error if any step fails.
pub async fn handler_home(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    if let Err(e) = clean_old_uploads("uploads", Duration::from_secs(600)).await {
        log::error!("Failed to clean old uploads: {}", e);
        return Ok(HttpResponse::InternalServerError().body("Server error"));
    }

    // Get recent uploads
    let recent_files = match state.db.list_recent_usage(5).await {
        Ok(files) => files,
        Err(e) => {
            log::error!("Failed to fetch recent files: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Server error"));
        }
    };

    // Read template content
    let body_content = match fs::read_to_string("src/html/home.html").await {
        Ok(content) => content,
        Err(e) => {
            log::error!("Failed to read home.html: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Server error"));
        }
    };

    use time::format_description::well_known::Rfc2822;

    // Prepare template data
    #[derive(Serialize)]
    struct FileUsageDisplay {
        id: uuid::Uuid,
        filename: String,
        metadata: serde_json::Value,
        last_used_at: String,
    }

    #[derive(Serialize)]
    struct TemplateData {
        recent_files: Vec<FileUsageDisplay>,
    }

    let recent_files = recent_files
        .into_iter()
        .map(|file| FileUsageDisplay {
            id: file.id,
            filename: file.filename,
            metadata: file.metadata,
            last_used_at: file.last_used_at.format(&Rfc2822).unwrap_or_else(|_| String::from("Unknown date")),
        })
        .collect();

    let data = TemplateData { recent_files };

    // Render template
    let mut reg = Handlebars::new();
    reg.register_template_string("main", &body_content)
        .map_err(|e| {
            log::error!("Failed to register template: {}", e);
            actix_web::error::ErrorInternalServerError("Template error")
        })?;

    let body = reg.render("main", &data).map_err(|e| {
        log::error!("Failed to render template: {}", e);
        actix_web::error::ErrorInternalServerError("Template error")
    })?;

    // Insert into base template
    let header_content = load_header_content().await;
    let response = header_content.replace("{{body}}", &body);

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response))
}
