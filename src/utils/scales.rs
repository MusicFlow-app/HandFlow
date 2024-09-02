/**
 * Generates a list of handpan scales with varying note counts.
 *
 * This function:
 *
 * 1. **Defines Full Scales**: A set of predefined handpan scales with their respective MIDI notes and TPC (Tonnetz Pitch Class) values.
 * 2. **Generates Variants**: For each scale, generates variants with note counts ranging from 9 to 13 notes by clipping the full scale.
 * 3. **Assigns IDs**: Each scale variant is assigned a unique ID.
 * 4. **Returns**: A vector of tuples containing the ID, scale name, MIDI notes, and TPC values.
 *
 * @return A vector of tuples where each tuple contains:
 *  - A unique ID (usize)
 *  - The scale name (&'static str)
 *  - A vector of MIDI notes (Vec<u8>)
 *  - A vector of TPC values (Vec<i8>)
 */
pub fn scales_list() -> Vec<(usize, &'static str, Vec<u8>, Vec<i8>)> {
    let full_scales = vec![
        (
            "D Kurd",
            vec![50, 57, 58, 60, 62, 64, 65, 67, 69, 70, 72, 74, 77],
            vec![16, 17, 12, 14, 16, 18, 13, 15, 17, 12, 14, 16, 13],
        ),
        (
            "Celtic",
            vec![50, 57, 60, 62, 64, 65, 67, 69, 72, 74, 77, 79, 81],
            vec![16, 10, 14, 16, 18, 19, 21, 23, 26, 28, 31, 33, 35],
        ),
        (
            "Integral",
            vec![45, 48, 50, 52, 55, 57, 60, 62, 64, 67, 69, 72, 74],
            vec![10, 13, 16, 18, 21, 23, 26, 28, 30, 33, 35, 38, 40],
        ),
        (
            "Equinox",
            vec![50, 57, 60, 62, 65, 67, 69, 72, 74, 77, 79, 81],
            vec![16, 10, 14, 16, 19, 21, 23, 26, 28, 31, 33, 35],
        ), // 12-note base
        (
            "Pygmy",
            vec![43, 46, 48, 51, 55, 58, 60, 63, 67],
            vec![10, 13, 14, 16, 19, 20, 22, 23, 26],
        ), // Typically 9-note base
        (
            "Hijaz",
            vec![50, 51, 55, 57, 61, 62, 65, 66, 69],
            vec![16, 17, 21, 23, 25, 26, 28, 29, 32],
        ), // Typically 9-note base
        (
            "C# Annaziska",
            vec![49, 56, 58, 60, 62, 63, 66, 68, 71],
            vec![15, 22, 24, 26, 28, 29, 32, 34, 37],
        ), // Typically 9-note base
        (
            "Melog Selisir",
            vec![50, 53, 55, 58, 60, 62, 65, 67, 70, 72],
            vec![16, 19, 21, 24, 26, 28, 31, 33, 36, 38],
        ), // Typically 10-note base
        (
            "Asha",
            vec![45, 48, 50, 52, 55, 57, 60, 62, 64, 67, 69],
            vec![10, 13, 16, 18, 21, 23, 26, 28, 30, 33, 35],
        ), // Typically 11-note base
    ];

    let mut scales = Vec::new();
    let mut id_counter = 0; // Initialize the ID counter

    // Iterate over each desired note count
    for note_count in 9..=13 {
        // Iterate over each full scale
        for (name, full_midi, full_tpc) in &full_scales {
            // Clip the full scale to the desired number of notes
            let clipped_midi = full_midi.iter().take(note_count).cloned().collect::<Vec<_>>();
            let clipped_tpc = full_tpc.iter().take(note_count).cloned().collect::<Vec<_>>();

            if full_midi.len() >= note_count {
                // Append to the scales list with an auto-incrementing ID
                scales.push((id_counter, *name, clipped_midi, clipped_tpc));
                id_counter += 1; // Increment the ID counter
            }
        }
    }

    scales
}

/**
 * Retrieves a handpan scale by its ID.
 *
 * This function:
 *
 * 1. **Fetches the Scale List**: Calls `scales_list` to get the list of all scales.
 * 2. **Finds the Scale**: Searches for the scale with the given ID.
 * 3. **Returns**: If found, returns the scale's name, MIDI notes, and TPC values as a tuple. Otherwise, returns `None`.
 *
 * @param scale_index The ID of the scale to retrieve.
 * @return An `Option` containing a tuple with the scale's name (String), MIDI notes (Vec<u8>), and TPC values (Vec<i8>).
 */
pub fn get_handpan_scale(scale_index: usize) -> Option<(String, Vec<u8>, Vec<i8>)> {
    scales_list()
        .into_iter()
        .find(|(id, _, _, _)| *id == scale_index)
        .map(|(_, name, notes, tpc)| (name.to_string(), notes, tpc))
}

/**
 * Converts a MIDI note number and TPC value into a human-readable note name and octave.
 *
 * This function:
 *
 * 1. **Defines Mapping**: Maps TPC values to corresponding note names.
 * 2. **Calculates Octave**: Determines the octave number based on the MIDI note.
 * 3. **Adjusts Note Name**: Adjusts the note name based on the TPC value.
 * 4. **Returns**: A tuple containing the note name (String) and octave (i8).
 *
 * @param midi The MIDI note number.
 * @param tpc The Tonnetz Pitch Class (TPC) value.
 * @return A tuple containing the note name (String) and octave (i8).
 */
pub fn midi_to_note_and_octave_with_tpc(midi: u8, tpc: i8) -> (String, i8) {
    // Mapping TPC to note names based on the provided array
    let tpc_to_note = [
        "F♭♭", "C♭♭", "G♭♭", "D♭♭", "A♭♭", "E♭♭", "B♭♭", "F♭", "C♭", "G♭", "D♭", "A♭", "E♭", "B♭",
        "F", "C", "G", "D", "A", "E", "B", "F♯", "C♯", "G♯", "D♯", "A♯", "E♯", "B♯", "F♯♯", "C♯♯",
        "G♯♯", "D♯♯", "A♯♯", "E♯♯", "B♯♯",
    ];

    // Calculate the octave
    let octave = (midi as i8 / 12) - 1;

    // Adjust for TPC indexing
    let note_name = if tpc >= -1 && tpc <= 33 {
        tpc_to_note[(tpc + 1) as usize].to_string()
    } else {
        "Invalid TPC".to_string() // Handle invalid TPC values
    };

    (note_name, octave)
}

/**
 * Finds the best transposition for a set of notes to match a given scale.
 *
 * This function:
 *
 * 1. **Iterates Transpositions**: Tests transpositions from -12 to +12 semitones.
 * 2. **Matches Notes**: For each transposition, counts the number of notes that match the target scale.
 * 3. **Evaluates Intervals**: Considers the preservation of harmonic intervals and applies penalties for mismatches.
 * 4. **Returns**: The transposition value that yields the highest score.
 *
 * @param notes A slice of MIDI notes to be transposed.
 * @param scale_notes A slice of MIDI notes representing the target scale.
 * @return The best transposition value (i32).
 */
pub fn find_best_transposition_with_harmonic_context(notes: &[u8], scale_notes: &[u8]) -> i32 {
    let mut best_transpose = 0;
    let mut max_score = 0.0;

    // Iterate over possible transpositions
    for transpose in -12..=12 {
        let mut matched_notes = 0;
        let mut interval_penalty = 0.0;

        // Transpose and score each note
        for i in 0..notes.len() {
            let transposed_note = (notes[i] as i32 + transpose) as u8;

            // Check if the transposed note is in the scale
            if scale_notes.contains(&transposed_note) {
                matched_notes += 1;
            }

            // Evaluate harmonic intervals if not the last note
            if i < notes.len() - 1 {
                let original_interval = (notes[i + 1] as i32 - notes[i] as i32).abs();
                let transposed_interval =
                    ((notes[i + 1] as i32 + transpose) - (notes[i] as i32 + transpose)).abs();

                // Penalize if the interval changes too much
                if original_interval != transposed_interval {
                    interval_penalty += (original_interval - transposed_interval).abs() as f64;
                }
            }
        }

        // Calculate a score considering both note matching and interval preservation
        let score = (matched_notes as f64) - interval_penalty;

        // Update the best transposition if this one scores higher
        if score > max_score {
            max_score = score;
            best_transpose = transpose;
        }
    }

    best_transpose
}

/**
 * Transposes a MIDI pitch and TPC value by a given number of semitones.
 *
 * This function:
 *
 * 1. **Applies Transposition**: Adjusts the MIDI pitch by the specified transposition value.
 * 2. **Determines New TPC**: Determines the new TPC value based on the transposition direction (positive for sharps, negative for flats).
 * 3. **Returns**: A tuple containing the transposed MIDI pitch and TPC value.
 *
 * @param pitch The original MIDI pitch.
 * @param tpc The original TPC value.
 * @param transpose The number of semitones to transpose.
 * @return An `Option` containing a tuple with the transposed MIDI pitch (u8) and TPC value (i8).
 */
pub fn transpose_pitch_and_tpc(pitch: u8, tpc: Option<i8>, transpose: i32) -> Option<(u8, i8)> {
    let pitch = pitch;
    if let Some(tpc) = tpc {
        // Appliquer la transposition au pitch
        let new_pitch = (pitch as i32 + transpose).clamp(0, 127) as u8;

        // Calculer le modulo 12 pour obtenir la classe de note (note class)
        let note_class = (new_pitch % 12) as i8;

        // Déterminer le nouveau TPC en fonction de la direction de la transposition
        let new_tpc = if transpose > 0 {
            // Utilisation des sharp (dièses) pour les transpositions positives
            match note_class {
                0 => 14,  // C
                1 => 21,  // C#
                2 => 16,  // D
                3 => 23,  // D#
                4 => 18,  // E
                5 => 13,  // F
                6 => 20,  // F#
                7 => 15,  // G
                8 => 22,  // G#
                9 => 17,  // A
                10 => 24, // A#
                11 => 19, // B
                _ => tpc,
            }
        } else if transpose < 0 {
            // Utilisation des flats (bémols) pour les transpositions négatives
            match note_class {
                0 => 14,  // C
                1 => 9,   // Db
                2 => 16,  // D
                3 => 11,  // Eb
                4 => 18,  // E
                5 => 13,  // F
                6 => 8,   // Gb
                7 => 15,  // G
                8 => 10,  // Ab
                9 => 17,  // A
                10 => 12, // Bb
                11 => 19, // B
                _ => tpc,
            }
        } else {
            // Si transpose est 0, renvoyer le TPC original
            tpc
        };

        return Some((new_pitch as u8, new_tpc as i8));
    }
    None
}
