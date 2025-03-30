use crate::db::Database;
use crate::models::PaginationParams;
use crate::error::AppError;
use crate::models::FavoriteRequest;
use actix_web::{web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

pub async fn get_library(query: web::Query<PaginationParams>, db: web::Data<Database>) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(25);

    let sort_by = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");
    
    let (tabs, total_count) = db.list_library(page, per_page, sort_by, sort_order)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().json(json!({
        "tabs": tabs,
        "total": total_count,
        "page": page,
        "per_page": per_page,
        "total_pages": (total_count as f64 / per_page as f64).ceil() as i64
    })))
}

pub async fn toggle_favorite(
    path: web::Path<Uuid>,
    request: web::Json<FavoriteRequest>,
    db: web::Data<Database>
) -> Result<HttpResponse, AppError> {
    let tab_id = path.into_inner();
    db.update_favorite_count(tab_id, request.increment)
        .await
        .map_err(AppError::Database)?;

    Ok(HttpResponse::Ok().finish())
}
