use crate::models::{Difficulty, ScoreJson};

/// Calculate the total number of notes in the score
fn count_total_notes(score: &ScoreJson) -> usize {
    let mut total_notes = 0;
    
    for part in &score.score_data.parts {
        for measure in &part.measures {
            for chord_notes in &measure.chords {
                // Each chord_notes is a Vec<Note>, so we count the length directly
                total_notes += chord_notes.len();
            }
        }
    }
    
    total_notes
}

/// Calculate the average note density (notes per measure)
fn calculate_note_density(score: &ScoreJson) -> f32 {
    let total_notes = count_total_notes(score) as f32;
    let mut total_measures = 0;
    
    for part in &score.score_data.parts {
        total_measures += part.measures.len();
    }
    
    if total_measures == 0 {
        return 0.0;
    }
    
    total_notes / total_measures as f32
}

/// Estimate difficulty based on key signature, tempo, and note density
pub fn estimate_difficulty(score: &ScoreJson) -> Difficulty {
    let key_score = score.metadata.key_signature.complexity_score();
    let tempo = score.metadata.tempo;
    let note_density = calculate_note_density(score);
    
    let mut difficulty_score = 0;
    
    // Key complexity (0-2 points)
    if key_score >= 6 {
        difficulty_score += 2;
    } else if key_score >= 3 {
        difficulty_score += 1;
    }
    
    // Tempo (0-2 points)
    if tempo > 160 {
        difficulty_score += 2;
    } else if tempo > 120 {
        difficulty_score += 1;
    }
    
    // Note density (0-3 points)
    if note_density > 8.0 {
        difficulty_score += 3;
    } else if note_density > 5.0 {
        difficulty_score += 2;
    } else if note_density > 3.0 {
        difficulty_score += 1;
    }
    
    // Final classification based on difficulty score and available difficulty levels
    // Get all available difficulty levels
    let difficulties = Difficulty::all();
    let difficulty_count = difficulties.len();
    
    // Calculate the maximum possible score (7 in this implementation)
    let max_score = 7;
    
    // Dynamically determine the difficulty based on the score and available levels
    // This automatically adapts if more difficulty levels are added in the future
    let index = if difficulty_count > 1 {
        let segment_size = (max_score as f32 / (difficulty_count - 1) as f32).ceil() as u8;
        let index = (difficulty_score / segment_size).min((difficulty_count - 1) as u8) as usize;
        index
    } else {
        0 // If there's only one difficulty level, use it
    };
    
    difficulties[index]
}

/// Analyze the score and update its difficulty level
pub fn analyze_difficulty(score: &mut ScoreJson) {
    let difficulty = estimate_difficulty(score);
    score.metadata.difficulty = difficulty;
    
    println!("Difficulty analysis: {} (score factors: key={}, tempo={}, density={})",
        difficulty.to_string(),
        score.metadata.key_signature.complexity_score(),
        score.metadata.tempo,
        calculate_note_density(score)
    );
}
