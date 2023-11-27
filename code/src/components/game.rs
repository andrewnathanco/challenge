use super::{keyboard::get_all_letters, tile::*, word::get_available_letters};
use chrono::prelude::*;
use leptos::{*, logging::log};
use serde::{Deserialize, Serialize};

use crate::components::word::get_word;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub game_key: i64,
    pub starting_word: String,
    pub starting_tiles: Vec<Tile>,
    pub available_letters: Vec<String>,
    pub computer_word: String,
}

impl Game {
    fn new() -> Self {
        let tiles = [
            Tile {
                letter: 'c',
                author: TileAuthor::Computer,
            },
            Tile {
                letter: 'o',
                author: TileAuthor::Computer,
            },
        ];

        Game {
            game_key: 1,
            starting_word: "computer".to_string(),
            starting_tiles: tiles.to_vec(),
            computer_word: "computer".to_string(),
            available_letters: get_all_letters(),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            game_key: 1,
            starting_word: "".to_string(),
            starting_tiles: [].to_vec(),
            computer_word: "".to_string(),
            available_letters: get_all_letters(),
        }
    }
}


pub fn use_game(
) -> (ReadSignal<Game>, WriteSignal<Game>) {
    let (game, set_game) = create_signal(Game::new());

    // get game key from the date
    let game_key = get_game_key();

    // get word
    let get_word = create_local_resource(
        move || {}, |_|async {get_word().await}
    );

    let tiles = [
        Tile{
            letter: "attract".chars().nth(0).unwrap_or_default(),
            author: TileAuthor::Computer,
        },
        Tile{
            letter: "attract".chars().nth(1).unwrap_or_default(),
            author: TileAuthor::Computer,
        },
        Tile{
            letter: "attract".chars().nth(2).unwrap_or_default(),
            author: TileAuthor::Computer,
        },
    ];

    create_effect(move |_| {


        get_word.and_then(|word| {
            set_game.update(|g| {
                let tiles = [
                    Tile{
                        letter: word.chars().nth(0).unwrap_or_default(),
                        author: TileAuthor::Computer,
                    },
                    Tile{
                        letter: word.chars().nth(1).unwrap_or_default(),
                        author: TileAuthor::Computer,
                    },
                    Tile{
                        letter: word.chars().nth(2).unwrap_or_default(),
                        author: TileAuthor::Computer,
                    },
                ];

                g.starting_tiles = tiles.to_vec();
                g.starting_word = word.to_string();
                g.computer_word = word.to_string();
                g.game_key = game_key
            });
        })
    });


    (game, set_game)
}

fn get_game_key() -> i64 {
    let now= Utc::now();
    let specific_date= Utc.with_ymd_and_hms(2023, 11, 26, 5, 0, 0).unwrap();
    let duration = now.signed_duration_since(specific_date);

    duration.num_days()
}

#[component]
pub fn GameHeader() -> impl IntoView {
    let (game, _) = use_game();

    view! { <div class="text-4xl">"Challenge #" {move || game.get().game_key}</div> }
}





