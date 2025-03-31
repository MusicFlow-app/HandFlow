use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(into = "u8", from = "u8")]
pub enum Category {
    Scale = 1,
    Song = 2,
    Exercise = 3,
}

impl Category {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Category::Scale),
            2 => Some(Category::Song),
            3 => Some(Category::Exercise),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Category::Scale => 1,
            Category::Song => 2,
            Category::Exercise => 3,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Category::Scale => "Scale".to_string(),
            Category::Song => "Song".to_string(),
            Category::Exercise => "Exercise".to_string(),
        }
    }
    
    /// Returns all category variants
    pub fn all() -> Vec<Self> {
        vec![
            Category::Scale,
            Category::Song,
            Category::Exercise,
        ]
    }
}

impl Default for Category {
    fn default() -> Self {
        Category::Song
    }
}

// Implement From<Category> for u8 to allow serialization as integer
impl From<Category> for u8 {
    fn from(category: Category) -> Self {
        category.to_u8()
    }
}

// Implement From<u8> for Category to allow deserialization from integer
impl From<u8> for Category {
    fn from(value: u8) -> Self {
        Category::from_u8(value).unwrap_or_default()
    }
}
