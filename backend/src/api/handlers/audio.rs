use actix_web::{web, HttpResponse, Result};
use std::{fs, path::PathBuf};

/// Serves audio files for handpan notes
///
/// This handler serves audio files from the static/audio directory
/// based on the requested note identifier. It supports:
/// - MIDI note numbers (37-88) for tone fields
/// - "gu" for the bottom resonator sound
/// - "slack" for the edge sound (redirects to metronome/clav.flac)
pub async fn serve_audio(path: web::Path<String>) -> Result<HttpResponse> {
    let note = path.into_inner();
    
    // Determine file path based on note type
    let file_path = if note == "gu" {
        PathBuf::from("static/audio/handpan/gu.flac")
    } else if note == "slack" {
        // Use metronome clav sound for slack
        PathBuf::from("static/audio/handpan/127.flac")
    } else {
        // For regular notes, use the MIDI number
        PathBuf::from(format!("static/audio/handpan/{}.flac", note))
    };
    
    // Read the file
    match fs::read(&file_path) {
        Ok(data) => {
            // Determine content type
            let content_type = if file_path.extension().and_then(|ext| ext.to_str()) == Some("flac") {
                "audio/flac"
            } else {
                "application/octet-stream"
            };
            
            // Return the file with appropriate headers
            Ok(HttpResponse::Ok()
                .content_type(content_type)
                .body(data))
        },
        Err(e) => {
            log::error!("Failed to read audio file {:?}: {}", file_path, e);
            Ok(HttpResponse::NotFound().finish())
        }
    }
}
