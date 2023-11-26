use leptos::*;
use serde::{Deserialize, Serialize};

use crate::components::{game::*, keyboard::Keyboard, tile::*};

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub tiles: Vec<Tile>,
    pub selected_letter: String,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            tiles: [].to_vec(),
            selected_letter: "_".to_string(),
        }
    }
}

pub fn use_session() -> (leptos::ReadSignal<Session>, leptos::WriteSignal<Session>) {
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

    let session_state = Session {
        tiles: word.to_vec(),
        selected_letter: String::from("_"),
    };

    create_signal(session_state)
}

#[component]
pub fn SessionView() -> impl IntoView {
    let (game, _) = use_game();
    let (session, set_session) = use_session();

    view! {
        <div class="flex flex-col space-y-2 h-full">
            <Tiles session=session/>
            <Keyboard session=session set_session=set_session/>

            <div class="flex flex-col space-y-2">
                <button
                    on:click=move |_| {
                        set_session
                            .update(|s| {
                                s.selected_letter = String::from("_");
                                s.tiles = game().current_word.to_vec();
                            })
                    }

                    class="border-2 border-gray-500 rounded-lg w-full p-2 text-gray-700"
                >
                    "Reset Game"
                </button>
            </div>
        </div>
    }
}

