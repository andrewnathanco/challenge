use super::{tile::*, word::get_available_letters, session::Session};
use chrono::prelude::*;
use leptos::{*, error::Result};
use leptos_use::storage::use_local_storage;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::components::word::get_word;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub game_key: i64,

    // starting values
    pub starting_word: String,
    pub starting_tiles: Vec<Tile>,
    pub starting_letters: Vec<char>,

    // current values
    pub selected_letter: char,
    pub available_letters: Vec<char>,
    pub current_tiles: Vec<Tile>,

    // session values
    pub sessions: Vec<Session>
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
            sessions: Vec::new(),
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
            sessions: Vec::new(),
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
        current_tiles: tiles.to_vec(),
        sessions: Vec::new()
    })
}

fn get_game_key() -> i64 {
    let now= Utc::now();
    let specific_date= Utc.with_ymd_and_hms(2023, 11, 26, 5, 0, 0).unwrap();
    let duration = now.signed_duration_since(specific_date);

    duration.num_days()
}

pub fn get_countdown_till_next_game() -> String {
    // Get the current local time
    let now = Utc::now();

    // midnight EST in UTC
    let midnight = Utc.with_ymd_and_hms(now.year(), now.month(), now.day(), 5, 0, 0).unwrap();

    // Calculate the duration until midnight EST
    let duration_until_midnight = midnight.signed_duration_since(now);

    format!(
        "{:02}:{:02}:{:02}", 24 + duration_until_midnight.num_hours(), 60 + (duration_until_midnight.num_minutes() % 60) as i32, 60 + (duration_until_midnight.num_seconds() % 60) as i32
    )
}

#[component]
pub fn GameHeader() -> impl IntoView {
    let (game, _) = use_game();
    let version = option_env!("CARGO_VERSION").unwrap_or("0.1.0");
    view! {
        <div class="flex space-x-2 justify-center items-center">
            <div class="text-4xl">"Challenge #" {move || game.get().game_key}</div>
            <div
                id="game-version"
                class="font-semibold w-min h-min text-gray-600 text-xs border-2 px-1 border-gray-600 rounded-lg"
            >
                {move || version}
            </div>
        </div>
    }
}






































































