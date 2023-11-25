use serde::{Deserialize, Serialize};

pub const TILE_COMP: &str =
    "w-16 h-20 bg-gray-300 rounded-lg  flex justify-center items-center border-2 border-gray-400";
pub const TILE_YOU: &str =
    "w-16 h-20 bg-green-300 border-2 border-green-500 rounded-lg flex justify-center items-center";
pub const TILE_EMPTY: &str =
    "w-16 h-20 border-2 border-gray-300 rounded-lg flex justify-center items-center text-gray-300";
pub const TILE_OPTION: &str =
    "w-16 h-20 border-2 border-green-600 rounded-lg flex justify-center items-center text-green-600";

#[derive(Serialize, Deserialize, Clone)]
pub enum TileAuthor {
    Computer,
    User,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tile {
    pub letter: String,
    pub author: TileAuthor,
}

