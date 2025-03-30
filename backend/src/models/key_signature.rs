use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum KeySignature {
    Cmaj, Gmaj, Dmaj, Amaj, Emaj, Bmaj, FsMaj, CsMaj,
    Fmaj, Bbmaj, Ebmaj, Abmaj, Dbmaj, Gbmaj, Cbmaj,
    Amin, Emin, Bmin, FsMin, CsMin, GsMin, DsMin, AsMin, 
    Dmin, Gmin, Cmin, Fmin, Bbmin, Ebmin, Abmin,
}

impl KeySignature {
    // Convert KeySignature to string representation
    pub fn to_string(&self) -> String {
        match self {
            KeySignature::Cmaj => "Cmaj".to_string(),
            KeySignature::Gmaj => "Gmaj".to_string(),
            KeySignature::Dmaj => "Dmaj".to_string(),
            KeySignature::Amaj => "Amaj".to_string(),
            KeySignature::Emaj => "Emaj".to_string(),
            KeySignature::Bmaj => "Bmaj".to_string(),
            KeySignature::FsMaj => "F#maj".to_string(),
            KeySignature::CsMaj => "C#maj".to_string(),
            KeySignature::Fmaj => "Fmaj".to_string(),
            KeySignature::Bbmaj => "Bbmaj".to_string(),
            KeySignature::Ebmaj => "Ebmaj".to_string(),
            KeySignature::Abmaj => "Abmaj".to_string(),
            KeySignature::Dbmaj => "Dbmaj".to_string(),
            KeySignature::Gbmaj => "Gbmaj".to_string(),
            KeySignature::Cbmaj => "Cbmaj".to_string(),
            KeySignature::Amin => "Amin".to_string(),
            KeySignature::Emin => "Emin".to_string(),
            KeySignature::Bmin => "Bmin".to_string(),
            KeySignature::FsMin => "F#min".to_string(),
            KeySignature::CsMin => "C#min".to_string(),
            KeySignature::GsMin => "G#min".to_string(),
            KeySignature::DsMin => "D#min".to_string(),
            KeySignature::AsMin => "A#min".to_string(),
            KeySignature::Dmin => "Dmin".to_string(),
            KeySignature::Gmin => "Gmin".to_string(),
            KeySignature::Cmin => "Cmin".to_string(),
            KeySignature::Fmin => "Fmin".to_string(),
            KeySignature::Bbmin => "Bbmin".to_string(),
            KeySignature::Ebmin => "Ebmin".to_string(),
            KeySignature::Abmin => "Abmin".to_string(),
        }
    }
    
    /// Returns a complexity score for the key signature based on the number of sharps/flats
    /// Score ranges from 0 (C major/A minor) to 7 (C# major/Ab minor)
    pub fn complexity_score(&self) -> u8 {
        match self {
            // Majors (0 to 7 sharps/flats)
            KeySignature::Cmaj => 0,
            KeySignature::Gmaj | KeySignature::Fmaj => 1,
            KeySignature::Dmaj | KeySignature::Bbmaj => 2,
            KeySignature::Amaj | KeySignature::Ebmaj => 3,
            KeySignature::Emaj | KeySignature::Abmaj => 4,
            KeySignature::Bmaj | KeySignature::Dbmaj => 5,
            KeySignature::FsMaj | KeySignature::Gbmaj => 6,
            KeySignature::CsMaj | KeySignature::Cbmaj => 7,

            // Minors (parallel to the majors)
            KeySignature::Amin => 0,
            KeySignature::Emin | KeySignature::Dmin => 1,
            KeySignature::Bmin | KeySignature::Gmin => 2,
            KeySignature::FsMin | KeySignature::Cmin => 3,
            KeySignature::CsMin | KeySignature::Fmin => 4,
            KeySignature::GsMin | KeySignature::Bbmin => 5,
            KeySignature::DsMin | KeySignature::Ebmin => 6,
            KeySignature::AsMin | KeySignature::Abmin => 7,
        }
    }

    // Parse a string to KeySignature
    pub fn from_string(key_str: &str) -> Option<Self> {
        match key_str {
            "Cmaj" => Some(KeySignature::Cmaj),
            "Gmaj" => Some(KeySignature::Gmaj),
            "Dmaj" => Some(KeySignature::Dmaj),
            "Amaj" => Some(KeySignature::Amaj),
            "Emaj" => Some(KeySignature::Emaj),
            "Bmaj" => Some(KeySignature::Bmaj),
            "F#maj" | "FsMaj" => Some(KeySignature::FsMaj),
            "C#maj" | "CsMaj" => Some(KeySignature::CsMaj),
            "Fmaj" => Some(KeySignature::Fmaj),
            "Bbmaj" => Some(KeySignature::Bbmaj),
            "Ebmaj" => Some(KeySignature::Ebmaj),
            "Abmaj" => Some(KeySignature::Abmaj),
            "Dbmaj" => Some(KeySignature::Dbmaj),
            "Gbmaj" => Some(KeySignature::Gbmaj),
            "Cbmaj" => Some(KeySignature::Cbmaj),
            "Amin" => Some(KeySignature::Amin),
            "Emin" => Some(KeySignature::Emin),
            "Bmin" => Some(KeySignature::Bmin),
            "F#min" | "FsMin" => Some(KeySignature::FsMin),
            "C#min" | "CsMin" => Some(KeySignature::CsMin),
            "G#min" | "GsMin" => Some(KeySignature::GsMin),
            "D#min" | "DsMin" => Some(KeySignature::DsMin),
            "A#min" | "AsMin" => Some(KeySignature::AsMin),
            "Dmin" => Some(KeySignature::Dmin),
            "Gmin" => Some(KeySignature::Gmin),
            "Cmin" => Some(KeySignature::Cmin),
            "Fmin" => Some(KeySignature::Fmin),
            "Bbmin" => Some(KeySignature::Bbmin),
            "Ebmin" => Some(KeySignature::Ebmin),
            "Abmin" => Some(KeySignature::Abmin),
            _ => None,
        }
    }
}
