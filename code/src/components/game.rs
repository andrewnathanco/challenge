use std::string;

use super::{keyboard::get_all_letters, tile::*};
use csv::ReaderBuilder;
use leptos::{error::Result, logging::log, server, ServerFnError, *};
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub game_key: i32,
    pub current_tiles: Vec<Tile>,
    pub available_letters: Vec<String>,
    pub computer_word: String,
}

impl Game {
    fn new() -> Self {
        let tiles = [
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
            current_tiles: tiles.to_vec(),
            computer_word: "computer".to_string(),
            available_letters: get_all_letters(),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            game_key: 1,
            current_tiles: [].to_vec(),
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
    let (game, set_game) = create_signal(Game::new());
    let get_words = create_local_resource(||{}, |_| async {get_file().await});

    create_effect(move |_| {
        get_words.and_then(|words| {
            set_game.update(|g| {
                let mut rng = rand::thread_rng();
                let random_index = rng.gen_range(0..words.len() - 1);

                let starting_word = words[random_index].to_string();
                let tiles = [
                    Tile{
                        letter: starting_word.chars().nth(0).unwrap_or_default().to_string(),
                        author: TileAuthor::Computer,
                    },
                    Tile{
                        letter: starting_word.chars().nth(1).unwrap_or_default().to_string(),
                        author: TileAuthor::Computer,
                    },
                    Tile{
                        letter: starting_word.chars().nth(2).unwrap_or_default().to_string(),
                        author: TileAuthor::Computer,
                    },
                ];

                g.current_tiles = tiles.to_vec();
                g.computer_word = starting_word;
            });
        })
    });


    (game, set_game)
}

async fn get_file() -> Result<Vec<String>> {
    // make the request
    let response = reqwasm::http::Request::get(&format!(
        "/valid_words.csv",
    ))
    .send()
    .await?;


    if response.status() == 200 {
        let csv_data = response.text().await?;
        let mut csv_reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());

        let available_words:Vec<String> = csv_reader
        .records()
        .into_iter()
        .filter_map(|data| data.ok())
        .map(|data| {
            data.get(0).unwrap_or_default().to_string()
        })
        .collect();
        
        return Ok(available_words)
    }

    Ok(["test".to_string()].to_vec())
}





































































































































