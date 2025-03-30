use crate::error::AppError;
use crate::models::score::Hand;
use roxmltree::Document;
use serde_json::{json, Value as JsonValue};

pub fn parse_mscx_metadata(content: &str) -> (String, String, String) {
    let mut composer = String::from("Unknown");
    let mut arranger = String::from("Unknown");
    let mut work_title = String::from("Unknown");

    if let Ok(doc) = Document::parse(content) {
        if let Some(score) = doc.descendants().find(|n| n.has_tag_name("Score")) {
            for meta_tag in score.descendants().filter(|n| n.has_tag_name("metaTag")) {
                if let Some(name_attr) = meta_tag.attribute("name") {
                    let value = meta_tag.text()
                        .map(|t| t.to_string())
                        .unwrap_or_else(|| String::from("Unknown"));

                    match name_attr {
                        "composer" => composer = value,
                        "arranger" => arranger = value,
                        "workTitle" => work_title = value,
                        _ => {}
                    }
                }
            }
        }
    } else {
        log::error!("Error while parsing XML content");
    }

    (work_title, composer, arranger)
}

pub fn parse_mscx_parts(content: &str) -> Result<Vec<(u32, String)>, AppError> {
    let doc = Document::parse(content)
        .map_err(|e| AppError::Parse(e.to_string()))?;
    
    let score = doc.descendants()
        .find(|n| n.has_tag_name("Score"))
        .ok_or_else(|| AppError::Parse("No Score element found".to_string()))?;
    
    let mut parts = Vec::new();
    let mut current_staff_ids = Vec::new();
    
    // Process each Part element
    for part in score.descendants().filter(|n| n.has_tag_name("Part")) {
        // Get the part name
        let part_name = part.descendants()
            .find(|n| n.has_tag_name("trackName"))
            .and_then(|n| n.text())
            .unwrap_or("Untitled")
            .to_string();
        
        current_staff_ids.clear();
        
        // Collect all Staff IDs for this part
        for staff in part.descendants().filter(|n| n.has_tag_name("Staff")) {
            if let Some(id_str) = staff.attribute("id") {
                if let Ok(id) = id_str.parse::<u32>() {
                    current_staff_ids.push(id);
                } else {
                    return Err(AppError::Parse(format!("Invalid staff ID: {}", id_str)));
                }
            }
        }
        
        // Add parts with appropriate names based on staff count
        if current_staff_ids.len() == 2 {
            // If two staffs, add Treble and Bass versions
            parts.push((current_staff_ids[0], format!("{} (Treble)", part_name)));
            parts.push((current_staff_ids[1], format!("{} (Bass)", part_name)));
        } else if !current_staff_ids.is_empty() {
            // Otherwise, add one entry per staff
            for &staff_id in &current_staff_ids {
                parts.push((staff_id, part_name.clone()));
            }
        }
    }
    
    Ok(parts)
}

pub fn parse_mscx_score(content: &str, part_id: u32) -> Result<Vec<(u32, String, Vec<Vec<(u32, String, bool, u32)>>)>, AppError> {
    let doc = Document::parse(content)
        .map_err(|e| AppError::Parse(e.to_string()))?;
    
    let score = doc.descendants()
        .find(|n| n.has_tag_name("Score"))
        .ok_or_else(|| AppError::Parse("No Score element found".to_string()))?;
    
    let mut measures = Vec::new();
    let mut measure_id = 0;
    let mut current_duration = String::from("quarter");
    let mut current_time_signature = String::new();
    let mut current_chord_notes = Vec::new();
    let mut measure_chords = Vec::new();
    let mut hand = Hand::Right;
    let note_type = 1;
    let note_type_rest = 0;
    
    // Find the correct staff
    let staff = score.descendants()
        .filter(|n| n.has_tag_name("Staff"))
        .find(|n| n.attribute("id")
            .and_then(|id| id.parse::<u32>().ok())
            .map_or(false, |id| id == part_id))
        .ok_or_else(|| AppError::Parse(format!("Staff {} not found", part_id)))?;
    
    // Process measures in the staff
    for measure in staff.descendants().filter(|n| n.has_tag_name("Measure")) {
        measure_id += 1;
        measure_chords.clear();
        
        // Handle time signature
        if let Some(time_sig) = measure.descendants().find(|n| n.has_tag_name("TimeSig")) {
            let sig_n = time_sig.descendants()
                .find(|n| n.has_tag_name("sigN"))
                .and_then(|n| n.text())
                .unwrap_or("4");
            let sig_d = time_sig.descendants()
                .find(|n| n.has_tag_name("sigD"))
                .and_then(|n| n.text())
                .unwrap_or("4");
            current_time_signature = format!("{sig_n}|{sig_d}");
        }
        
        // Process chords and rests
        for element in measure.descendants() {
            match element.tag_name().name() {
                "Chord" => {
                    current_chord_notes.clear();
                    
                    // Get duration
                    if let Some(dur) = element.descendants()
                        .find(|n| n.has_tag_name("durationType"))
                        .and_then(|n| n.text()) {
                        current_duration = dur.to_string();
                    }
                    
                    // Process notes in chord
                    for note in element.descendants().filter(|n| n.has_tag_name("Note")) {
                        let pitch = note.descendants()
                            .find(|n| n.has_tag_name("pitch"))
                            .and_then(|n| n.text())
                            .and_then(|n| n.parse::<u32>().ok())
                            .unwrap_or(0);
                        
                        if pitch > 0 {
                            current_chord_notes.push((pitch, current_duration.clone(), matches!(hand, Hand::Left), note_type));
                        }
                    }
                    
                    if !current_chord_notes.is_empty() {
                        measure_chords.push(current_chord_notes.clone());
                        // Toggle hand after each chord
                        hand = match hand {
                            Hand::Right => Hand::Left,
                            Hand::Left => Hand::Right
                        };
                    }
                },
                "Rest" => {
                    current_chord_notes.clear();
                    
                    // Get duration for rest
                    if let Some(dur) = element.descendants()
                        .find(|n| n.has_tag_name("durationType"))
                        .and_then(|n| n.text()) {
                        current_duration = dur.to_string();
                    }
                    
                    current_chord_notes.push((0, current_duration.clone(), matches!(hand, Hand::Left), note_type_rest));
                    measure_chords.push(current_chord_notes.clone());
                    // Toggle hand after each rest
                    hand = match hand {
                        Hand::Right => Hand::Left,
                        Hand::Left => Hand::Right
                    };
                },
                _ => {}
            }
        }
        
        measures.push((measure_id as u32, current_time_signature.clone(), measure_chords.clone()));
    }
    
    Ok(measures)
}

pub fn parse_mscx(content: &str) -> Result<(JsonValue, JsonValue), AppError> {
    // Parse metadata
    let (work_title, composer, arranger) = parse_mscx_metadata(content);
    let metadata = json!({
        "workTitle": work_title,
        "composer": composer,
        "arranger": arranger,
        "tempo": 120,
        "keySignature": "Cmaj",
        "difficulty": 2,  // Default value, can be updated later
        "category": 2     // Default value, can be updated later
    });
    
    // Parse available parts
    let available_parts = parse_mscx_parts(content)?;
    
    // Filter out parts without staff IDs
    let available_parts: Vec<_> = available_parts.into_iter()
        .filter(|(staff_id, _)| *staff_id > 0)
        .collect();
    
    // Create score_data structure
    let mut score_data = json!({"parts": []});
    for part in available_parts {
        // Parse score for this part
        let measures = parse_mscx_score(content, part.0)
            .map_err(|e| {
                log::error!("Failed to parse score for part {}: {:?}", part.1, e);
                AppError::Parse(format!("Failed to parse score for part {}", part.1))
            })?;
        
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
    
    Ok((metadata, score_data))
}
