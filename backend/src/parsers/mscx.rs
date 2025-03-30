use crate::error::AppError;
use crate::models::score::{self, Hand, NoteDuration, Note, Part, Measure, NoteType, Metadata, default_cmaj, MidiPitch};
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

pub fn parse_mscx_score(content: &str, part_id: u32, _division: u32) -> Result<Vec<(u32, String, Vec<Vec<(u32, String, bool, u32)>>)>, AppError> {
    
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
    // Initialize hand to Right (false) at the start
    let mut hand = Hand::Right;
    let note_type_regular = NoteType::Normal as u32;
    let note_type_rest = NoteType::Rest as u32;
    
    // Find the correct staff
    
    // Try to find staff as direct child of Score (MuseScore format)
    let staff = score.children()
        .filter(|n| n.has_tag_name("Staff"))
        .find(|n| n.attribute("id")
            .and_then(|id| id.parse::<u32>().ok())
            .map_or(false, |id| id == part_id));
    
    // If not found as direct child, try descendants (standard MusicXML)
    let staff = if let Some(s) = staff {
        s
    } else {
        // Try to find in descendants (nested within Part elements)
        score.descendants()
            .filter(|n| n.has_tag_name("Staff"))
            .find(|n| n.attribute("id")
                .and_then(|id| id.parse::<u32>().ok())
                .map_or(false, |id| id == part_id))
            .ok_or_else(|| AppError::Parse(format!("Staff {} not found", part_id)))?
    };
    
    // Process measures in the staff
    
    for measure in staff.descendants().filter(|n| n.has_tag_name("Measure")) {
        measure_id += 1;
        measure_chords.clear();
        
        // Check if we have a voice element (MuseScore format)
        let voice_elements = measure.children().filter(|n| n.has_tag_name("voice")).collect::<Vec<_>>();
        let has_voice = !voice_elements.is_empty();
        
        // Handle time signature - check if it's in a voice element first (MuseScore format)
        let time_sig = if has_voice {
            voice_elements.iter()
                .filter_map(|v| v.descendants().find(|n| n.has_tag_name("TimeSig")))
                .next()
        } else {
            measure.descendants().find(|n| n.has_tag_name("TimeSig"))
        };
        
        if let Some(time_sig) = time_sig {
            let sig_n = time_sig.descendants()
                .find(|n| n.has_tag_name("sigN"))
                .and_then(|n| n.text())
                .unwrap_or("4");
            let sig_d = time_sig.descendants()
                .find(|n| n.has_tag_name("sigD"))
                .and_then(|n| n.text())
                .unwrap_or("4");
            current_time_signature = format!("{}|{}", sig_n, sig_d);
        }
        
        // Tuplet handling - check if it's in a voice element first (MuseScore format)
        let mut tuplet_normal_notes = 0;
        let mut tuplet_actual_notes = 0;

        let tuplet = if has_voice {
            voice_elements.iter()
                .filter_map(|v| v.descendants().find(|n| n.has_tag_name("Tuplet")))
                .next()
        } else {
            measure.descendants().find(|n| n.has_tag_name("Tuplet"))
        };

        if let Some(tuplet) = tuplet {
            tuplet_normal_notes = tuplet.descendants()
                .find(|n| n.has_tag_name("normalNotes"))
                .and_then(|n| n.text())
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);

            tuplet_actual_notes = tuplet.descendants()
                .find(|n| n.has_tag_name("actualNotes"))
                .and_then(|n| n.text())
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);
                
        }
        
        // Process chords and rests
        
        // If we have voice elements (MuseScore format), look for chords and rests within them
        let elements_to_process = if has_voice {
            // Count chords and rests within voice elements
            let _chord_count: usize = voice_elements.iter()
                .map(|v| v.children().filter(|n| n.has_tag_name("Chord")).count())
                .sum();
            let _rest_count: usize = voice_elements.iter()
                .map(|v| v.children().filter(|n| n.has_tag_name("Rest")).count())
                .sum();
            

            
            // Collect all direct children of all voice elements that are chords or rests
            voice_elements.iter()
                .flat_map(|v| v.children())
                .filter(|n| n.has_tag_name("Chord") || n.has_tag_name("Rest"))
                .collect::<Vec<_>>()
        } else {
            // Standard format - use descendants of measure
            let _chord_count = measure.descendants().filter(|n| n.has_tag_name("Chord")).count();
            let _rest_count = measure.descendants().filter(|n| n.has_tag_name("Rest")).count();

            
            measure.descendants()
                .filter(|n| n.has_tag_name("Chord") || n.has_tag_name("Rest"))
                .collect::<Vec<_>>()
        };
        

        
        for element in elements_to_process {
            match element.tag_name().name() {
                "Chord" => {
                    current_chord_notes.clear();

                    // Determine the hand for the current chord
                    let current_hand = hand;

                    // Get duration
                    if let Some(dur) = element.descendants()
                        .find(|n| n.has_tag_name("durationType"))
                        .and_then(|n| n.text()) {
                        current_duration = dur.to_string();
                    }

                    // Process notes in chord

                    let notes = element.descendants().filter(|n| n.has_tag_name("Note")).collect::<Vec<_>>();

                    
                    for note in notes {
                        // In MuseScore format, pitch is directly in the Note element
                        let pitch = note.descendants()
                            .find(|n| n.has_tag_name("pitch"))
                            .and_then(|n| n.text())
                            .and_then(|n| n.parse::<u32>().ok())
                            .unwrap_or(0);
                        


                        if pitch > 0 {
                            // Calculate duration adjustment for tuplets
                            let duration_multiplier = if tuplet_normal_notes > 0 && tuplet_actual_notes > 0 {
                                tuplet_normal_notes as f32 / tuplet_actual_notes as f32
                            } else {
                                1.0
                            };
                            
                            // Apply tuplet adjustment to duration if needed
                            let adjusted_duration = if duration_multiplier != 1.0 {
                                // Map the base duration to the tuplet-adjusted duration
                                let base_duration = match current_duration.as_str() {
                                    "whole" => "whole",
                                    "half" => "half",
                                    "quarter" => "quarter",
                                    "eighth" => "eighth",
                                    "16th" => "16th",
                                    "32nd" => "32nd",
                                    "64th" => "64th",
                                    _ => current_duration.as_str()
                                };
                                
                                // For triplets (3:2), adjust the duration
                                // e.g., if we have 3 eighth notes in the time of 2 eighth notes,
                                // each note is actually a "triplet eighth" (equivalent to a 12th note)
                                let tuplet_duration = match (tuplet_actual_notes, tuplet_normal_notes, base_duration) {
                                    (3, 2, "eighth") => "triplet-eighth",
                                    (3, 2, "quarter") => "triplet-quarter",
                                    (3, 2, "16th") => "triplet-16th",
                                    (3, 2, "32nd") => "triplet-32nd",
                                    // Add other tuplet patterns as needed
                                    _ => base_duration
                                };
                                

                                
                                tuplet_duration.to_string()
                            } else {
                                current_duration.clone()
                            };
                            
                            current_chord_notes.push((pitch, adjusted_duration.clone(), matches!(current_hand, Hand::Left), note_type_regular));

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
                }
                "Rest" => {
                    current_chord_notes.clear();

                    // Determine the hand for the current rest
                    let current_hand = hand;

                    // Get duration for rest
                    if let Some(dur) = element.descendants()
                        .find(|n| n.has_tag_name("durationType"))
                        .and_then(|n| n.text()) {
                        current_duration = dur.to_string();

                    }

                    // Calculate duration adjustment for tuplets
                    let duration_multiplier = if tuplet_normal_notes > 0 && tuplet_actual_notes > 0 {
                        tuplet_normal_notes as f32 / tuplet_actual_notes as f32
                    } else {
                        1.0
                    };
                    
                    // Apply tuplet adjustment to duration if needed
                    let adjusted_duration = if duration_multiplier != 1.0 {
                        // Map the base duration to the tuplet-adjusted duration
                        let base_duration = match current_duration.as_str() {
                            "whole" => "whole",
                            "half" => "half",
                            "quarter" => "quarter",
                            "eighth" => "eighth",
                            "16th" => "16th",
                            "32nd" => "32nd",
                            "64th" => "64th",
                            _ => current_duration.as_str()
                        };
                        
                        // For triplets (3:2), adjust the duration
                        let tuplet_duration = match (tuplet_actual_notes, tuplet_normal_notes, base_duration) {
                            (3, 2, "eighth") => "triplet-eighth",
                            (3, 2, "quarter") => "triplet-quarter",
                            (3, 2, "16th") => "triplet-16th",
                            (3, 2, "32nd") => "triplet-32nd",
                            // Add other tuplet patterns as needed
                            _ => base_duration
                        };
                        

                        
                        tuplet_duration.to_string()
                    } else {
                        current_duration.clone()
                    };

                    current_chord_notes.push((0, adjusted_duration.clone(), matches!(current_hand, Hand::Left), note_type_rest));
                    measure_chords.push(current_chord_notes.clone());
                    
                    // Toggle hand after each rest
                    hand = match hand {
                        Hand::Right => Hand::Left,
                        Hand::Left => Hand::Right
                    };

                }
                _ => {}
            }
        }
        
        // Add the processed measure to the list
        measures.push((measure_id as u32, current_time_signature.clone(), measure_chords.clone()));
    }
    
    Ok(measures)
}

pub fn parse_mscx(content: &str) -> Result<(JsonValue, JsonValue), AppError> {
    log::info!("Starting new MSCX parsing");
    
    // Parse metadata
    let (work_title, composer, arranger) = parse_mscx_metadata(content);
    log::info!("Metadata parsed: title={}, composer={}, arranger={}", work_title, composer, arranger);

    // Extract division
    let doc = Document::parse(content)
        .map_err(|e| {
            log::error!("Failed to parse MusicXML document: {}", e);
            AppError::Parse(e.to_string())
        })?;

    let score = doc.descendants()
        .find(|n| n.has_tag_name("Score"))
        .ok_or_else(|| AppError::Parse("No Score element found".to_string()))?;

    let division_node = score.descendants()
        .find(|n| n.has_tag_name("Division"))
        .ok_or_else(|| AppError::Parse("No Division element found".to_string()))?;

    let division_text = division_node.text()
        .ok_or_else(|| AppError::Parse("No text in Division element".to_string()))?;

    let division = division_text.parse::<u32>()
        .map_err(|e| AppError::Parse(format!("Invalid division value: {}", e)))?;

    // Extract tempo
    let mut tempo = 120;
    
    // First, try to find Tempo node in any measure
    for measure in score.descendants().filter(|n| n.has_tag_name("Measure")) {
        // Check for tempo in voice elements (MuseScore format)
        let voice_elements = measure.children().filter(|n| n.has_tag_name("voice")).collect::<Vec<_>>();
        let has_voice = !voice_elements.is_empty();
        
        let tempo_node = if has_voice {
            // Look in voice elements first
            voice_elements.iter()
                .filter_map(|v| v.descendants().find(|n| n.has_tag_name("Tempo")))
                .next()
        } else {
            // Fall back to measure descendants
            measure.descendants().find(|n| n.has_tag_name("Tempo"))
        };
        
        if let Some(tempo_node) = tempo_node {

            
            // First, try to get tempo directly from the tempo element
            if let Some(tempo_value_node) = tempo_node.descendants().find(|n| n.has_tag_name("tempo")) {
                if let Some(tempo_value_text) = tempo_value_node.text() {

                    if let Ok(tempo_value) = tempo_value_text.parse::<f64>() {
                        // In MuseScore, the tempo value is a multiplier of 60 BPM
                        // For example, a value of 3 means 180 BPM (3 * 60)
                        let calculated_tempo = (tempo_value * 60.0) as u32;

                        tempo = calculated_tempo;
                        break; // Found tempo, exit loop
                    }
                }
            }
            
            // If direct tempo not found, try to parse from text
            if let Some(text_node) = tempo_node.descendants().find(|n| n.has_tag_name("text")) {
                if let Some(text) = text_node.text() {

                    // Extract BPM from text like "♩ = 180"
                    if let Some(bpm_str) = text.split("=").last() {
                        let cleaned_bpm = bpm_str.trim().chars()
                            .filter(|c| c.is_digit(10))
                            .collect::<String>();
                        
                        if let Ok(bpm) = cleaned_bpm.parse::<u32>() {

                            tempo = bpm;
                            break; // Found tempo, exit loop
                        }
                    }
                }
            }
        }
    }
    


    let metadata = Metadata {
        work_title,
        composer,
        arranger,
        tempo,
        key_signature: default_cmaj(),
        difficulty: 2,  // Default value, can be updated later
        category: 2     // Default value, can be updated later
    };

    // Parse available parts
    let available_parts = parse_mscx_parts(content)?;

    
    // Filter out parts without staff IDs
    let available_parts: Vec<_> = available_parts.into_iter()
        .filter(|(staff_id, _)| *staff_id > 0)
        .collect();

    
    // Create score_data structure
    let mut parts = Vec::new();
    for (part_id, part_name) in available_parts {

        
        // Parse score for this part
        let raw_measures = parse_mscx_score(content, part_id, division)
            .map_err(|_e| AppError::Parse(format!("Failed to parse score for part {}", part_name)))?;
        

        
        // Convert raw measures to Measure objects
        let measures = raw_measures.into_iter().map(|(id, time_sig, chords)| {
            // Convert raw chords to Note objects
            let chord_objects = chords.into_iter().map(|chord_notes| {
                chord_notes.into_iter().map(|(pitch, duration, is_left, note_type)| {
                    Note {
                        pitch: MidiPitch::new(pitch as u8),
                        duration: NoteDuration::from_str(&duration),
                        hand: if is_left { Hand::Left } else { Hand::Right },
                        note_type: if note_type == 0 { NoteType::Rest } else { NoteType::Normal }
                    }
                }).collect()
            }).collect();
            
            Measure {
                id,
                time_signature: Some(time_sig),
                chords: chord_objects
            }
        }).collect::<Vec<Measure>>();
        

        
        // Create Part object
        let part = Part {
            id: part_id,
            name: part_name.clone(),
            measures
        };
        
        log::info!("Part '{}' processed with {} measures", part_name, part.measures.len());
        parts.push(part);
    }
    
    // Create ScoreData object
    // Store the number of parts before moving the vector
    let parts_count = parts.len();
    
    let score_data = score::ScoreData { parts };
    
    // Convert to JSON for compatibility with existing code
    let metadata_json = serde_json::to_value(metadata).unwrap_or(json!({}));
    let score_data_json = serde_json::to_value(score_data).unwrap_or(json!({"parts": []}));
    
    log::info!("MSCX parsing finished with {} parts", parts_count);
    
    Ok((metadata_json, score_data_json))
}
