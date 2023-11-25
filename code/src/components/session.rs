use serde::{Deserialize, Serialize};

use super::tile::Tile;

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub tiles: Vec<Tile>,
    pub selected_letter: String,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            tiles: [].to_vec(),
            selected_letter: "_".to_string(),
        }
    }
}

