use serde::{Deserialize, Serialize};
use crate::utils::midi::note_to_midi;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

// Import the enum variants for use throughout this file
use self::NotePosition::{Top, Bottom};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, EnumIter, EnumString)]
pub enum ScaleType {
    Aegean,
    Aeolian,
    Akebono,
    Amara,
    AmaraTriple,
    AmaraKurdMini,
    AmaraKurdHijaz,
    Anahata,
    Arboreal,
    Ashakiran,
    Avalon,
    Blues,
    ChaoGuo,
    DeepShello,
    DaNaYo,
    Dorian,
    Equinox,
    EquinoxBottomised,
    EquinoxLowPygmyKurdDorianMini,
    GoldenGate,
    HarmonicLora,
    HarmonicMinor,
    HighAvalon,
    HijazMercury,
    Hijaz,
    HijazKarMercury,
    HijazMajor,
    HijazTarznauyn,
    Insen,
    IntegralMercury,
    IntegralSam,
    Jibuk,
    Kamaji,
    Klezmara,
    Kurd,
    KurdextendedGio,
    LaSirena,
    LowMystic,
    LowPygmy,
    LowPygmyOctave,
    Lydian,
    MagicVoyage,
    Major,
    MelogSelisir,
    Minor,
    Mixolydian4,
    Mixolydian7m,
    MysticHagane,
    MysticSam,
    Nazar,
    Onoleo,
    Oxalis,
    Paradisev1,
    Paradisev2,
    PentatonicMajor,
    Pygmy,
    RagaDesh,
    RomanianHijaz,
    RomanianMineurHarmonique,
    Sabye,
    Saladin,
    ShelloPango,
    Ursaminor,
    YshaSavita,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanScaleCategory {
    Modal,
    Oriental,
    PentatonicAsian,
    Mystical,
    MinorFolk,
    Ethnic,
    Experimental,
}

impl PanScaleCategory {
    /// Retourne toutes les catégories de handpan
    pub fn all() -> Vec<Self> {
        vec![
            PanScaleCategory::Modal,
            PanScaleCategory::Oriental,
            PanScaleCategory::PentatonicAsian,
            PanScaleCategory::Mystical,
            PanScaleCategory::MinorFolk,
            PanScaleCategory::Ethnic,
            PanScaleCategory::Experimental,
        ]
    }
    
    /// Conversion en chaîne de caractères pour l'affichage
    pub fn to_string(&self) -> String {
        match self {
            PanScaleCategory::Modal => "Modal".to_string(),
            PanScaleCategory::Oriental => "Oriental".to_string(),
            PanScaleCategory::PentatonicAsian => "Pentatonic Asian".to_string(),
            PanScaleCategory::Mystical => "Mystical".to_string(),
            PanScaleCategory::MinorFolk => "Minor/Folk".to_string(),
            PanScaleCategory::Ethnic => "Ethnic".to_string(),
            PanScaleCategory::Experimental => "Experimental".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HandpanScale {
    pub id: String,
    pub name: String,
    pub category: PanScaleCategory,
    pub relative_ding: String,
    pub notes: Vec<NoteHandpan>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum NotePosition {
    Top,
    Bottom,
    Inner,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteHandpan {
    pub note_index: u32,
    pub position: NotePosition,
    pub distance_relative_to_ding: i32,
    pub calculated_pitch: Option<i32>,
}


pub const ALL_DINGS: [&str; 20] = [
    "E2", "F2", "F#2", "G2", "G#2", 
    "A2", "A#2", "B2", "C3", "C#3", 
    "D3", "D#3", "E3", "F3", "F#3", 
    "G3", "G#3", "A3", "A#3", "B3",
];

impl ScaleType {
    // Get the associated HandpanScale for this scale type
    pub fn get_scale(&self) -> HandpanScale {
        match self {
            ScaleType::Aegean => HandpanScale {
                id: "Aegean".to_string(),
                name: "Aegean".to_string(),
                category: PanScaleCategory::Experimental,
                relative_ding: "F3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 4, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 18, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 23, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::Aeolian => HandpanScale {
                id: "Aeolian".to_string(),
                name: "Aeolian".to_string(),
                category: PanScaleCategory::Modal,
                relative_ding: "E3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 13, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // A4
                ],
            },
            
            ScaleType::Akebono => HandpanScale {
                id: "Akebono".to_string(),
                name: "Akebono".to_string(),
                category: PanScaleCategory::PentatonicAsian,
                relative_ding: "E3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 13, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // C5
                ],
            },
            
            ScaleType::Amara => HandpanScale {
                id: "Amara".to_string(),
                name: "Amara".to_string(),
                category: PanScaleCategory::MinorFolk,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // G5
                ],
            },
            
            ScaleType::AmaraTriple => HandpanScale {
                id: "AmaraTriple".to_string(),
                name: "Amara triple".to_string(),
                category: PanScaleCategory::MinorFolk,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // A5
                ],
            },
            
            ScaleType::AmaraKurdMini => HandpanScale {
                id: "AmaraKurdMini".to_string(),
                name: "Amara Kurd mini".to_string(),
                category: PanScaleCategory::MinorFolk,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Bottom, distance_relative_to_ding: 3, calculated_pitch: None },  // (C4)
                    NoteHandpan { note_index: 2, position: Bottom, distance_relative_to_ding: 5, calculated_pitch: None },  // (D4)
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 4, position: Bottom, distance_relative_to_ding: 8, calculated_pitch: None },  // (F4)
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 10, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 11, position: Bottom, distance_relative_to_ding: 20, calculated_pitch: None },  // (F5)
                    NoteHandpan { note_index: 12, position: Bottom, distance_relative_to_ding: 22, calculated_pitch: None },  // (G5)
                ],
            },
            
            ScaleType::AmaraKurdHijaz => HandpanScale {
                id: "AmaraKurdHijaz".to_string(),
                name: "Amara Kurd Hijaz".to_string(),
                category: PanScaleCategory::MinorFolk,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Bottom, distance_relative_to_ding: 3, calculated_pitch: None },  // (C4)
                    NoteHandpan { note_index: 2, position: Bottom, distance_relative_to_ding: 5, calculated_pitch: None },  // (D4)
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 4, position: Bottom, distance_relative_to_ding: 8, calculated_pitch: None },  // (F4)
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 6, position: Bottom, distance_relative_to_ding: 11, calculated_pitch: None },  // (G#4)
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 10, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 11, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 12, position: Bottom, distance_relative_to_ding: 20, calculated_pitch: None },  // (F5)
                    NoteHandpan { note_index: 13, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // G5
                    NoteHandpan { note_index: 14, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // A5
                    NoteHandpan { note_index: 15, position: Bottom, distance_relative_to_ding: 26, calculated_pitch: None },  // (B5)
                    NoteHandpan { note_index: 16, position: Bottom, distance_relative_to_ding: 27, calculated_pitch: None },  // (C6)
                ],
            },
            
            ScaleType::Anahata => HandpanScale {
                id: "Anahata".to_string(),
                name: "Anahata".to_string(),
                category: PanScaleCategory::Mystical,
                relative_ding: "C3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // G3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 9, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // C5
                ],
            },
            
            ScaleType::Arboreal => HandpanScale {
                id: "Arboreal".to_string(),
                name: "Arboreal".to_string(),
                category: PanScaleCategory::Experimental,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // F5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // G5
                ],
            },
            
            ScaleType::Ashakiran => HandpanScale {
                id: "Ashakiran".to_string(),
                name: "Ashakiran".to_string(),
                category: PanScaleCategory::Experimental,
                relative_ding: "C3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Bottom, distance_relative_to_ding: 2, calculated_pitch: None },  // (D3)
                    NoteHandpan { note_index: 2, position: Bottom, distance_relative_to_ding: 4, calculated_pitch: None },  // (E3)
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // F3
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // G3
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 9, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 10, position: Bottom, distance_relative_to_ding: 17, calculated_pitch: None },  // (F4)
                    NoteHandpan { note_index: 11, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 12, position: Top, distance_relative_to_ding: 21, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 13, position: Bottom, distance_relative_to_ding: 23, calculated_pitch: None },  // (B4)
                    NoteHandpan { note_index: 14, position: Bottom, distance_relative_to_ding: 24, calculated_pitch: None },  // (C5)
                    NoteHandpan { note_index: 15, position: Bottom, distance_relative_to_ding: 26, calculated_pitch: None },  // (D5)
                    NoteHandpan { note_index: 16, position: Bottom, distance_relative_to_ding: 28, calculated_pitch: None },  // (E5)
                ],
            },
            
            ScaleType::Avalon => HandpanScale {
                id: "Avalon".to_string(),
                name: "Avalon".to_string(),
                category: PanScaleCategory::Mystical,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 3, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                ],
            },
            
            ScaleType::Blues => HandpanScale {
                id: "Blues".to_string(),
                name: "Blues".to_string(),
                category: PanScaleCategory::Ethnic,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // F5
                ],
            },
            
            ScaleType::ChaoGuo => HandpanScale {
                id: "ChaoGuo".to_string(),
                name: "Chao Guo".to_string(),
                category: PanScaleCategory::PentatonicAsian,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 3, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // G5
                ],
            },
            
            ScaleType::DeepShello => HandpanScale {
                id: "DeepShello".to_string(),
                name: "Deep Shello".to_string(),
                category: PanScaleCategory::Ethnic,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 3, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::DaNaYo => HandpanScale {
                id: "DaNaYo".to_string(),
                name: "DaNaYo".to_string(),
                category: PanScaleCategory::Ethnic,
                relative_ding: "E3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 13, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // C5
                ],
            },
            
            ScaleType::Dorian => HandpanScale {
                id: "Dorian".to_string(),
                name: "Dorian".to_string(),
                category: PanScaleCategory::Modal,
                relative_ding: "D3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 9, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // A4
                ],
            },
            
            ScaleType::Equinox => HandpanScale {
                id: "Equinox".to_string(),
                name: "Equinox".to_string(),
                category: PanScaleCategory::MinorFolk,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 3, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::EquinoxBottomised => HandpanScale {
                id: "EquinoxBottomised".to_string(),
                name: "Equinox bottomised".to_string(),
                category: PanScaleCategory::MinorFolk,
                relative_ding: "F3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Bottom, distance_relative_to_ding: -4, calculated_pitch: None },  // (C#3)
                    NoteHandpan { note_index: 2, position: Bottom, distance_relative_to_ding: 2, calculated_pitch: None },  // (G3)
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 3, calculated_pitch: None },  // G#3
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // C#4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // D#4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 10, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // C5
                ],
            },
            
            ScaleType::EquinoxLowPygmyKurdDorianMini => HandpanScale {
                id: "EquinoxLowPygmyKurdDorianMini".to_string(),
                name: "Equinox Low Pygmy Kurd Dorian mini".to_string(),
                category: PanScaleCategory::Experimental,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Bottom, distance_relative_to_ding: 2, calculated_pitch: None },  // (B3)
                    NoteHandpan { note_index: 2, position: Bottom, distance_relative_to_ding: 3, calculated_pitch: None },  // (C4)
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 5, position: Bottom, distance_relative_to_ding: 9, calculated_pitch: None },  // (F#4)
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 10, position: Bottom, distance_relative_to_ding: 17, calculated_pitch: None },  // (D5)
                    NoteHandpan { note_index: 11, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::GoldenGate => HandpanScale {
                id: "GoldenGate".to_string(),
                name: "Golden Gate".to_string(),
                category: PanScaleCategory::Mystical,
                relative_ding: "C3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 4, calculated_pitch: None },  // E3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // G3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 18, calculated_pitch: None },  // F#4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 23, calculated_pitch: None },  // B4
                ],
            },
            
            ScaleType::HarmonicLora => HandpanScale {
                id: "HarmonicLora".to_string(),
                name: "Harmonic Lora".to_string(),
                category: PanScaleCategory::Experimental,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Bottom, distance_relative_to_ding: 2, calculated_pitch: None },  // (B3)
                    NoteHandpan { note_index: 2, position: Bottom, distance_relative_to_ding: 3, calculated_pitch: None },  // (C4)
                    NoteHandpan { note_index: 3, position: Bottom, distance_relative_to_ding: 5, calculated_pitch: None },  // (D4)
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 10, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 11, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 12, position: Bottom, distance_relative_to_ding: 23, calculated_pitch: None },  // (G#5)
                    NoteHandpan { note_index: 13, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // A5
                    NoteHandpan { note_index: 14, position: Bottom, distance_relative_to_ding: 26, calculated_pitch: None },  // (B5)
                    NoteHandpan { note_index: 15, position: Bottom, distance_relative_to_ding: 27, calculated_pitch: None },  // (C6)
                ],
            },
            
            ScaleType::HarmonicMinor => HandpanScale {
                id: "HarmonicMinor".to_string(),
                name: "Harmonic minor".to_string(),
                category: PanScaleCategory::Experimental,
                relative_ding: "A2".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Bottom, distance_relative_to_ding: 2, calculated_pitch: None },  // (B2)
                    NoteHandpan { note_index: 2, position: Bottom, distance_relative_to_ding: 3, calculated_pitch: None },  // (C3)
                    NoteHandpan { note_index: 3, position: Bottom, distance_relative_to_ding: 5, calculated_pitch: None },  // (D3)
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E3
                    NoteHandpan { note_index: 5, position: Bottom, distance_relative_to_ding: 8, calculated_pitch: None },  // (F3)
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // G#3
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 10, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 11, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 12, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 13, position: Top, distance_relative_to_ding: 23, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 14, position: Bottom, distance_relative_to_ding: 24, calculated_pitch: None },  // (A4)
                    NoteHandpan { note_index: 15, position: Bottom, distance_relative_to_ding: 26, calculated_pitch: None },  // (B4)
                    NoteHandpan { note_index: 16, position: Bottom, distance_relative_to_ding: 27, calculated_pitch: None },  // (C5)
                    NoteHandpan { note_index: 17, position: Bottom, distance_relative_to_ding: 29, calculated_pitch: None },  // (D5)
                ],
            },
            
            ScaleType::HighAvalon => HandpanScale {
                id: "HighAvalon".to_string(),
                name: "High Avalon".to_string(),
                category: PanScaleCategory::Mystical,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 3, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::HijazMercury => HandpanScale {
                id: "HijazMercury".to_string(),
                name: "Hijaz (Mercury)".to_string(),
                category: PanScaleCategory::Oriental,
                relative_ding: "B3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 6, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 9, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 13, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::Hijaz => HandpanScale {
                id: "Hijaz".to_string(),
                name: "Hijaz".to_string(),
                category: PanScaleCategory::Oriental,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::HijazKarMercury => HandpanScale {
                id: "HijazKarMercury".to_string(),
                name: "Hijaz Kar (Mercury)".to_string(),
                category: PanScaleCategory::Oriental,
                relative_ding: "B3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 6, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 9, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 13, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // D#5
                ],
            },
            
            ScaleType::HijazMajor => HandpanScale {
                id: "HijazMajor".to_string(),
                name: "Hijaz Major".to_string(),
                category: PanScaleCategory::Oriental,
                relative_ding: "E3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 13, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // B4
                ],
            },
            
            ScaleType::HijazTarznauyn => HandpanScale {
                id: "HijazTarznauyn".to_string(),
                name: "Hijaz Tarznauyn".to_string(),
                category: PanScaleCategory::Oriental,
                relative_ding: "D3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // A#3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 13, calculated_pitch: None },  // D#4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // F#4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 10, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // D5
                ],
            },
            
            ScaleType::Insen => HandpanScale {
                id: "Insen".to_string(),
                name: "Insen".to_string(),
                category: PanScaleCategory::PentatonicAsian,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // F5
                ],
            },
            
            ScaleType::IntegralMercury => HandpanScale {
                id: "IntegralMercury".to_string(),
                name: "Integral (Mercury)".to_string(),
                category: PanScaleCategory::Experimental,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // G5
                ],
            },
            
            ScaleType::IntegralSam => HandpanScale {
                id: "IntegralSam".to_string(),
                name: "Integral (Sam)".to_string(),
                category: PanScaleCategory::Experimental,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // F5
                ],
            },
            
            ScaleType::Jibuk => HandpanScale {
                id: "Jibuk".to_string(),
                name: "Jibuk".to_string(),
                category: PanScaleCategory::Ethnic,
                relative_ding: "D3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // G3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 9, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // G4
                ],
            },
            
            ScaleType::Kamaji => HandpanScale {
                id: "Kamaji".to_string(),
                name: "Kamaji".to_string(),
                category: PanScaleCategory::Oriental,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Bottom, distance_relative_to_ding: 2, calculated_pitch: None },  // (B3)
                    NoteHandpan { note_index: 2, position: Bottom, distance_relative_to_ding: 3, calculated_pitch: None },  // (C4)
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 6, position: Bottom, distance_relative_to_ding: 10, calculated_pitch: None },  // (G4)
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 10, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 11, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 12, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 13, position: Bottom, distance_relative_to_ding: 20, calculated_pitch: None },  // (F5)
                    NoteHandpan { note_index: 14, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // A5
                ],
            },
            
            ScaleType::Klezmara => HandpanScale {
                id: "Klezmara".to_string(),
                name: "Klezmara".to_string(),
                category: PanScaleCategory::Ethnic,
                relative_ding: "E3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 13, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::Kurd => HandpanScale {
                id: "Kurd".to_string(),
                name: "Kurd".to_string(),
                category: PanScaleCategory::MinorFolk,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Bottom, distance_relative_to_ding: 3, calculated_pitch: None },  // (C4)
                    NoteHandpan { note_index: 2, position: Bottom, distance_relative_to_ding: 5, calculated_pitch: None },  // (D4)
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 10, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 11, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // F5
                    NoteHandpan { note_index: 12, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // G5
                    NoteHandpan { note_index: 13, position: Bottom, distance_relative_to_ding: 24, calculated_pitch: None },  // (A5)
                    NoteHandpan { note_index: 14, position: Bottom, distance_relative_to_ding: 26, calculated_pitch: None },  // (B5)
                    NoteHandpan { note_index: 15, position: Bottom, distance_relative_to_ding: 27, calculated_pitch: None },  // (C6)
                    NoteHandpan { note_index: 16, position: Bottom, distance_relative_to_ding: 29, calculated_pitch: None },  // (D6)
                ],
            },
            
            ScaleType::KurdextendedGio => HandpanScale {
                id: "KurdextendedGio".to_string(),
                name: "Kurd extended (Gio)".to_string(),
                category: PanScaleCategory::MinorFolk,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Bottom, distance_relative_to_ding: -4, calculated_pitch: None },  // (F3)
                    NoteHandpan { note_index: 2, position: Bottom, distance_relative_to_ding: -2, calculated_pitch: None },  // (G3)
                    NoteHandpan { note_index: 3, position: Bottom, distance_relative_to_ding: 2, calculated_pitch: None },  // (B3)
                    NoteHandpan { note_index: 4, position: Bottom, distance_relative_to_ding: 3, calculated_pitch: None },  // (C4)
                    NoteHandpan { note_index: 5, position: Bottom, distance_relative_to_ding: 5, calculated_pitch: None },  // (D4)
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 10, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 11, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 12, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 13, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 14, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // F5
                    NoteHandpan { note_index: 15, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // G5
                    NoteHandpan { note_index: 16, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // A5
                ],
            },
            
            ScaleType::LaSirena => HandpanScale {
                id: "LaSirena".to_string(),
                name: "La Sirena".to_string(),
                category: PanScaleCategory::Ethnic,
                relative_ding: "D3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 3, calculated_pitch: None },  // F3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 9, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // A4
                ],
            },
            
            ScaleType::LowMystic => HandpanScale {
                id: "LowMystic".to_string(),
                name: "Low Mystic".to_string(),
                category: PanScaleCategory::Mystical,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 3, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::LowPygmy => HandpanScale {
                id: "LowPygmy".to_string(),
                name: "Low Pygmy".to_string(),
                category: PanScaleCategory::MinorFolk,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 2, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 3, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::LowPygmyOctave => HandpanScale {
                id: "LowPygmyOctave".to_string(),
                name: "Low Pygmy Octave".to_string(),
                category: PanScaleCategory::MinorFolk,
                relative_ding: "A2".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 26, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 27, calculated_pitch: None },  // C5
                ],
            },
            
            ScaleType::Lydian => HandpanScale {
                id: "Lydian".to_string(),
                name: "Lydian".to_string(),
                category: PanScaleCategory::Modal,
                relative_ding: "F3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 18, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 23, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::MagicVoyage => HandpanScale {
                id: "MagicVoyage".to_string(),
                name: "Magic Voyage".to_string(),
                category: PanScaleCategory::Mystical,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 3, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // G5
                ],
            },
            
            ScaleType::Major => HandpanScale {
                id: "Major".to_string(),
                name: "Major".to_string(),
                category: PanScaleCategory::Modal,
                relative_ding: "C3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // F3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // G3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 9, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // G4
                ],
            },
            
            ScaleType::MelogSelisir => HandpanScale {
                id: "MelogSelisir".to_string(),
                name: "Melog Selisir".to_string(),
                category: PanScaleCategory::PentatonicAsian,
                relative_ding: "C3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 4, calculated_pitch: None },  // E3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // F3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // G3
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // G4
                ],
            },
            
            ScaleType::Minor => HandpanScale {
                id: "Minor".to_string(),
                name: "Minor".to_string(),
                category: PanScaleCategory::Modal,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::Mixolydian4 => HandpanScale {
                id: "Mixolydian4".to_string(),
                name: "Mixolydian (-4)".to_string(),
                category: PanScaleCategory::Modal,
                relative_ding: "G3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 9, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 21, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::Mixolydian7m => HandpanScale {
                id: "Mixolydian7m".to_string(),
                name: "Mixolydian (-7m)".to_string(),
                category: PanScaleCategory::Modal,
                relative_ding: "G3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 9, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 21, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::MysticHagane => HandpanScale {
                id: "MysticHagane".to_string(),
                name: "Mystic (Hagane)".to_string(),
                category: PanScaleCategory::Mystical,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // G5
                ],
            },
            
            ScaleType::MysticSam => HandpanScale {
                id: "MysticSam".to_string(),
                name: "Mystic (Sam)".to_string(),
                category: PanScaleCategory::Mystical,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // G5
                ],
            },
            
            ScaleType::Nazar => HandpanScale {
                id: "Nazar".to_string(),
                name: "Nazar".to_string(),
                category: PanScaleCategory::Experimental,
                relative_ding: "B2".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Bottom, distance_relative_to_ding: 1, calculated_pitch: None },  // (C3)
                    NoteHandpan { note_index: 2, position: Bottom, distance_relative_to_ding: 5, calculated_pitch: None },  // (E3)
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // F#3
                    NoteHandpan { note_index: 4, position: Bottom, distance_relative_to_ding: 8, calculated_pitch: None },  // (G3)
                    NoteHandpan { note_index: 5, position: Bottom, distance_relative_to_ding: 10, calculated_pitch: None },  // (A3)
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 13, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 8, position: Bottom, distance_relative_to_ding: 15, calculated_pitch: None },  // (D4)
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // D#4
                    NoteHandpan { note_index: 10, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 11, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // F#4
                    NoteHandpan { note_index: 12, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 13, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 14, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 15, position: Bottom, distance_relative_to_ding: 25, calculated_pitch: None },  // (C5)
                    NoteHandpan { note_index: 16, position: Bottom, distance_relative_to_ding: 28, calculated_pitch: None },  // (D#5)
                    NoteHandpan { note_index: 17, position: Bottom, distance_relative_to_ding: 29, calculated_pitch: None },  // (E5)
                ],
            },
            
            ScaleType::Onoleo => HandpanScale {
                id: "Onoleo".to_string(),
                name: "Onoleo".to_string(),
                category: PanScaleCategory::Mystical,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // C#5
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // F5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // A5
                ],
            },
            
            ScaleType::Oxalis => HandpanScale {
                id: "Oxalis".to_string(),
                name: "Oxalis".to_string(),
                category: PanScaleCategory::Mystical,
                relative_ding: "C3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 4, calculated_pitch: None },  // E3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // F3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // G3
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 9, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // G4
                ],
            },
            
            ScaleType::Paradisev1 => HandpanScale {
                id: "Paradisev1".to_string(),
                name: "Paradise (v1)".to_string(),
                category: PanScaleCategory::Ethnic,
                relative_ding: "C3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 4, calculated_pitch: None },  // E3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // G3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // G4
                ],
            },
            
            ScaleType::Paradisev2 => HandpanScale {
                id: "Paradisev2".to_string(),
                name: "Paradise (v2)".to_string(),
                category: PanScaleCategory::Ethnic,
                relative_ding: "C3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 4, calculated_pitch: None },  // E3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // F3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // G3
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // G4
                ],
            },
            
            ScaleType::PentatonicMajor => HandpanScale {
                id: "PentatonicMajor".to_string(),
                name: "Pentatonic Major".to_string(),
                category: PanScaleCategory::Modal,
                relative_ding: "C3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // F3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // G3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 9, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 21, calculated_pitch: None },  // A4
                ],
            },
            
            ScaleType::Pygmy => HandpanScale {
                id: "Pygmy".to_string(),
                name: "Pygmy".to_string(),
                category: PanScaleCategory::MinorFolk,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // F5
                ],
            },
            
            ScaleType::RagaDesh => HandpanScale {
                id: "RagaDesh".to_string(),
                name: "Raga Desh".to_string(),
                category: PanScaleCategory::Oriental,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // C#5
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 22, calculated_pitch: None },  // G5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // A5
                ],
            },
            
            ScaleType::RomanianHijaz => HandpanScale {
                id: "RomanianHijaz".to_string(),
                name: "Romanian Hijaz".to_string(),
                category: PanScaleCategory::Oriental,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::RomanianMineurHarmonique => HandpanScale {
                id: "RomanianMineurHarmonique".to_string(),
                name: "Romanian mineur harmonique".to_string(),
                category: PanScaleCategory::Oriental,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 3, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                ],
            },
            
            ScaleType::Sabye => HandpanScale {
                id: "Sabye".to_string(),
                name: "Sabye".to_string(),
                category: PanScaleCategory::Ethnic,
                relative_ding: "C3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Bottom, distance_relative_to_ding: 2, calculated_pitch: None },  // (D3)
                    NoteHandpan { note_index: 2, position: Bottom, distance_relative_to_ding: 4, calculated_pitch: None },  // (E3)
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // F3
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // G3
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 9, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 10, position: Bottom, distance_relative_to_ding: 17, calculated_pitch: None },  // (F4)
                    NoteHandpan { note_index: 11, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 12, position: Bottom, distance_relative_to_ding: 21, calculated_pitch: None },  // (A4)
                    NoteHandpan { note_index: 13, position: Bottom, distance_relative_to_ding: 23, calculated_pitch: None },  // (B4)
                    NoteHandpan { note_index: 14, position: Bottom, distance_relative_to_ding: 24, calculated_pitch: None },  // (C5)
                    NoteHandpan { note_index: 15, position: Bottom, distance_relative_to_ding: 26, calculated_pitch: None },  // (D5)
                ],
            },
            
            ScaleType::Saladin => HandpanScale {
                id: "Saladin".to_string(),
                name: "Saladin".to_string(),
                category: PanScaleCategory::Oriental,
                relative_ding: "E3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 5, calculated_pitch: None },  // A3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 13, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // B4
                ],
            },
            
            ScaleType::ShelloPango => HandpanScale {
                id: "ShelloPango".to_string(),
                name: "ShelloPango".to_string(),
                category: PanScaleCategory::Ethnic,
                relative_ding: "C3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // G3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 10, calculated_pitch: None },  // A#3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // D#4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 20, calculated_pitch: None },  // G#4
                    NoteHandpan { note_index: 9, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // C5
                ],
            },
            
            ScaleType::Ursaminor => HandpanScale {
                id: "Ursaminor".to_string(),
                name: "Ursa minor".to_string(),
                category: PanScaleCategory::MinorFolk,
                relative_ding: "A3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 8, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // A4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // B4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 15, calculated_pitch: None },  // C5
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // D5
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // E5
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // A5
                ],
            },
            
            ScaleType::YshaSavita => HandpanScale {
                id: "YshaSavita".to_string(),
                name: "Ysha Savita".to_string(),
                category: PanScaleCategory::PentatonicAsian,
                relative_ding: "C3".to_string(), // not used, reference only to be a helper to understand the scale
                notes: vec![
                    NoteHandpan { note_index: 1, position: Top, distance_relative_to_ding: 7, calculated_pitch: None },  // G3
                    NoteHandpan { note_index: 2, position: Top, distance_relative_to_ding: 11, calculated_pitch: None },  // B3
                    NoteHandpan { note_index: 3, position: Top, distance_relative_to_ding: 12, calculated_pitch: None },  // C4
                    NoteHandpan { note_index: 4, position: Top, distance_relative_to_ding: 14, calculated_pitch: None },  // D4
                    NoteHandpan { note_index: 5, position: Top, distance_relative_to_ding: 16, calculated_pitch: None },  // E4
                    NoteHandpan { note_index: 6, position: Top, distance_relative_to_ding: 17, calculated_pitch: None },  // F4
                    NoteHandpan { note_index: 7, position: Top, distance_relative_to_ding: 19, calculated_pitch: None },  // G4
                    NoteHandpan { note_index: 8, position: Top, distance_relative_to_ding: 24, calculated_pitch: None },  // C5
                ],
            },
            
            
        }
    }

    // Helper method to get the name as a string
    pub fn name(&self) -> String {
        self.get_scale().name
    }
    
    // Parse a string to create a ScaleType
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        // Use the automatically generated EnumString implementation
        match s.parse::<ScaleType>() {
            Ok(scale_type) => Ok(scale_type),
            Err(_) => Err("Invalid scale type")
        }
    }
}

// Get all handpan scales
pub fn get_all_scales() -> Vec<HandpanScale> {
    // Return scales for all scale types using dynamic iteration
    ScaleType::iter().map(|scale_type| scale_type.get_scale()).collect()
}

// Get notes for a specific scale and ding
pub fn get_notes_for_scale_and_ding(scale_type: &ScaleType, ding: &str) -> Option<Vec<NoteHandpan>> {
    let scale = scale_type.get_scale();
    let ding_midi = note_to_midi(ding)?;
    
    let notes = scale.notes.iter().map(|note| {
        let mut note = note.clone();
        note.calculated_pitch = Some(ding_midi + note.distance_relative_to_ding);
        note
    }).collect();
    
    Some(notes)
}
