use serde::{Deserialize, Serialize};

use super::tile::Tile;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub game_key: i32,
    pub start_tiles: Vec<Tile>,
}

