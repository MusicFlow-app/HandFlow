use crate::db::Database;
use crate::parsers::mscx::{parse_mscx_metadata, parse_mscx_parts, parse_mscx_score};
use crate::parsers::midi::parse_midi;
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use crate::error::AppError;
use futures_util::StreamExt;
use serde_json::json;

pub async fn upload_file(mut payload: Multipart, db: web::Data<Database>) -> Result<HttpResponse, AppError> {
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
        let content = String::from_utf8_lossy(&data);
        
        let file_type = match infer::get(&data) {
            Some(file_type) => file_type.extension(),
            None => {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "error": "Could not determine file type"
                })))
            }
        };

        let (metadata, score_data) = match file_type {
            "mscx" | "mscz" => {
                // Parse metadata
                let (work_title, composer, arranger) = parse_mscx_metadata(&content);
                let metadata = json!({
                    "workTitle": work_title,
                    "composer": composer,
                    "arranger": arranger,
                    "tempo": 120,
                    "keySignature": "Cmaj",
                    "difficulty": 2,  // Default value, can be updated later
                    "category": 2     // Default value, can be updated later
                });
                
                // Parse score data
                let available_parts = match parse_mscx_parts(&content) {
                    Ok(parts) => parts,
                    Err(_) => return Ok(HttpResponse::BadRequest().json(json!({
                        "error": "Failed to parse MSCX parts"
                    })))
                };

                // Filter out parts without staff IDs
                let available_parts: Vec<_> = available_parts.into_iter()
                    .filter(|(staff_id, _)| *staff_id > 0)
                    .collect();

                // Create score_data structure
                let mut score_data = json!({"parts": []});
                for part in available_parts {
                    // Parse score for this part
                    let measures = match parse_mscx_score(&content, part.0) {
                        Ok(result) => result,
                        Err(e) => {
                            eprintln!("Failed to parse score for part {}: {:?}", part.1, e);
                            return Ok(HttpResponse::InternalServerError().json(json!({
                                "error": "Failed to parse score"
                            })));
                        }
                    };

                    // Add part to score_data
                    if let Some(parts_array) = score_data.as_object_mut().and_then(|obj| obj["parts"].as_array_mut()) {
                        parts_array.push(json!({
                            "id": part.0,
                            "name": part.1,
                            "measures": measures.iter().map(|(id, time_sig, chords)| json!({
                                "id": id,
                                "timeSignature": time_sig,
                                "chords": chords
                            })).collect::<Vec<_>>()
                        }));
                    }
                }
                (metadata, score_data)
            },
            "mid" => {
                match parse_midi(&data) {
                    Ok(json_value) => (json!({}), json_value),
                    Err(e) => {
                        eprintln!("Failed to parse MIDI file: {:?}", e);
                        return Ok(HttpResponse::InternalServerError().json(json!({
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

        match db.log_tab(file_size, metadata, score_data).await {
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
