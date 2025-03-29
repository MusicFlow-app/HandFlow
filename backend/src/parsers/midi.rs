use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use crate::error::AppError;

#[derive(Serialize, Debug, Clone)]
struct Note {
    pitch: u8,
    duration: String,
    hand: bool,
    note_type: u8,
}

#[derive(Serialize)]
struct Chord(Vec<Note>);

#[derive(Serialize)]
struct Measure {
    id: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_signature: Option<String>,
    chords: Vec<Chord>,
}

#[derive(Serialize)]
struct Part {
    id: usize,
    name: String,
    measures: Vec<Measure>,
}

#[derive(Serialize)]
struct Metadata {
    work_title: String,
    composer: String,
    arranger: String,
    tempo: u32,
    key_signature: String,
    difficulty: u8,
    category: u8,
}

#[derive(Serialize)]
struct ScoreData {
    parts: Vec<Part>,
}

#[derive(Debug, Clone)]
enum TimelineEvent {
    Note(Note),
}

#[derive(Debug, Clone)]
struct MeasureData {
    first_note_time: Option<u32>,
    last_note_time: Option<u32>,
}

fn format_key_signature(key: i8, scale: u8) -> String {
    let names = [
        "C", "G", "D", "A", "E", "B", "F#", "C#", "F", "Bb", "Eb", "Ab", "Db", "Gb", "Cb",
    ];
    let index = if key >= 0 {
        key as usize
    } else {
        (7 - key.abs() as usize) + 7
    };
    let name = names.get(index).unwrap_or(&"C");
    let mode = if scale == 0 { "maj" } else { "min" };
    format!("{}{}", name, mode)
}

fn calculate_note_ticks(ppq: u16) -> (u32, u32, u32, u32, u32, u32, u32) {
    let whole_note_ticks = ppq as u32 * 4;
    let half_note_ticks = ppq as u32 * 2;
    let quarter_note_ticks = ppq as u32;
    let eighth_note_ticks = ppq as u32 / 2;
    let sixteenth_note_ticks = ppq as u32 / 4;
    let thirty_second_note_ticks = ppq as u32 / 8;
    let sixty_fourth_note_ticks = ppq as u32 / 16;
    
    (whole_note_ticks, half_note_ticks, quarter_note_ticks, eighth_note_ticks,
     sixteenth_note_ticks, thirty_second_note_ticks, sixty_fourth_note_ticks)
}

fn duration_to_ticks(duration: &str, ppq: u16) -> u32 {
    let (whole_note_ticks, half_note_ticks, quarter_note_ticks, eighth_note_ticks,
         sixteenth_note_ticks, thirty_second_note_ticks, sixty_fourth_note_ticks) = calculate_note_ticks(ppq);
         
    match duration {
        "whole" => whole_note_ticks,
        "half" => half_note_ticks,
        "quarter" => quarter_note_ticks,
        "eighth" => eighth_note_ticks,
        "16th" => sixteenth_note_ticks,
        "32nd" => thirty_second_note_ticks,
        "64th" => sixty_fourth_note_ticks,
        _ => quarter_note_ticks, // Default to quarter note
    }
}

fn determine_rest_duration(gap: u32, ppq: u16) -> &'static str {
    let (whole_note_ticks, half_note_ticks, quarter_note_ticks, eighth_note_ticks,
         sixteenth_note_ticks, thirty_second_note_ticks, sixty_fourth_note_ticks) = calculate_note_ticks(ppq);
    
    if gap >= whole_note_ticks {
        "whole"
    } else if gap >= half_note_ticks {
        "half"
    } else if gap >= quarter_note_ticks {
        "quarter"
    } else if gap >= eighth_note_ticks {
        "eighth"
    } else if gap >= sixteenth_note_ticks {
        "16th"
    } else if gap >= thirty_second_note_ticks {
        "32nd"
    } else if gap >= sixty_fourth_note_ticks {
        "64th"
    } else {
        "32nd"
    }
}

fn duration_from_ticks(ticks: u32, ppq: u16) -> Option<&'static str> {
    // Convert ticks to beats (1 beat = 1 quarter note)
    let beats = ticks as f32 / ppq as f32;
    
    // Calculate exact duration in beats
    let duration_map = [
        (4.0, "whole"),
        (2.0, "half"),
        (1.0, "quarter"),
        (0.5, "eighth"),
        (0.25, "16th"),
        (0.125, "32nd"),
        (0.0625, "64th")
    ];

    // Find the closest standard duration
    duration_map
        .iter()
        .min_by(|&&(a, _), &&(b, _)| {
            (a - beats).abs().partial_cmp(&(b - beats).abs()).unwrap()
        })
        .map(|&(_, name)| name)
}

pub fn parse_midi(data: &[u8]) -> Result<(serde_json::Value, serde_json::Value), AppError> {
    log::info!("Starting MIDI parsing");
    let smf = Smf::parse(data).map_err(|e| AppError::Parse(e.to_string()))?;
    
    let ppq = match smf.header.timing {
        Timing::Metrical(t) => {
            let ppq = t.as_int();
            ppq
        },
        _ => return Err(AppError::Parse("Only metrical timing supported".to_string())),
    };

    let mut work_title = String::new();
    let mut composer = String::new();
    let mut arranger = String::new();
    let mut tempo_bpm = 120; // Default tempo
    let mut initial_tempo_set = false; // Flag to track if we've set the initial tempo
    let mut key_signature = "Cmaj".to_string();
    
    // default time signature = 4/4
    let mut numer = 4;
    let mut denom = 2u8;
    
    let mut parts = Vec::new();

    for (i, track) in smf.tracks.iter().enumerate() {
        let mut part_name = format!("Part {}", i + 1);
        let mut abs_time = 0u32;
        let mut active_notes: HashMap<u8, u32> = HashMap::new();
        let mut note_events: Vec<(u32, Note)> = Vec::new();
        
        // Track all note on/off events for better analysis
        let mut all_midi_events: Vec<(u32, String, u8)> = Vec::new();

        let mut time_sig_events: BTreeMap<u32, String> = BTreeMap::new();
        
        for event in track {
            abs_time += event.delta.as_int() as u32;

            match event.kind {
                TrackEventKind::Meta(meta) => match meta {
                    MetaMessage::TrackName(name) => {
                        if work_title.is_empty() {
                            work_title = String::from_utf8_lossy(name).to_string();
                        }
                    }
                    MetaMessage::Text(text) => {
                        let text_str = String::from_utf8_lossy(text).to_string();
                        if text_str.to_lowercase().contains("composer:") {
                            composer = text_str.split(':').nth(1).unwrap_or("").trim().to_string();
                        } else if text_str.to_lowercase().contains("arranger:") {
                            arranger = text_str.split(':').nth(1).unwrap_or("").trim().to_string();
                        } else if work_title.is_empty() {
                            work_title = text_str;
                        }
                    }
                    MetaMessage::InstrumentName(name) => {
                        part_name = String::from_utf8_lossy(name).to_string();
                    }
                    MetaMessage::Tempo(us_per_quarter) => {
                        let us_per_quarter_value = us_per_quarter.as_int();
                        let new_tempo_bpm = 60_000_000 / us_per_quarter_value;
                        log::info!("Tempo event: {} microseconds per quarter note = {} BPM", 
                                  us_per_quarter_value, new_tempo_bpm);
                        
                        // Only use the first tempo event we encounter
                        if !initial_tempo_set {
                            tempo_bpm = new_tempo_bpm;
                            initial_tempo_set = true;
                        }
                    }
                    MetaMessage::KeySignature(key, scale) => {
                        key_signature = format_key_signature(key, scale.into());
                    }
                    MetaMessage::TimeSignature(n, d, _, _) => {
                        let sig = format!("{}|{}", n, 2u8.pow(d.into()));
                        time_sig_events.insert(abs_time, sig.clone());
                        numer = n;
                        denom = d;
                    }
                    _ => {}
                },
                TrackEventKind::Midi { message, .. } => match message {
                    MidiMessage::NoteOn { key, vel } if vel.as_int() > 0 => {
                        all_midi_events.push((abs_time, "NoteOn".to_string(), key.as_int()));
                        
                        active_notes.insert(key.as_int(), abs_time);
                        note_events.push((
                            abs_time,
                            Note {
                                pitch: key.as_int(),
                                duration: "quarter".to_string(), // Will be updated on NoteOff
                                hand: i == 0,
                                note_type: 1,
                            },
                        ));
                    }
                    MidiMessage::NoteOff { key, .. } => {
                        all_midi_events.push((abs_time, "NoteOff".to_string(), key.as_int()));
                        
                        if let Some(start_time) = active_notes.remove(&key.as_int()) {
                            let duration_ticks = abs_time - start_time;
                            
                            if let Some(duration_name) = duration_from_ticks(duration_ticks, ppq) {
                                if let Some(idx) = note_events.iter().position(|(t, n)| 
                                    *t == start_time && n.pitch == key.as_int()
                                ) {
                                    note_events[idx].1.duration = duration_name.to_string();
                                }
                            }
                        }
                    }
                    MidiMessage::NoteOn { key, vel } if vel.as_int() == 0 => {
                        all_midi_events.push((abs_time, "NoteOff".to_string(), key.as_int()));
                        
                        if let Some(start_time) = active_notes.remove(&key.as_int()) {
                            let duration_ticks = abs_time - start_time;
                            
                            if let Some(duration_name) = duration_from_ticks(duration_ticks, ppq) {
                                if let Some(idx) = note_events.iter().position(|(t, n)| 
                                    *t == start_time && n.pitch == key.as_int()
                                ) {
                                    note_events[idx].1.duration = duration_name.to_string();
                                }
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // We'll use helper functions that calculate note durations based on ppq
        
        // Sort all note events by timestamp
        let mut sorted_note_events = note_events.clone();
        sorted_note_events.sort_by_key(|(time, _)| *time);
        
        log::info!("Total note events to process: {}", sorted_note_events.len());
        
        // Create a timeline of all events
        // This will help us identify gaps (potential rests)
        let mut musical_timeline: BTreeMap<u32, Vec<TimelineEvent>> = BTreeMap::new();
        
        // Track measure boundaries for more informed decisions
        let mut measure_boundaries: BTreeSet<u32> = BTreeSet::new();
        for measure in 0..=((sorted_note_events.last().map(|(t, _)| t).unwrap_or(&0) / (numer as u32 * ppq as u32)) + 1) {
            measure_boundaries.insert(measure * (numer as u32 * ppq as u32));
        }
        
        // First pass: collect all note events in this measure
        for (timestamp, note) in sorted_note_events.iter() {
            // Add to timeline
            musical_timeline.entry(*timestamp)
                .or_insert_with(|| Vec::new())
                .push(TimelineEvent::Note(note.clone()));
        }
        
        // Now identify gaps and add rests
        let mut active_measure_data: BTreeMap<u32, MeasureData> = BTreeMap::new();
        
        // Process the timeline in chronological order
        let mut complete_chords: Vec<(u32, Vec<Note>)> = Vec::new();
        
        // Analyze measure by measure for better musical context
        let max_measure = sorted_note_events.last()
            .map(|(t, _)| t / (numer as u32 * ppq as u32))
            .unwrap_or(0);
        
        log::info!("Processing {} measures for rest detection", max_measure + 1);
        
        for measure in 0..=max_measure {
            let measure_start = measure * (numer as u32 * ppq as u32);
            let measure_end = (measure + 1) * (numer as u32 * ppq as u32) - 1;
            
            // Get all events in this measure
            let measure_events: Vec<(u32, &Vec<TimelineEvent>)> = musical_timeline
                .range(measure_start..=measure_end)
                .map(|(k, v)| (*k, v))
                .collect();
            
            if measure_events.is_empty() {
                
                // Add a whole rest at the start of the measure
                complete_chords.push((measure_start, vec![Note {
                    pitch: 0,
                    duration: "whole".to_string(),
                    hand: false,
                    note_type: 0,
                }]));
                continue;
            }
            
            // Initialize or get existing measure data
            let measure_data = active_measure_data
                .entry(measure)
                .or_insert_with(|| MeasureData {
                    last_note_time: None,
                    first_note_time: None,
                });
            
            // First pass: collect all note events in this measure
            for (time, events) in &measure_events {
                for event in events.iter() {
                    let TimelineEvent::Note(note) = event;
                    if note.pitch > 0 {  // Actual note, not a rest
                        // Update measure data with first and last note times
                        if measure_data.first_note_time.is_none() {
                            measure_data.first_note_time = Some(*time);
                        }
                        measure_data.last_note_time = Some(*time);
                    }
                }
            }
            
            
            // Second pass: process events and detect gaps
            let mut last_time_in_measure: Option<u32> = None;
            
            for (time, events) in &measure_events {
                // Extract actual notes (not rests)
                let notes: Vec<Note> = events.iter()
                    .filter_map(|e| {
                        let TimelineEvent::Note(note) = e;
                        if note.pitch > 0 {
                            Some(note.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                
                // Check for gap since last event (potential rest)
                if let Some(last_time) = last_time_in_measure {
                    // Determine the previous note's duration
                    let prev_note_idx = if let Some(pos) = sorted_note_events.iter().position(|(t, _)| *t == last_time) {
                        pos
                    } else {
                        sorted_note_events.iter().position(|(t, _)| *t < last_time).unwrap_or(0)
                    };
                    
                    let prev_note_duration_ticks = if prev_note_idx < sorted_note_events.len() {
                        let (_, note) = &sorted_note_events[prev_note_idx];
                        duration_to_ticks(note.duration.as_str(), ppq)
                    } else {
                        0
                    };
                    
                    // Use modified gap calculation formula that includes note duration
                    let gap = if *time > last_time + prev_note_duration_ticks {
                        *time - last_time - prev_note_duration_ticks
                    } else {
                        0 // No gap if notes overlap or are too close
                    };
                    
                    // Only consider gaps that are musically significant
                    // A sixty-fourth note is ppq/16
                    if gap >= ppq as u32 / 16 {
                        // Determine appropriate rest duration based on gap size
                        let rest_duration = determine_rest_duration(gap, ppq);

                        
                        // Add the rest to complete_chords
                        complete_chords.push((last_time, vec![Note {
                            pitch: 0,
                            duration: rest_duration.to_string(),
                            hand: false,
                            note_type: 0,
                        }]));
                    }
                } else if *time > measure_start && measure_data.first_note_time.unwrap_or(0) > measure_start {
                    // Handle rest at beginning of measure if needed
                    let gap = *time - measure_start;
                    
                    // A sixty-fourth note is ppq/16
                    if gap >= ppq as u32 / 16 {
                        // Determine appropriate rest duration for gap at start of measure
                        let rest_duration = determine_rest_duration(gap, ppq);
                        
                        // Add the rest at the start of the measure
                        complete_chords.push((measure_start, vec![Note {
                            pitch: 0,
                            duration: rest_duration.to_string(),
                            hand: false,
                            note_type: 0,
                        }]));
                    }
                }
                
                // Add the current notes to complete_chords if there are any
                if !notes.is_empty() {
                    complete_chords.push((*time, notes));
                }
                
                // Update last time for gap detection
                last_time_in_measure = Some(*time);
            }
            
            // Check for gap at end of measure
            if let Some(last_time) = last_time_in_measure {
                // Calculate the total measure duration based on time signature
                // For a 4/4 time signature, this would be 4 quarter notes
                // A quarter note is ppq ticks
                let expected_measure_duration = (numer as u32) * ppq as u32;
                
                // Calculate how many beats have been used in this measure
                // We need to collect all notes in this measure and calculate the total occupied time
                let mut used_ticks_in_measure = 0;

                // Iterate through each chord in the measure
                for (time, notes) in &complete_chords {
                    if *time >= measure_start && *time < measure_end {
                        // Iterate through each note in the chord
                        for note in notes {
                            let duration_ticks = duration_to_ticks(note.duration.as_str(), ppq);

                            used_ticks_in_measure += duration_ticks;
                        }
                    }
                }

                // Calculate how many ticks are remaining in the measure
                let remaining_ticks = if expected_measure_duration > used_ticks_in_measure {
                    expected_measure_duration - used_ticks_in_measure
                } else {
                    0 // Measure is already full or overflowing
                };
                
                // Only add a rest if there are enough remaining ticks to be musically significant
                // A sixty-fourth note is ppq/16
                if remaining_ticks >= ppq as u32 / 16 {
                    // Find the appropriate rest duration based on the remaining ticks
                    let rest_duration = determine_rest_duration(remaining_ticks, ppq);
                    
                    // Add the rest at the end of this measure
                    complete_chords.push((last_time, vec![Note {
                        pitch: 0,
                        duration: rest_duration.to_string(),
                        hand: false,
                        note_type: 0,
                    }]));
                }
            }
        }
        
        // Ensure chords are sorted by timestamp
        complete_chords.sort_by_key(|(time, _)| *time);

        let mut measures = Vec::new();
        let mut current_measure_chords = Vec::new();
        let mut measure_id = 1;
        let mut measure_start_time = 0u32;
        let mut previous_signature: Option<String> = None;
        let mut current_hand = false;

        // Calculate ticks per measure based on time signature
        // For 4/4 time: 4 beats per measure * ppq ticks per beat
        // For 3/4 time: 3 beats per measure * ppq ticks per beat
        // For 6/8 time: 6 beats per measure * (ppq/2) ticks per beat (8th note gets the beat)
        let mut ticks_per_measure = match denom {
            // If denominator is 8 (eighth note gets the beat), adjust ppq accordingly
            3 => (numer as u32) * (ppq as u32 / 2),
            // For quarter note and half note denominators
            _ => (numer as u32) * ppq as u32
        };

        for (timestamp, notes) in complete_chords {
            if let Some(tsig) = time_sig_events.get(&timestamp) {
                if Some(tsig.clone()) != previous_signature {
                    let parts: Vec<_> = tsig.split('|').collect();
                    if parts.len() == 2 {
                        if let (Ok(n), Ok(d)) = (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
                            numer = n;
                            denom = (d as f32).log2() as u8;
                            // Update ticks per measure based on new time signature
                            ticks_per_measure = match denom {
                                // If denominator is 8 (eighth note gets the beat), adjust ppq accordingly
                                3 => (numer as u32) * (ppq as u32 / 2),
                                // For quarter note and half note denominators
                                _ => (numer as u32) * ppq as u32
                            };
                            log::info!("New time signature: {}|{}, ticks per measure: {}", 
                                numer, 2u8.pow(denom.into()), ticks_per_measure);
                        }
                    }
                }
            }

            // Calculate current measure based on timestamp
            let current_measure = timestamp / ticks_per_measure;
            let start_measure = measure_start_time / ticks_per_measure;
            
            if current_measure > start_measure {
                let signature = time_sig_events.get(&measure_start_time).cloned();
                let include_sig = signature.as_ref().map_or(false, |sig| Some(sig) != previous_signature.as_ref());

                measures.push(Measure {
                    id: measure_id,
                    time_signature: if include_sig { signature.clone() } else { None },
                    chords: current_measure_chords,
                });

                previous_signature = signature;
                current_measure_chords = Vec::new();
                measure_id += 1;
                measure_start_time = current_measure * ticks_per_measure;
            }

            // Update notes with the current hand value and toggle for next chord
            let mut updated_notes = notes;
            for note in &mut updated_notes {
                note.hand = current_hand;
            }
            current_hand = !current_hand; // Toggle hand for next chord
            
            current_measure_chords.push(Chord(updated_notes));
        }

        if !current_measure_chords.is_empty() {
            let signature = time_sig_events.get(&measure_start_time).cloned();
            let include_sig = signature.as_ref().map_or(false, |sig| Some(sig) != previous_signature.as_ref());

            measures.push(Measure {
                id: measure_id,
                time_signature: if include_sig { signature } else { None },
                chords: current_measure_chords,
            });
        }

        // Check if this part contains any actual notes (not just rests)
        let has_actual_notes = measures.iter().any(|measure| {
            measure.chords.iter().any(|chord| {
                chord.0.iter().any(|note| note.pitch > 0)
            })
        });
        
        // Only add the part if it contains at least one actual note
        if has_actual_notes {
            log::info!("Adding part {} with {} measures", i + 1, measures.len());
            parts.push(Part {
                id: i + 1,
                name: part_name,
                measures,
            });
        } else {
            log::info!("Skipping part {} as it contains only rests", i + 1);
        }
    }

    let metadata = serde_json::to_value(Metadata {
        work_title: work_title,
        composer,
        arranger,
        tempo: tempo_bpm,
        key_signature: key_signature,
        difficulty: 2,
        category: 2,
    }).map_err(|e| AppError::Parse(e.to_string()))?;

    let score_data = serde_json::to_value(ScoreData { parts })
        .map_err(|e| AppError::Parse(e.to_string()))?;

    Ok((metadata, score_data))
}