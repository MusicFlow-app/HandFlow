use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::models::{Category, Difficulty};
use crate::error::AppError;

#[derive(Serialize, Deserialize)]
struct EnumValue {
    id: u8,
    name: String,
}

/// Get all difficulty levels
pub async fn get_difficulties() -> HttpResponse {
    // Dynamically generate the list of difficulties using the all() method
    let difficulties: Vec<EnumValue> = Difficulty::all()
        .into_iter()
        .map(|difficulty| EnumValue {
            id: difficulty.to_u8(),
            name: difficulty.to_string(),
        })
        .collect();

    HttpResponse::Ok().json(difficulties)
}

/// Get all categories
pub async fn get_categories() -> HttpResponse {
    // Dynamically generate the list of categories using the all() method
    let categories: Vec<EnumValue> = Category::all()
        .into_iter()
        .map(|category| EnumValue {
            id: category.to_u8(),
            name: category.to_string(),
        })
        .collect();

    HttpResponse::Ok().json(categories)
}

/// Get difficulty by ID
pub async fn get_difficulty(path: web::Path<u8>) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let difficulty = Difficulty::from_u8(id).ok_or_else(|| AppError::NotFound("Difficulty not found".to_string()))?;
    Ok(HttpResponse::Ok().json(EnumValue {
        id: difficulty.to_u8(),
        name: difficulty.to_string(),
    }))
}

/// Get category by ID
pub async fn get_category(path: web::Path<u8>) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let category = Category::from_u8(id).ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;
    Ok(HttpResponse::Ok().json(EnumValue {
        id: category.to_u8(),
        name: category.to_string(),
    }))
}


