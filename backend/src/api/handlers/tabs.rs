use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::db::Database;
use crate::error::AppError;
use crate::models::ScoreData;

pub async fn get_tab_details(
    path: web::Path<Uuid>,
    db: web::Data<Database>
) -> Result<HttpResponse, AppError> {
    let tab = db.get_tab(path.into_inner()).await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Tab not found".to_string()))?;
    
    let response = tab.to_response()
        .map_err(|e| AppError::Parse(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_part_measures(
    path: web::Path<(Uuid, i32)>,
    db: web::Data<Database>
) -> Result<HttpResponse, AppError> {
    let (tab_id, part_id) = path.into_inner();
    
    let tab = db.get_tab(tab_id).await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Tab not found".to_string()))?;
    
    let score_data: ScoreData = serde_json::from_value(tab.score_data)
        .map_err(|e| AppError::Parse(e.to_string()))?;
    
    let part = score_data.parts.into_iter()
        .find(|p| p.id == part_id as u32)
        .ok_or_else(|| AppError::NotFound("Part not found".to_string()))?;
    
    Ok(HttpResponse::Ok().json(part.measures))
}
