use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(into = "u8", from = "u8")]
pub enum Difficulty {
    Novice = 1,
    Skilled = 2,
    Advanced = 3,
}

impl Difficulty {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Difficulty::Novice),
            2 => Some(Difficulty::Skilled),
            3 => Some(Difficulty::Advanced),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Difficulty::Novice => 1,
            Difficulty::Skilled => 2,
            Difficulty::Advanced => 3,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Difficulty::Novice => "Novice".to_string(),
            Difficulty::Skilled => "Skilled".to_string(),
            Difficulty::Advanced => "Advanced".to_string(),
        }
    }
    
    /// Returns all difficulty variants
    pub fn all() -> Vec<Self> {
        vec![
            Difficulty::Novice,
            Difficulty::Skilled,
            Difficulty::Advanced,
        ]
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::Skilled
    }
}

// Implement From<Difficulty> for u8 to allow serialization as integer
impl From<Difficulty> for u8 {
    fn from(difficulty: Difficulty) -> Self {
        difficulty.to_u8()
    }
}

// Implement From<u8> for Difficulty to allow deserialization from integer
impl From<u8> for Difficulty {
    fn from(value: u8) -> Self {
        Difficulty::from_u8(value).unwrap_or_default()
    }
}
