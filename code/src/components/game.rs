use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

use super::tile::{Tile, TileAuthor};

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub game_key: i32,
    pub start_tiles: Vec<Tile>,
}


// game.rs
#[server(GetGame)]
pub async fn get_game() -> Result<Game, ServerFnError> {
    let word =
        [
            Tile {
                letter: String::from("c"),
                author: TileAuthor::Computer,
            },
            Tile {
                letter: String::from("a"),
                author: TileAuthor::Computer,
            },
        ];

    let game_state = Game {
        game_key: 1,
        start_tiles: word.to_vec(),
    };

    Ok(game_state)
}

