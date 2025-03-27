use actix_web::{web, HttpResponse, Error};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use serde_json::json;
use serde::Deserialize;
use uuid::Uuid;
use crate::db::Database;

#[derive(Deserialize)]
struct FavoriteRequest {
    increment: bool,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<i64>,
    per_page: Option<i64>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}

pub async fn upload_file(mut payload: Multipart, db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let mut file_data = None;
    let mut filename = String::new();
    let mut file_size = 0;

    while let Some(item) = payload.next().await {
        let mut field = item?;
        if field.name() == "file" {
            filename = field.content_disposition().get_filename().unwrap_or("").to_string();
            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                let bytes = chunk?;
                file_size += bytes.len() as i64;
                data.extend_from_slice(&bytes);
            }
            file_data = Some(data);
        }
    }

    if let Some(data) = file_data {
        let content = String::from_utf8_lossy(&data);
        let metadata = json!({
            "title": filename.replace(".mscz", ""),
            "composer": "Unknown",
            "category": "Song",
            "difficulty": "Intermediate"
        });

        match db.log_tab(&filename, file_size, metadata.clone(), &content).await {
            Ok(tab) => Ok(HttpResponse::Ok().json(tab)),
            Err(e) => {
                eprintln!("Database error: {}", e);
                Ok(HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to save file"
                })))
            }
        }
    } else {
        Ok(HttpResponse::BadRequest().json(json!({
            "error": "No file provided"
        })))
    }
}

pub async fn get_recent_tabs(query: web::Query<PaginationParams>, db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(25);

    let sort_by = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");
    
    match db.list_recent_tabs(page, per_page, sort_by, sort_order).await {
        Ok((tabs, total_count)) => Ok(HttpResponse::Ok().json(json!({
            "tabs": tabs,
            "total": total_count,
            "page": page,
            "per_page": per_page,
            "total_pages": (total_count as f64 / per_page as f64).ceil() as i64
        }))),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch recent files"
            })))
        }
    }
}

async fn toggle_favorite(
    path: web::Path<Uuid>,
    db: web::Data<Database>,
    req: web::Json<FavoriteRequest>,
) -> Result<HttpResponse, Error> {
    let tab_id = path.into_inner();
    
    match db.update_favorite_count(tab_id, req.increment).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => {
            log::error!("Failed to update favorite count: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/upload", web::post().to(upload_file))
            .route("/tabs/recent", web::get().to(get_recent_tabs))
            .route("/tabs/{id}/favorite", web::post().to(toggle_favorite))
    );
}
