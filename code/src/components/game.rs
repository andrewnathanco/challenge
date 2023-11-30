use super::{keyboard::get_all_letters, tile::*, word::get_available_letters};
use chrono::prelude::*;
use leptos::{*,error::Result, logging::log};
use leptos_use::storage::use_local_storage;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::components::word::get_word;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub game_key: i64,
    pub starting_word: String,
    pub starting_tiles: Vec<Tile>,
    pub starting_letters: Vec<char>,
    pub selected_letter: char,
    pub available_letters: Vec<char>,
    pub current_tiles: Vec<Tile>
}

impl Game {
    pub fn new() -> Self {
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
            selected_letter: '_',
            game_key: 0,
            starting_word: "computer".to_string(),
            starting_tiles: tiles.to_vec(),
            available_letters: Vec::new(),
            starting_letters: Vec::new(),
            current_tiles: tiles.to_vec(),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            selected_letter: '_',
            game_key: 0,
            starting_word: "".to_string(),
            starting_tiles: [].to_vec(),
            starting_letters: Vec::new(),
            available_letters: Vec::new(),
            current_tiles: [].to_vec(),
        }
    }
}


impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.game_key == other.game_key
    }
}

pub fn use_game(
) -> (Signal<Game>, WriteSignal<Game>) {
    let (game, set_game, _) = use_local_storage("game", Game::new());

    // get word
    let get_new_game = create_local_resource(
        move || {}, |_|async {get_new_game().await}
    );

    create_effect(move |_| {
        get_new_game.and_then(|new_game| {
            if new_game.game_key != game().game_key {
                set_game.set(new_game.clone());
            }
        })
    });


    (game, set_game)
}

pub async fn get_new_game() -> Result<Game> {
    let word = get_word(get_game_key()).await.unwrap_or_default();
    let mut rng = rand::thread_rng();
    let num_chars = rng.gen_range(0..3);
    let mut tiles: Vec<Tile> = Vec::new();


    let chars = word.chars().take(num_chars);
    for char in chars {
        tiles.push(Tile{
            letter: char,
            author: TileAuthor::Computer,
        })
    }

    let tiles = [
        Tile{
            letter: word.chars().nth(0).unwrap_or_default(),
            author: TileAuthor::Computer,
        },
        Tile{
            letter: word.chars().nth(1).unwrap_or_default(),
            author: TileAuthor::Computer,
        },
    ];

    let avail_letters = get_available_letters(tiles.to_vec()).await.unwrap_or_default();
    Ok(Game { 
        selected_letter: '_',
        game_key: get_game_key(), 
        starting_word: word, 
        starting_tiles: tiles.to_vec(), 
        starting_letters: avail_letters.clone(), 
        available_letters: avail_letters, 
        current_tiles: tiles.to_vec() 
    })
}

fn get_game_key() -> i64 {
    let now= Utc::now();
    let specific_date= Utc.with_ymd_and_hms(2023, 11, 28, 5, 0, 0).unwrap();
    let duration = now.signed_duration_since(specific_date);

    duration.num_days()
}

#[component]
pub fn GameHeader() -> impl IntoView {
    let (game, _) = use_game();

    view! { <div class="text-4xl">"Challenge #" {move || game.get().game_key}</div> }
}
















































