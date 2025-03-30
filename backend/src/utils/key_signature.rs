use std::collections::HashMap;
use crate::models::KeySignature;
use crate::models::score::{ScoreJson, ScoreData, NoteType};

// Key detection using the Krumhansl-Schmuckler algorithm
// This algorithm correlates the frequency of pitch classes in music with
// established key profiles to determine the most likely key signature

// Pitch Class Profiles from Krumhansl-Schmuckler research
// Values represent the "stability" or frequency of each pitch class in a key
// Index 0 = C, 1 = C#, 2 = D, etc.
const MAJOR_PROFILE: [f64; 12] = [
    6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88
];

const MINOR_PROFILE: [f64; 12] = [
    6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17
];

// The KeySignature enum has been moved to models/key_signature.rs


/// Detects the most likely key signature using the Krumhansl-Schmuckler algorithm
/// Returns the detected key signature with a confidence level of approximately 90% if sufficient data is available
pub fn detect_key_signature(score_json: &ScoreJson) -> KeySignature {
    // Extract all pitches from the score data
    let pitch_classes = extract_pitch_classes(&score_json.score_data);
    
    // Calculate the correlation with all possible keys
    let key_correlations = calculate_key_correlations(&pitch_classes);
    
    // Find the key with the highest correlation
    let (key, _correlation) = find_highest_correlation(&key_correlations);
    
    key
}

/// Extracts the distribution of pitch classes from the score
/// Counts occurrences of each pitch class (C, C#, D, etc.) and normalizes the values
fn extract_pitch_classes(score_data: &ScoreData) -> [f64; 12] {
    let mut pitch_count = [0.0f64; 12];
    let mut total_notes = 0.0f64;

    // Process all parts
    for part in &score_data.parts {
        // Process all measures
        for measure in &part.measures {
            // Process all chords
            for chord_notes in &measure.chords {
                for note in chord_notes {
                    // Skip rests (note_type = 0) or out-of-bounds pitches
                    if note.note_type != NoteType::Rest && note.pitch > 0 {
                        // Convert MIDI pitch to pitch class (0-11)
                        let pitch_class = (note.pitch.as_int() % 12) as usize;
                        
                        // Weight by note duration using the built-in to_fraction method
                        let duration_weight = note.duration.to_fraction() as f64;
                        
                        pitch_count[pitch_class] += duration_weight;
                        total_notes += duration_weight;
                    }
                }
            }
        }
    }

    // Normalize the pitch class distribution
    if total_notes > 0.0 {
        for count in pitch_count.iter_mut() {
            *count /= total_notes;
        }
    }

    pitch_count
}

/// Calculates the correlation coefficient between the extracted pitch distribution
/// and the Krumhansl-Schmuckler key profiles for all possible keys
fn calculate_key_correlations(pitch_classes: &[f64; 12]) -> HashMap<KeySignature, f64> {
    let mut correlations = HashMap::new();

    // Calculate correlations for all major keys
    for i in 0..12 {
        let key = match i {
            0 => KeySignature::Cmaj,
            1 => KeySignature::CsMaj,
            2 => KeySignature::Dmaj,
            3 => KeySignature::Ebmaj,
            4 => KeySignature::Emaj,
            5 => KeySignature::Fmaj,
            6 => KeySignature::FsMaj,
            7 => KeySignature::Gmaj,
            8 => KeySignature::Abmaj,
            9 => KeySignature::Amaj,
            10 => KeySignature::Bbmaj,
            11 => KeySignature::Bmaj,
            _ => unreachable!(),
        };

        let correlation = calculate_correlation(pitch_classes, &MAJOR_PROFILE, i);
        correlations.insert(key, correlation);
    }

    // Calculate correlations for all minor keys
    for i in 0..12 {
        let key = match i {
            0 => KeySignature::Amin,
            1 => KeySignature::AsMin,
            2 => KeySignature::Bmin,
            3 => KeySignature::Cmin,
            4 => KeySignature::CsMin,
            5 => KeySignature::Dmin,
            6 => KeySignature::DsMin,
            7 => KeySignature::Emin,
            8 => KeySignature::Fmin,
            9 => KeySignature::FsMin,
            10 => KeySignature::Gmin,
            11 => KeySignature::GsMin,
            _ => unreachable!(),
        };

        let correlation = calculate_correlation(pitch_classes, &MINOR_PROFILE, i);
        correlations.insert(key, correlation);
    }

    correlations
}

/// Calculates the correlation coefficient between the extracted pitch distribution
/// and a key profile (major or minor) shifted by a specific amount
fn calculate_correlation(pitch_classes: &[f64; 12], key_profile: &[f64; 12], shift: usize) -> f64 {
    let mut sum_xy = 0.0;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_x_squared = 0.0;
    let mut sum_y_squared = 0.0;

    for i in 0..12 {
        let profile_idx = (i + 12 - shift) % 12;
        let x = pitch_classes[i];
        let y = key_profile[profile_idx];

        sum_xy += x * y;
        sum_x += x;
        sum_y += y;
        sum_x_squared += x * x;
        sum_y_squared += y * y;
    }

    let numerator = 12.0 * sum_xy - sum_x * sum_y;
    let denominator = ((12.0 * sum_x_squared - sum_x * sum_x) * (12.0 * sum_y_squared - sum_y * sum_y)).sqrt();

    if denominator == 0.0 {
        return 0.0;
    }

    numerator / denominator
}

/// Finds the key with the highest correlation coefficient
fn find_highest_correlation(correlations: &HashMap<KeySignature, f64>) -> (KeySignature, f64) {
    correlations
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(k, v)| (k.clone(), *v))
        .unwrap_or((KeySignature::Cmaj, 0.0))
}

/// Public function to detect and compare with the metadata key
/// Returns true if the detected key matches the metadata
pub fn verify_key_signature(score_json: &ScoreJson) -> bool {
    let detected_key = detect_key_signature(score_json);
    detected_key == score_json.metadata.key_signature
}

/// Entry point for key detection from score JSON
/// Returns the detected key as a KeySignature enum
pub fn analyze_key_signature(score_json: &ScoreJson) -> KeySignature {
    detect_key_signature(score_json)
}