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

#[derive(Serialize)]
struct Output {
    metadata: Metadata,
    score_data: ScoreData,
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
    let base = ppq as f32;
    let ratio = ticks as f32 / base;
    match ratio {
        x if (x - 4.0).abs() < 0.2 => Some("whole"),
        x if (x - 2.0).abs() < 0.2 => Some("half"),
        x if (x - 1.0).abs() < 0.2 => Some("quarter"),
        x if (x - 0.5).abs() < 0.1 => Some("eighth"),
        x if (x - 0.25).abs() < 0.05 => Some("16th"),
        x if (x - 0.125).abs() < 0.03 => Some("32nd"),
        x if (x - 0.0625).abs() < 0.015 => Some("64th"),
        _ => None,
    }
}

pub fn parse_midi(data: &[u8]) -> Result<serde_json::Value, AppError> {
    let smf = Smf::parse(data).map_err(|e| AppError::Parse(e.to_string()))?;

    let ppq = match smf.header.timing {
        Timing::Metrical(t) => t.as_int(),
        _ => return Err(AppError::Parse("Only metrical timing supported".to_string())),
    };

    let mut work_title = String::new();
    let mut composer = String::new();
    let mut arranger = String::new();
    let mut tempo_bpm = 120;
    let mut key_signature = "Cmaj".to_string();
    let mut current_notes = Vec::new();

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

        let mut time_sig_events: BTreeMap<u32, String> = BTreeMap::new();

        for event in track {
            abs_time += event.delta.as_int();

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
                    MidiMessage::NoteOn { key, vel } if vel > 0 => {
                        active_notes.insert(key.as_int(), abs_time);
                    }
                    MidiMessage::NoteOff { key, .. } => {
                        if let Some(start_time) = active_notes.remove(&key.as_int()) {
                            let duration_ticks = abs_time - start_time;
                            if let Some(duration_name) = duration_from_ticks(duration_ticks, ppq) {
                                current_notes.push(Note {
                                    pitch: key.as_int() as u8,
                                    duration: duration_name.to_string(),
                                    hand: true,
                                    note_type: 1,
                                });
                            }
                        }
                    }
                    MidiMessage::NoteOn { key, vel } if vel.as_int() == 0 => {
                        if let Some(start_time) = active_notes.remove(&key.as_int()) {
                            let duration_ticks = abs_time - start_time;
                            if let Some(duration_name) = duration_from_ticks(duration_ticks, ppq) {
                                note_events.push((
                                    start_time,
                                    Note {
                                        pitch: key.as_int(),
                                        duration: duration_name.to_string(),
                                        hand: false,
                                        note_type: 1,
                                    },
                                ));
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let mut grouped_chords: BTreeMap<u32, Vec<Note>> = BTreeMap::new();
        for (start_time, note) in note_events {
            grouped_chords.entry(start_time).or_default().push(note);
        }

        let mut complete_chords: Vec<(u32, Vec<Note>)> = Vec::new();
        let mut last_time = 0u32;

        for (&start_time, note_group) in &grouped_chords {
            if start_time > last_time {
                let gap = start_time - last_time;
                if let Some(duration) = duration_from_ticks(gap, ppq) {
                    complete_chords.push((
                        last_time,
                        vec![Note {
                            pitch: 0,
                            duration: duration.to_string(),
                            hand: false,
                            note_type: 0,
                        }],
                    ));
                }
            }
            complete_chords.push((start_time, note_group.clone()));
            last_time = start_time;
        }

        let mut measures = Vec::new();
        let mut current_measure_chords = Vec::new();
        let mut measure_id = 1;
        let mut measure_start_time = 0u32;
        let mut previous_signature: Option<String> = None;

        let mut ticks_per_measure = (numer as u32) * ppq as u32 / 2u32.pow(denom as u32);

        for (timestamp, notes) in complete_chords {
            if let Some(tsig) = time_sig_events.get(&timestamp) {
                if Some(tsig.clone()) != previous_signature {
                    let parts: Vec<_> = tsig.split('|').collect();
                    if parts.len() == 2 {
                        if let (Ok(n), Ok(d)) = (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
                            numer = n;
                            denom = (d as f32).log2() as u8;
                            ticks_per_measure = (numer as u32) * ppq as u32 / 2u32.pow(denom as u32);
                        }
                    }
                }
            }

            if timestamp >= measure_start_time + ticks_per_measure {
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
                measure_start_time += ticks_per_measure;
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

    let output = Output {
        metadata: Metadata {
            work_title: work_title,
            composer,
            arranger,
            tempo: tempo_bpm,
            key_signature: key_signature,
            difficulty: 2,
            category: 2,
        },
        score_data: ScoreData { parts },
    };

    let json_value = serde_json::to_value(&output).map_err(|e| AppError::Parse(e.to_string()))?;
    Ok(json_value)
}