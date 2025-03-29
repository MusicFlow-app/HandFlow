use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use crate::error::AppError;

#[derive(Serialize, Clone)]
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
            log::info!("PPQ (ticks per quarter note): {}", ppq);
            ppq
        },
        _ => return Err(AppError::Parse("Only metrical timing supported".to_string())),
    };

    let mut work_title = String::new();
    let mut composer = String::new();
    let mut arranger = String::new();
    let mut tempo_bpm = 120;
    let mut key_signature = "Cmaj".to_string();
    let mut _current_notes: Vec<Note> = Vec::new();

    // default time signature = 4/4
    let mut numer = 4;
    let mut denom = 2u8;
    let mut _current_time_signature = format!("{}|{}", numer, 2u8.pow(denom.into()));

    let mut parts = Vec::new();

    for (i, track) in smf.tracks.iter().enumerate() {
        let mut part_name = format!("Part {}", i + 1);
        let mut abs_time = 0u32;
        let mut active_notes: HashMap<u8, u32> = HashMap::new();
        let mut note_events: Vec<(u32, Note)> = Vec::new();
        
        // Track all note on/off events for better analysis
        let mut all_midi_events: Vec<(u32, String, u8)> = Vec::new();

        let mut time_sig_events: BTreeMap<u32, String> = BTreeMap::new();
        
        // Calculate initial ticks per measure (4/4 time)
        let ticks_per_measure = 4 * ppq as u32;
        log::info!("Initial ticks per measure: {}, PPQ: {}", ticks_per_measure, ppq);

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
                        tempo_bpm = 60_000_000 / us_per_quarter.as_int();
                    }
                    MetaMessage::KeySignature(key, scale) => {
                        key_signature = format_key_signature(key, scale.into());
                    }
                    MetaMessage::TimeSignature(n, d, _, _) => {
                        let sig = format!("{}|{}", n, 2u8.pow(d.into()));
                        time_sig_events.insert(abs_time, sig.clone());
                        numer = n;
                        denom = d;
                        _current_time_signature = sig;
                    }
                    _ => {}
                },
                TrackEventKind::Midi { message, .. } => match message {
                    MidiMessage::NoteOn { key, vel } if vel.as_int() > 0 => {
                        // Log NoteOn events for debugging
                        log::debug!("NoteOn: pitch={}, time={} ticks", key.as_int(), abs_time);
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
                        // Log NoteOff events for debugging
                        log::debug!("NoteOff: pitch={}, time={} ticks", key.as_int(), abs_time);
                        all_midi_events.push((abs_time, "NoteOff".to_string(), key.as_int()));
                        
                        if let Some(start_time) = active_notes.remove(&key.as_int()) {
                            let duration_ticks = abs_time - start_time;
                            log::debug!("Note duration: {} ticks for pitch {}", duration_ticks, key.as_int());
                            
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
                        // Log NoteOn with velocity 0 (equivalent to NoteOff) for debugging
                        log::debug!("NoteOn(vel=0): pitch={}, time={} ticks", key.as_int(), abs_time);
                        all_midi_events.push((abs_time, "NoteOff".to_string(), key.as_int()));
                        
                        if let Some(start_time) = active_notes.remove(&key.as_int()) {
                            let duration_ticks = abs_time - start_time;
                            log::debug!("Note duration: {} ticks for pitch {}", duration_ticks, key.as_int());
                            
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

        // Log all MIDI events for analysis
        log::info!("All MIDI events: {}", all_midi_events.len());
        for (time, event_type, pitch) in &all_midi_events {
            log::debug!("MIDI Event: {} pitch={} at {} ticks", event_type, pitch, time);
        }

        // Sort note events by timestamp
        note_events.sort_by_key(|(timestamp, _)| *timestamp);
        
        // Log all note events before grouping
        log::info!("Note events before grouping: {}", note_events.len());
        for (time, note) in &note_events {
            log::debug!("Note event: pitch={}, duration={}, at {} ticks", 
                      note.pitch, note.duration, time);
        }

        // New approach: Quantize notes to a grid based on the time signature
        // This will help determine where rests should be placed
        let grid_resolution = (ppq / 16) as u32; // 64th note resolution for more precise quantization
        let mut quantized_events: BTreeMap<u32, Vec<Note>> = BTreeMap::new();
        
        for (timestamp, note) in note_events {
            // For actual notes (not rests), keep their original timing
            // Only quantize rests
            let quantized_time = if note.pitch > 0 {
                timestamp
            } else {
                (timestamp + grid_resolution / 2) / grid_resolution * grid_resolution
            };
            
            log::debug!("Quantizing note pitch={} from {} to {} ticks", 
                      note.pitch, timestamp, quantized_time);
            
            quantized_events.entry(quantized_time)
                .or_insert_with(Vec::new)
                .push(note);
        }
        
        // Now fill in rests where needed based on musical structure
        let mut complete_chords: Vec<(u32, Vec<Note>)> = Vec::new();
        let mut last_event_time = 0;
        
        // Calculate ticks per measure based on time signature
        // For 4/4 time: 4 beats per measure * ppq ticks per beat
        // For 3/4 time: 3 beats per measure * ppq ticks per beat
        // For 6/8 time: 6 beats per measure * (ppq/2) ticks per beat (8th note gets the beat)
        let ticks_per_measure = (numer as u32) * ppq as u32;
        log::info!("Ticks per measure: {}, PPQ: {}, Time sig: {}|{}", 
            ticks_per_measure, ppq, numer, 2u8.pow(denom.into()));
        
        // Convert quantized events to a sorted vector
        let mut sorted_events: Vec<(u32, Vec<Note>)> = quantized_events
            .into_iter()
            .collect();
        sorted_events.sort_by_key(|(time, _)| *time);
        
        // Process each event and add rests only where musically appropriate
        for (time, notes) in sorted_events {
            if time > last_event_time && last_event_time > 0 {
                let gap = time - last_event_time;
                
                // Only add rest if gap is at least an eighth note (ppq/2)
                // This prevents too many small rests from being added
                if gap >= ppq as u32 / 2 {
                    log::debug!("Adding rest at {} ticks, duration {} ticks (beats: {})", 
                              last_event_time, gap, gap as f32 / ppq as f32);
                    
                    if let Some(duration_name) = duration_from_ticks(gap, ppq) {
                        complete_chords.push((last_event_time, vec![Note {
                            pitch: 0,
                            duration: duration_name.to_string(),
                            hand: false,
                            note_type: 0,
                        }]));
                    }
                }
            }
            
            complete_chords.push((time, notes));
            last_event_time = time;
        }

        let mut measures = Vec::new();
        let mut current_measure_chords = Vec::new();
        let mut measure_id = 1;
        let mut measure_start_time = 0u32;
        let mut previous_signature: Option<String> = None;

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
        
        log::info!("Initial time signature: {}|{}, ticks per measure: {}", 
            numer, 2u8.pow(denom.into()), ticks_per_measure);

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

            log::debug!("Current timestamp: {}, measure_start: {}, ticks_per_measure: {}", 
                timestamp, measure_start_time, ticks_per_measure);
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

            current_measure_chords.push(Chord(notes));
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

        parts.push(Part {
            id: i + 1,
            name: part_name,
            measures,
        });
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