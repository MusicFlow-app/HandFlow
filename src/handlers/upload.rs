use crate::templates::{
    html::generate_html_css_legend, html::load_header_content, html::sanitize_html,
};
use crate::templates::{parser::parse_mscx_metadata, parser::parse_mscx_parts};
use crate::utils::{file::is_valid_zip, file::sanitize_file_name, scales::scales_list};
use actix_multipart::Multipart;
use actix_web::HttpResponse;
use futures_util::StreamExt;
use rand::{distributions::Alphanumeric, Rng};
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use zip::ZipArchive;

/// The `UPLOAD_COUNTER` and `MAX_UPLOADS` constants are used to manage and limit the number of simultaneous file uploads
/// in the web application.
///
/// - **`UPLOAD_COUNTER`**: This is an atomic counter that tracks the current number of active uploads.
///   It is initialized to `0` and is incremented or decremented atomically using methods like `fetch_add` and `fetch_sub`.
///   The use of an atomic counter ensures that operations on this variable are thread-safe, making it suitable for
///   use in a concurrent environment like a web server.
///
/// - **`MAX_UPLOADS`**: This constant defines the maximum number of simultaneous uploads allowed in the application.
///   If the number of active uploads (tracked by `UPLOAD_COUNTER`) exceeds this value, the application will
///   reject new upload requests with a `429 Too Many Requests` response. This helps prevent server overload and ensures
///   that the server can handle uploads efficiently without being overwhelmed.
static UPLOAD_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_UPLOADS: usize = 100;

/// Asynchronously handles the upload and processing of an MSCZ file (a compressed file format).
///
/// This function performs the following steps:
///
/// 1. **Upload Limit Check**: Increments the upload counter to track the number of active uploads.
///    If the number of active uploads exceeds `MAX_UPLOADS`, the function returns a `429 Too Many Requests` response.
///
/// 2. **File Handling**: Iterates through the uploaded file data:
///    - If a file is detected, a unique file name is generated using a timestamp and random suffix.
///    - The file is saved to a designated upload directory, ensuring the directory exists with appropriate permissions.
///
/// 3. **File Writing**: The function writes the received chunks of data to the file asynchronously using `tokio::fs::File`.
///
/// 4. **ZIP File Processing**:
///    - Opens the saved MSCZ file as a ZIP archive.
///    - Validates the ZIP file's integrity and size.
///    - Searches for the `.mscx` file within the ZIP archive and reads its content.
///    - Saves the extracted `.mscx` file to the upload directory.
///
/// 5. **Response Preparation**:
///    - Parses the MSCX file for available parts and generates HTML options for those parts.
///    - Loads a template file, injects the necessary content, and generates the final HTML response.
///
/// 6. **Clean-Up**: Decrements the upload counter after processing is complete or if an error occurs.
///
/// 7. **Error Handling**:
///    - Logs errors encountered during the file processing.
///    - Returns appropriate HTTP responses (e.g., `InternalServerError`, `BadRequest`) based on the error context.
///
/// 8. **Final Response**: Returns an HTTP response with the generated HTML content, including metadata about the uploaded and processed file.
pub async fn handle_mscz_upload(mut payload: Multipart) -> HttpResponse {
    let current_uploads = UPLOAD_COUNTER.fetch_add(1, Ordering::SeqCst);

    if current_uploads >= MAX_UPLOADS {
        UPLOAD_COUNTER.fetch_sub(1, Ordering::SeqCst);
        return HttpResponse::TooManyRequests().body("Too many uploads in progress");
    }

    let mut mscx_content = String::new();
    let mut mscx_path: Option<PathBuf> = None;

    while let Some(Ok(mut field)) = payload.next().await {
        let content_disposition = field.content_disposition();
        let name = content_disposition.get_name(); // This is already an Option<&str>

        if let Some(name) = name {
            if name == "file" {
                let unique_suffix: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(8)
                    .map(char::from)
                    .collect();
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let file_name = sanitize_file_name(&format!(
                    "uploaded_file_{}_{}.mscz",
                    timestamp, unique_suffix
                ));

                let upload_dir = PathBuf::from("uploads");
                if !upload_dir.exists() {
                    if let Err(e) = fs::create_dir_all(&upload_dir).await {
                        log::error!("Failed to create upload directory: {:?}", e);
                        return HttpResponse::InternalServerError().body("Failed to save the file");
                    }

                    if let Err(e) =
                        fs::set_permissions(&upload_dir, std::fs::Permissions::from_mode(0o700))
                            .await
                    {
                        log::error!("Failed to set directory permissions: {:?}", e);
                        return HttpResponse::InternalServerError().body("Failed to save the file");
                    }
                }

                let mscz_path = upload_dir.join(file_name);

                let mut file = fs::File::create(mscz_path.clone()).await.unwrap();

                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    file.write_all(&data).await.unwrap();
                }

                drop(file);

                let file = match fs::File::open(&mscz_path).await {
                    Ok(file) => file.into_std().await,
                    Err(e) => {
                        log::error!("Failed to open uploaded file: {:?}", e);
                        return HttpResponse::InternalServerError().body("Failed to process file");
                    }
                };

                let mut zip = match ZipArchive::new(file) {
                    Ok(zip) => zip,
                    Err(e) => {
                        log::error!("Failed to open ZIP archive: {:?}", e);
                        return HttpResponse::InternalServerError().body("Failed to process file");
                    }
                };

                if !is_valid_zip(&mut zip) {
                    log::error!("ZIP archive is invalid or too large");
                    return HttpResponse::BadRequest().body("Invalid or too large ZIP file");
                }

                for i in 0..zip.len() {
                    let mut file = match zip.by_index(i) {
                        Ok(file) => file,
                        Err(e) => {
                            log::error!("Failed to read file from ZIP: {:?}", e);
                            return HttpResponse::InternalServerError()
                                .body("Failed to extract file");
                        }
                    };
                    if file.name().ends_with(".mscx") {
                        if let Err(e) = file.read_to_string(&mut mscx_content) {
                            log::error!("Failed to read .mscx content: {:?}", e);
                            return HttpResponse::InternalServerError()
                                .body("Failed to extract file");
                        }

                        let mscx_file_name =
                            format!("extracted_file_{}_{}.mscx", timestamp, unique_suffix);
                        let mscx_file_path = upload_dir.join(mscx_file_name);
                        if let Err(e) = tokio::fs::write(&mscx_file_path, &mscx_content).await {
                            log::error!("Failed to save extracted .mscx file: {:?}", e);
                            return HttpResponse::InternalServerError().body("Failed to save file");
                        }

                        mscx_path = Some(mscx_file_path);
                        break;
                    }
                }
            }
        }
    }

    UPLOAD_COUNTER.fetch_sub(1, Ordering::SeqCst);

    if mscx_content.is_empty() {
        return HttpResponse::BadRequest()
            .body("Failed to extract .mscx content from uploaded file");
    }

    let available_parts = parse_mscx_parts(&mscx_content);
    if available_parts.is_err() {
        return HttpResponse::InternalServerError().body("Failed to parse MSCX parts");
    }
    let available_parts = available_parts.unwrap();

    let part_options = available_parts
        .into_iter()
        .map(|(id, name)| {
            format!(
                "<option value=\"{}\">{}</option>",
                id,
                &sanitize_html(&name)
            )
        })
        .collect::<String>();

    let mut grouped_options = String::new();
    let mut last_note_count = 0;

    for (id, name, notes, _) in scales_list() {
        let note_count = notes.len();

        if note_count != last_note_count {
            if last_note_count != 0 {
                grouped_options.push_str("</optgroup>");
            }
            grouped_options.push_str(&format!("<optgroup label=\"{} Notes\">", note_count));
            last_note_count = note_count;
        }

        grouped_options.push_str(&format!(
            "<option value=\"{}\">{} ({} notes)</option>",
            id, name, note_count
        ));
    }

    if last_note_count != 0 {
        grouped_options.push_str("</optgroup>");
    }

    let (work_title, composer, arranger) = parse_mscx_metadata(&mscx_content);

    let body_path = "src/html/upload_tmpl.html";
    let mut body_file = match tokio::fs::File::open(body_path).await {
        Ok(file) => file,
        Err(e) => {
            log::error!("Failed to open template file: {:?}", e);
            UPLOAD_COUNTER.fetch_sub(1, Ordering::SeqCst);
            return HttpResponse::InternalServerError().body("Failed to open template file");
        }
    };

    let mut body_content = String::new();
    if let Err(e) = tokio::io::AsyncReadExt::read_to_string(&mut body_file, &mut body_content).await
    {
        log::error!("Failed to read template file: {:?}", e);
        return HttpResponse::InternalServerError().body("Failed to read template file");
    }

    let legend_html = generate_html_css_legend();

    let body_content = body_content
        .replace("{{work_title}}", &sanitize_html(&work_title))
        .replace("{{composer}}", &sanitize_html(&composer))
        .replace("{{arranger}}", &sanitize_html(&arranger))
        .replace("{{mscx_path}}", &mscx_path.unwrap().display().to_string())
        .replace("{{part_options}}", &part_options)
        .replace("{{legend_html}}", &legend_html)
        .replace("{{scale_options}}", &grouped_options);

    // Load header content
    let header_content = load_header_content().await;
    let response = header_content.replace("{{body}}", &body_content);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response)
}
