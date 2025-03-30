use crate::db::Database;
use crate::parsers::mscx::parse_mscx;
use crate::parsers::midi::parse_midi;
use crate::utils::key_signature::{analyze_key_signature, verify_key_signature};
use crate::utils::difficulty_analysis::analyze_difficulty;
use crate::models::score::ScoreJson;
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use crate::error::AppError;
use futures_util::StreamExt;
use serde_json::json;
use std::io::{Cursor, Read};
use zip::ZipArchive;

pub async fn upload_file(mut payload: Multipart, _db: web::Data<Database>) -> Result<HttpResponse, AppError> {
    let mut file_data = None;
    let mut file_size = 0;

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| AppError::Upload(e.to_string()))?;
        if field.name() == "file" {
            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                let bytes = chunk.map_err(|e| AppError::Upload(e.to_string()))?;
                file_size += bytes.len() as i64;
                data.extend_from_slice(&bytes);
            }
            file_data = Some(data);
        }
    }

    if let Some(data) = file_data {
        // Handle potential MSCZ (zipped) files
        let content = if let Some(ft) = infer::get(&data) {
            if ft.extension() == "zip" {
                // Try to unzip and find .mscx file
                let reader = Cursor::new(&data);
                match ZipArchive::new(reader) {
                    Ok(mut archive) => {
                        let mut mscx_content = None;
                        for i in 0..archive.len() {
                            if let Ok(mut file) = archive.by_index(i) {
                                if file.name().ends_with(".mscx") {
                                    let mut buffer = Vec::new();
                                    if file.read_to_end(&mut buffer).is_ok() {
                                        if let Ok(contents) = String::from_utf8(buffer) {
                                            mscx_content = Some(contents);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        mscx_content.unwrap_or_else(|| String::from_utf8_lossy(&data).into_owned())
                    },
                    Err(_) => String::from_utf8_lossy(&data).into_owned()
                }
            } else {
                String::from_utf8_lossy(&data).into_owned()
            }
        } else {
            String::from_utf8_lossy(&data).into_owned()
        };
        
        // Detect file type using magic bytes
        let file_type = if let Some(ft) = infer::get(&data) {
            ft.extension()
        } else {
            // Fallback to file extension
            if content.contains("<museScore") {
                "mscx"
            } else if data.starts_with(b"MThd") {
                "mid"
            } else {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "error": "Could not determine file type"
                })))
            }
        };
        // Normalize file extensions
        let normalized_type = if file_type == "midi" {
            "mid"
        } else if file_type == "zip" && content.contains("<museScore") {
            "mscz"
        } else {
            file_type
        };

        let (metadata, score_data) = match normalized_type {
            "mscx" | "mscz" => {
                match parse_mscx(&content) {
                    Ok((metadata, score_data)) => (metadata, score_data),
                    Err(e) => {
                        log::error!("Failed to parse MSCX file: {:?}", e);
                        return Ok(HttpResponse::BadRequest().json(json!({
                            "error": "Failed to parse MSCX file"
                        })));
                    }
                }
            },
            "mid" => {
                match parse_midi(&data) {
                    Ok((metadata, score_data)) => (metadata, score_data),
                    Err(e) => {
                        log::error!("Failed to parse MIDI file: {:?}", e);
                        return Ok(HttpResponse::BadRequest().json(json!({
                            "error": "Failed to parse MIDI file"
                        })));
                    }
                }
            }
            _ => {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "error": "Unsupported file type"
                })))
            }
        };

        // Create ScoreJson for key detection
        let mut score_json = ScoreJson {
            file_size: file_size as u32,
            metadata: serde_json::from_value(metadata).map_err(|e| AppError::Parse(e.to_string()))?,
            score_data: serde_json::from_value(score_data).map_err(|e| AppError::Parse(e.to_string()))?,
        };
        
        // Analyze and update the key signature
        let detected_key = analyze_key_signature(&score_json);
        let original_key = score_json.metadata.key_signature.clone();
        score_json.metadata.key_signature = detected_key;
        
        // Verify if the detected key matches the original key
        let key_verified = verify_key_signature(&score_json);
        if !key_verified {
            println!("Warning: Detected key '{}' differs from original key '{}'", 
                detected_key.to_string(), original_key.to_string());
        }
        
        // Analyze and update the difficulty level
        analyze_difficulty(&mut score_json);
        
        // DEBUG: Database insertion disabled for debugging
        // if let Ok(_) = _db.insert_tab(score_json.clone()).await {
        //     Ok(HttpResponse::Ok().json(score_json))
        // } else {
        //     Ok(HttpResponse::InternalServerError().json(json!({
        //         "error": "Failed to save to database"
        //     })))
        // }
        
        // Return parsed data directly for debugging
        Ok(HttpResponse::Ok().json(score_json))
    } else {
        Ok(HttpResponse::BadRequest().json(json!({
            "error": "No file provided"
        })))
    }
}
