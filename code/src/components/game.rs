use super::{keyboard::get_all_letters, tile::*};
use leptos::{server, ServerFnError, *};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub game_key: i32,
    pub current_word: Vec<Tile>,
    pub available_letters: Vec<String>,
    pub computer_word: String,
}

impl Game {
    fn new() -> Self {
        let word = [
            Tile {
                letter: String::from("c"),
                author: TileAuthor::Computer,
            },
            Tile {
                letter: String::from("o"),
                author: TileAuthor::Computer,
            },
        ];

        Game {
            game_key: 1,
            current_word: word.to_vec(),
            computer_word: "computer".to_string(),
            available_letters: get_all_letters(),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            game_key: 1,
            current_word: [].to_vec(),
            computer_word: "".to_string(),
            available_letters: get_all_letters(),
        }
    }
}

#[component]
pub fn GameHeader() -> impl IntoView {
    let (game, _) = use_game();

    view! { <div class="text-4xl">"Challenge #" {move || game.get().game_key}</div> }
}

pub fn use_game() -> (ReadSignal<Game>, WriteSignal<Game>) {
    create_signal(Game::new())
}

