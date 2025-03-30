use serde::{Deserialize, Serialize};
use midly::num::u7;
use crate::models::key_signature::KeySignature;
use crate::models::{Category, Difficulty};

#[derive(Debug, Clone, Copy)]
pub struct MidiPitch(u7);

// Custom serialization to store as u8
impl Serialize for MidiPitch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.as_int().serialize(serializer)
    }
}

// Custom deserialization from u8
impl<'de> Deserialize<'de> for MidiPitch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Ok(Self(u7::new(value)))
    }
}

// Implement Display for formatting
impl std::fmt::Display for MidiPitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_int())
    }
}

// Implement comparison with integers
impl PartialEq<u8> for MidiPitch {
    fn eq(&self, other: &u8) -> bool {
        self.0.as_int() == *other
    }
}

impl PartialOrd<u8> for MidiPitch {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.as_int().partial_cmp(other)
    }
}

impl MidiPitch {
    pub fn new(value: u8) -> Self {
        Self(u7::new(value))
    }

    pub fn as_int(&self) -> u8 {
        self.0.as_int()
    }

    pub fn as_u7(&self) -> u7 {
        self.0
    }
}

impl From<u7> for MidiPitch {
    fn from(value: u7) -> Self {
        Self(value)
    }
}

impl From<MidiPitch> for u7 {
    fn from(value: MidiPitch) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum NoteType {
    Rest = 0,
    Normal = 1,
    Ghost = 2,
    Dead = 3,
    Grace = 4,
    Cue = 5,
    Slash = 6,
    Harmonic = 7,
}

impl Default for NoteType {
    fn default() -> Self {
        NoteType::Normal
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NoteDuration {
    Whole,      // 4 beats
    Half,       // 2 beats
    Quarter,    // 1 beat
    Eighth,     // 1/2 beat
    Sixteenth,  // 1/4 beat
    ThirtySecond, // 1/8 beat
    SixtyFourth,  // 1/16 beat
}

impl std::fmt::Display for NoteDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl NoteDuration {
    pub fn from_str(s: &str) -> Self {
        match s {
            "whole" => Self::Whole,
            "half" => Self::Half,
            "quarter" => Self::Quarter,
            "eighth" => Self::Eighth,
            "16th" => Self::Sixteenth,
            "32nd" => Self::ThirtySecond,
            "64th" => Self::SixtyFourth,
            _ => Self::Quarter, // Default to quarter note
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Whole => "whole",
            Self::Half => "half",
            Self::Quarter => "quarter",
            Self::Eighth => "eighth",
            Self::Sixteenth => "16th",
            Self::ThirtySecond => "32nd",
            Self::SixtyFourth => "64th",
        }
    }

    pub fn to_fraction(&self) -> f32 {
        match self {
            Self::Whole => 4.0,
            Self::Half => 2.0,
            Self::Quarter => 1.0,
            Self::Eighth => 0.5,
            Self::Sixteenth => 0.25,
            Self::ThirtySecond => 0.125,
            Self::SixtyFourth => 0.0625,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Hand {
    Left,
    Right
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub pitch: MidiPitch,
    pub duration: NoteDuration,
    #[serde(default)]
    pub note_type: NoteType,
    pub hand: Hand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chord {
    #[serde(default)]
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measure {
    pub id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_signature: Option<(u8, u8)>,
    pub chords: Vec<Vec<Note>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    pub id: u32,
    pub name: String,
    pub measures: Vec<Measure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreData {
    pub parts: Vec<Part>,
}

pub fn default_unknown() -> String {
    "Unknown".to_string()
}

pub fn default_cmaj() -> KeySignature {
    KeySignature::Cmaj
}

pub fn default_tempo() -> u32 {
    120
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(default = "default_unknown")]
    pub work_title: String,
    #[serde(default = "default_unknown")]
    pub composer: String,
    #[serde(default = "default_unknown")]
    pub arranger: String,
    #[serde(default = "default_cmaj")]
    pub key_signature: KeySignature,
    #[serde(default = "default_tempo")]
    pub tempo: u32,
    #[serde(default)]
    pub difficulty: Difficulty,
    #[serde(default)]
    pub category: Category,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreJson {
    pub file_size: u32,
    pub metadata: Metadata,
    pub score_data: ScoreData,
}
