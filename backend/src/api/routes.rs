use actix_web::web;
use super::handlers::{get_library, get_tab_details, toggle_favorite, get_part_measures, upload_file, generate_tab_html};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/upload", web::post().to(upload_file))
            .route("/library", web::get().to(get_library))
            .route("/tabs/{id}/favorite", web::post().to(toggle_favorite))
            .route("/tabs/{id}", web::get().to(get_tab_details))
            .route("/tabs/{id}/parts/{part_id}/measures", web::get().to(get_part_measures))
            .route("/tabs/{id}/parts/{part_id}/generate", web::get().to(generate_tab_html))
    );
}
