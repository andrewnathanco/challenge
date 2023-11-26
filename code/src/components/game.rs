use super::{keyboard::get_all_letters, tile::*};
use csv::ReaderBuilder;
use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub game_key: i32,
    pub starting_word: String,
    pub starting_tiles: Vec<Tile>,
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

#[component]
pub fn GameHeader() -> impl IntoView {
    let (game, _) = use_game();

    view! { <div class="text-4xl">"Challenge #" {move || game.get().game_key}</div> }
}

pub fn use_game() -> (ReadSignal<Game>, WriteSignal<Game>) {
    let (game, set_game) = create_signal(Game::new());
    let get_word = create_local_resource(||{}, |_| async {get_random_word().await});

    create_effect(move |_| {
        get_word.and_then(|word| {
            set_game.update(|g| {
                let tiles = [
                    Tile{
                        letter: word.chars().nth(0).unwrap_or_default().to_string(),
                        author: TileAuthor::Computer,
                    },
                    Tile{
                        letter: word.chars().nth(1).unwrap_or_default().to_string(),
                        author: TileAuthor::Computer,
                    },
                    Tile{
                        letter: word.chars().nth(2).unwrap_or_default().to_string(),
                        author: TileAuthor::Computer,
                    },
                ];

                g.starting_tiles = tiles.to_vec();
                g.starting_word = word.to_string();
                g.computer_word = word.to_string();
            });
        })
    });


    (game, set_game)
}

async fn get_words() -> Result<Vec<String>> {
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

    Ok(["computer".to_string()].to_vec(

    ))
}




async fn get_random_word() -> Result<String> { 
    let avail_words = get_words().await?;
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..avail_words.len() - 1);

    Ok(avail_words[random_index].to_string())
}
























