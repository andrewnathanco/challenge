use leptos::{logging::log, *};
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

use crate::components::{game::*, keyboard::Keyboard, tile::*};

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[component]
pub fn SessionView() -> impl IntoView {
    // create the initial session
    let (session, set_session, _) = use_local_storage("session", Session::default());
    let (game, _) = use_game();

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
                                s.tiles = game().current_tiles.to_vec();
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

