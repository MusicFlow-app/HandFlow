use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::models::handpan::{ALL_DINGS, get_all_scales, get_notes_for_scale_and_ding, ScaleType, NoteHandpan, NotePosition, PanScaleCategory};
use crate::utils::midi::{note_to_midi, midi_to_note};

#[derive(Debug, Serialize, Deserialize)]
struct CategoryResponse {
    id: String,
    name: String,
    description: String,
}

// List all available categories
pub async fn list_categories() -> impl Responder {
    let categories = PanScaleCategory::all()
        .into_iter()
        .map(|cat| {
            CategoryResponse {
                id: format!("{:?}", cat),
                name: cat.to_string(),
                description: cat.to_description(),
            }
        })
        .collect::<Vec<CategoryResponse>>();
    HttpResponse::Ok().json(categories)
}

// List all available dings
pub async fn list_dings() -> impl Responder {
    HttpResponse::Ok().json(ALL_DINGS.to_vec())
}

#[derive(Debug, Serialize)]
struct ScaleResponse {
    name: String,
    id: String,
    category: String,
    max_notes: usize,
}

// List all available scales
pub async fn list_scales() -> impl Responder {
    let scales = get_all_scales();
    
    let response: Vec<ScaleResponse> = scales.into_iter().map(|scale| {
        ScaleResponse {
            name: scale.name,
            id: scale.id,
            category: format!("{:?}", scale.category),
            max_notes: scale.notes.len(),
        }
    }).collect();
    
    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
pub struct HandpanNoteParams {
    pub id: String,  // Renamed from scale_type to match route parameter
    pub ding: String,
}

// Get notes for a specific scale and ding
pub async fn get_notes(info: web::Path<HandpanNoteParams>) -> impl Responder {
    // Parse the id string to ScaleType enum using from_str method
    let scale_type = match ScaleType::from_str(&info.id) {
        Ok(scale) => scale,
        Err(_) => return HttpResponse::NotFound().json("Invalid scale type")
    };
    
    match get_notes_for_scale_and_ding(&scale_type, &info.ding) {
        Some(notes) => HttpResponse::Ok().json(notes),
        None => HttpResponse::NotFound().json("Scale or ding not found")
    }
}

#[derive(Deserialize)]
pub struct HandpannerNotationRequest {
    pub notation: String,
    pub ding_wanted: Option<String>,
    pub name: Option<String>,
}

/// Import notes from handpanner notation format (e.g. "A/C E G A B C E G")
pub async fn import_from_handpanner_notation(request: web::Json<HandpannerNotationRequest>) -> impl Responder {
    let handpan_definition = &request.notation;
    let ding_wanted = request.ding_wanted.as_deref().unwrap_or("");
    let name = request.name.as_deref().unwrap_or("");
    
    // Split by the slash to separate ding from other notes
    let parts: Vec<&str> = handpan_definition.split('/').collect();
    if parts.len() != 2 {
        return HttpResponse::BadRequest().json("Invalid notation format. Expected format: 'Ding/Note1 Note2 Note3...'")
    }
    
    // Parse the definition ding and other notes
    let definition_ding = parts[0].trim();
    let definition_rest_notes = parts[1].trim();
    
    // Parse the ding information (using default octave 3)
    let definition_ding_obj = split_note_name_and_octave(definition_ding, Some(3));
    if definition_ding_obj.0.is_empty() {
        return HttpResponse::BadRequest().json(format!("Invalid ding note format: {}", definition_ding))
    }
    
    // Check if the definition ding is in the list of valid ding notes
    let formatted_def_ding = format!("{}{}", definition_ding_obj.0, definition_ding_obj.1.unwrap_or(3));
    if !ALL_DINGS.contains(&formatted_def_ding.as_str()) {
        return HttpResponse::BadRequest().json(format!("Definition ding note '{}' is not valid. Valid ding notes are: {:?}", formatted_def_ding, ALL_DINGS))
    }
    
    // If ding_wanted is specified, use it; otherwise use the definition ding
    let ding_wanted_obj = if !ding_wanted.is_empty() {
        split_note_name_and_octave(ding_wanted, Some(3))
    } else {
        definition_ding_obj.clone()
    };
    
    if ding_wanted_obj.0.is_empty() {
        return HttpResponse::BadRequest().json(format!("Invalid target ding: {}", ding_wanted))
    }
    
    // Calculate transposition semitones
    let transpose_by = semitones_difference(
        &definition_ding_obj.0, definition_ding_obj.1.unwrap_or(3),
        &ding_wanted_obj.0, ding_wanted_obj.1.unwrap_or(3)
    );
    
    // Create the handpan ding note
    let ding_ref_midi = match note_to_midi(&format!("{}{}", definition_ding_obj.0, definition_ding_obj.1.unwrap_or(3))) {
        Some(midi) => midi,
        None => return HttpResponse::BadRequest().json(format!("Invalid ding note: {}", definition_ding))
    };

    // Create the handpan ding note
    let ding_midi = match note_to_midi(&format!("{}{}", ding_wanted_obj.0, ding_wanted_obj.1.unwrap_or(3))) {
        Some(midi) => midi,
        None => return HttpResponse::BadRequest().json(format!("Invalid ding note: {}", ding_wanted))
    };
    
    let mut handpan_notes = Vec::new();

    // Create NoteHandpan object for ding
    handpan_notes.push(NoteHandpan {
        note_index: (0) as u32, // Note index starts at 1 (after ding at 0)
        position: NotePosition::Top,
        distance_relative_to_ding: 0,
        calculated_pitch: Some(ding_midi),
        calculated_note: Some(midi_to_note(ding_midi)),
    });

    // Initialize tracking variables for octave progression
    let mut previous_note_octave = ding_wanted_obj.1.unwrap_or(3);
    let mut previous_note_index = get_note_index(&ding_wanted_obj.0);
    
    // Parse the rest notes
    let note_tokens: Vec<&str> = definition_rest_notes.split_whitespace().collect();
    
    // Create NoteHandpan objects for each note
    for (i, note_octave_paren) in note_tokens.iter().enumerate() {
        // Determine if it's a bottom note (in parentheses) or inner note (in brackets)
        let is_bottom = note_octave_paren.starts_with('(') && note_octave_paren.ends_with(')');
        let is_inner = note_octave_paren.starts_with('[') && note_octave_paren.ends_with(']');
        
        // Remove parentheses or brackets
        let note_octave = note_octave_paren
            .replace("(", "")
            .replace(")", "")
            .replace("[", "")
            .replace("]", "");
        
        // Split note name and octave
        let note_obj = split_note_name_and_octave(&note_octave, None);
        if note_obj.0.is_empty() {
            return HttpResponse::BadRequest().json(format!("Invalid note: {}", note_octave_paren))
        }
        
        // Transpose the note
        let transposed_note = transpose_note(&note_obj.0, note_obj.1, transpose_by);
        
        // Get the note index in the chromatic scale
        let note_index = get_note_index(&transposed_note.0);
        
        // Determine octave based on the TypeScript logic
        let octave = if let Some(explicit_octave) = transposed_note.1 {
            // If octave is explicitly specified after transposition, use it
            explicit_octave
        } else {
            // Following the TypeScript logic for implied octave
            // If current note index <= previous note index, increment the octave
            if note_index <= previous_note_index {
                previous_note_octave + 1
            } else {
                previous_note_octave
            }
        };
        
        // Convert to MIDI
        // This is fucked up i need to investigate
        match note_to_midi(&format!("{}{}", transposed_note.0, octave)) {
            Some(note_midi) => {
                // Calculate distance from ding
                let distance = note_midi - ding_ref_midi;
                let calculated_pitch = ding_midi + distance;
                
                // Create NoteHandpan object
                handpan_notes.push(NoteHandpan {
                    note_index: (i + 1) as u32, // Note index starts at 1 (after ding at 0)
                    position: if is_bottom {
                        NotePosition::Bottom
                    } else if is_inner {
                        NotePosition::Inner
                    } else {
                        NotePosition::Top
                    },
                    distance_relative_to_ding: distance,
                    calculated_pitch: Some(calculated_pitch),
                    calculated_note: Some(midi_to_note(calculated_pitch)),
                });
                
                // Update tracking variables for next note
                previous_note_octave = octave;
                previous_note_index = note_index;
            },
            None => return HttpResponse::BadRequest().json(format!("Invalid note: {}", note_octave_paren))
        }
    }
    
    // Create a response with additional metadata similar to the TypeScript version
    #[derive(Serialize)]
    struct HandpanResponse {
        notes: Vec<NoteHandpan>,
        name: String,
        generic_name: String,
    }
    
    // If name is provided, generate a handpan name in the same format as TypeScript
    let (full_name, generic_name) = if !name.is_empty() {
        // Count top, bottom, and inner notes
        let nb_top = handpan_notes.iter().filter(|n| matches!(n.position, NotePosition::Top)).count();
        let nb_bot = handpan_notes.iter().filter(|n| matches!(n.position, NotePosition::Bottom)).count();
        let nb_inner = handpan_notes.iter().filter(|n| matches!(n.position, NotePosition::Inner)).count();
        
        // Format the recap part (e.g., "8+2+0+1")
        let nb_recap = match (nb_top, nb_bot, nb_inner) {
            (t, b, i) if b > 0 && i > 0 => format!("{t}+{b}+{i}"),
            (t, b, 0) if b > 0 => format!("{t}+{b}"),
            (t, 0, i) if i > 0 => format!("{t}+0+{i}"),
            (t, 0, 0) => format!("{t}"),
            _ => format!("{nb_top}") // Fallback
        };
        
        // Create the generic name
        let generic = format!("{name} {nb_recap}");
        
        // Create the full handpan name
        let octave_suffix = if ding_wanted_obj.1.unwrap_or(3) != 3 {
            ding_wanted_obj.1.unwrap_or(3).to_string()
        } else {
            "".to_string()
        };
        
        let full = format!("{}{} {}", ding_wanted_obj.0, octave_suffix, generic);
        (full, generic)
    } else {
        // Count top, bottom, and inner notes
        let nb_top = handpan_notes.iter().filter(|n| matches!(n.position, NotePosition::Top)).count();
        let nb_bot = handpan_notes.iter().filter(|n| matches!(n.position, NotePosition::Bottom)).count();
        let nb_inner = handpan_notes.iter().filter(|n| matches!(n.position, NotePosition::Inner)).count();
        
        // Format the recap part (e.g., "8+2+0+1")
        let nb_recap = match (nb_top, nb_bot, nb_inner) {
            (t, b, i) if b > 0 && i > 0 => format!("{t}+{b}+{i}+1"),
            (t, b, 0) if b > 0 => format!("{t}+{b}+1"),
            (t, 0, i) if i > 0 => format!("{t}+0+{i}+1"),
            (t, 0, 0) => format!("{t}+1"),
            _ => format!("{nb_top}+1") // Fallback
        };
        
        // Create the generic name
        let generic = format!("import Handpan {nb_recap}");
        let full = format!("{} {}", ding_wanted_obj.0, generic);
        (full, generic)
    };
    
    // Return the handpan response with metadata
    HttpResponse::Ok().json(HandpanResponse {
        notes: handpan_notes,
        name: full_name,
        generic_name,
    })
}

/// Helper function to split a note string into name and octave
fn split_note_name_and_octave(note_str: &str, default_octave: Option<i32>) -> (String, Option<i32>) {
    // Extract valid note name (A-G with possible # or b)
    let note_name: String = note_str.chars()
        .take_while(|c| !c.is_numeric())
        .collect();
    let octave_str: String = note_str.chars().filter(|c| c.is_numeric()).collect();
    
    let octave = if !octave_str.is_empty() {
        octave_str.parse::<i32>().ok()
    } else {
        default_octave
    };
    
    (note_name, octave)
}

/// Calculate semitone difference between two notes (similar to TypeScript semitonesDifference)
fn semitones_difference(source_note: &str, source_octave: i32, target_note: &str, target_octave: i32) -> i32 {
    let notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    
    let source_index = notes.iter().position(|&n| n == source_note);
    let target_index = notes.iter().position(|&n| n == target_note);
    
    if source_index.is_none() || target_index.is_none() {
        return 0; // Return 0 if either note is invalid
    }
    
    let source_midi = (source_octave + 1) * 12 + source_index.unwrap() as i32;
    let target_midi = (target_octave + 1) * 12 + target_index.unwrap() as i32;
    
    target_midi - source_midi
}

/// Transpose a note by a number of semitones (similar to TypeScript transposeNoteObj)
fn transpose_note(note_name: &str, octave_opt: Option<i32>, semitones: i32) -> (String, Option<i32>) {
    let notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    
    // If note is not in the chromatic scale or octave is not specified, return as is
    if !notes.contains(&note_name) || octave_opt.is_none() {
        return (note_name.to_string(), octave_opt);
    }
    
    let octave = octave_opt.unwrap();
    
    // Calculate MIDI value and transpose
    let note_index = notes.iter().position(|&n| n == note_name).unwrap() as i32;
    let midi_value = (octave + 1) * 12 + note_index + semitones;
    
    // Calculate new octave and note index
    let new_octave = (midi_value / 12) - 1;
    let new_note_index = (midi_value % 12) as usize;
    
    (notes[new_note_index].to_string(), Some(new_octave))
}

/// Helper function to get the index of a note in the chromatic scale
fn get_note_index(note_name: &str) -> i32 {
    let notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    
    match notes.iter().position(|&n| n == note_name) {
        Some(index) => index as i32,
        None => 0, // Default to 0 if not found
    }
}
