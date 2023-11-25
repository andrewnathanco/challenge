use serde::{Deserialize, Serialize};

use super::tile::Tile;

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub tiles: Vec<Tile>,
    pub selected_letter: String,
}

