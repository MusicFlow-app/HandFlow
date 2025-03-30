use actix_web::web;
use super::handlers::{get_library, get_tab_details, toggle_favorite, get_part_measures, upload_file, generate_tab_html};
use super::handlers::metadata::{get_difficulties, get_categories, get_difficulty, get_category};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/upload", web::post().to(upload_file))
            .route("/library", web::get().to(get_library))
            .route("/tabs/{id}/favorite", web::post().to(toggle_favorite))
            .route("/tabs/{id}", web::get().to(get_tab_details))
            .route("/tabs/{id}/parts/{part_id}/measures", web::get().to(get_part_measures))
            .route("/tabs/{id}/parts/{part_id}/generate", web::get().to(generate_tab_html))
            .route("/metadata/difficulties", web::get().to(get_difficulties))
            .route("/metadata/categories", web::get().to(get_categories))
            .route("/metadata/difficulty/{id}", web::get().to(get_difficulty))
            .route("/metadata/category/{id}", web::get().to(get_category))
    );
}
