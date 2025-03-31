// MIDI utility functions for converting between note names and MIDI values

// Map note name to MIDI value (C4 = 60)
pub fn note_to_midi(note: &str) -> Option<i32> {
    let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    
    // Parse note name and octave - ensure we get a valid note name
    // Extract the note name (letters) and octave (numbers) separately
    let note_name: String = note.chars().take_while(|c| !c.is_numeric()).collect();
    let octave: String = note.chars().skip(note_name.len()).collect();
    
    // Validate note name is one of the valid notes
    
    // Trim any whitespace that might be causing issues
    let trimmed_note_name = note_name.trim();
    if !note_names.contains(&trimmed_note_name) {
        return None;
    }
    
    // Find note index using the trimmed note name
    let note_index = match note_names.iter().position(|&n| n == trimmed_note_name) {
        Some(idx) => idx,
        None => return None
    };
    
    // Parse octave
    let octave = match octave.parse::<i32>() {
        Ok(o) => o,
        Err(_) => return None
    };
    
    // Calculate MIDI value
    Some((octave + 1) * 12 + note_index as i32)
}

// Convert MIDI pitch to note name (e.g., 60 -> "C4")
pub fn midi_to_note(midi_value: i32) -> String {
    let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    
    // Calculate octave and note index
    let octave = (midi_value / 12) - 1;
    let note_index = midi_value % 12;
    
    // Combine note name and octave
    format!("{}{}", note_names[note_index as usize], octave)
}