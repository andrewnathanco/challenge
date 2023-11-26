use leptos::{server, ServerFnError, *};
use serde::{Deserialize, Serialize};
use super::{tile::*, keyboard::get_all_letters};

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub game_key: i32,
    pub current_word: Vec<Tile>,
    pub available_letters: Vec<String>,
    pub computer_word: String,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            game_key: 0,
            current_word: [].to_vec(),
            computer_word: "".to_string(),
            available_letters: get_all_letters(),
        }
    }
}

// game.rs
#[server]
pub async fn get_game(
    new_word: Vec<Tile>
) -> Result<Game, ServerFnError> {
    let word =
        [
            Tile {
                letter: String::from("c"),
                author: TileAuthor::Computer,
            },
            Tile {
                letter: String::from("o"),
                author: TileAuthor::Computer,
            },
        ];

    let game_state = Game {
        game_key: 1,
        current_word: word.to_vec(),
        computer_word: "computer".to_string(),
        available_letters: get_all_letters(),
    };

    Ok(game_state)
}

#[component]
pub fn GameHeader() -> impl IntoView {
    let game =
        create_resource(|| (), |_| async { get_game([].to_vec()).await });
    let game_view = move || {
        game.and_then(|game|  
            {
                view! { <div class="text-4xl">"Challenge: #" {game.game_key}</div> }
            }).collect_view()
    };

    view! {
        <Suspense fallback=move || view! { <p>"Loading game..."</p> }>
            <ul>{game_view}</ul>
        </Suspense>
    }
}


pub fn use_game() -> (ReadSignal<Game>, WriteSignal<Game>)  {
    let (game, set_game) = create_signal(Game::default());
    let game_res =
        create_resource(|| (), |_| async { get_game([].to_vec()).await });

    // take that response and get back our current game for the session
    create_effect(move |_| {
        game_res.and_then(|g| set_game(g.clone()));
    });

    (game, set_game)
}















































