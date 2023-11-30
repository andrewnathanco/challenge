use std::thread::available_parallelism;

use leptos::{logging::log, *};
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

use crate::components::{game::*, keyboard::Keyboard, tile::*};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {}

impl Default for Session {
    fn default() -> Self {
        Session {}
    }
}

#[component]
pub fn SessionView() -> impl IntoView {
    // create the initial session
    let (session, _, _) = use_local_storage("session", Session::default());
    let (game, set_game) = use_game();

    let reset_game = move |_| {
        let starting_tiles = game().starting_tiles;
        let starting_letters = game().starting_letters;

        set_game.update(|g| {
            g.selected_letter = '_';
            g.current_tiles = starting_tiles;
            g.available_letters = starting_letters;
        });
    };

    view! {
        <div class="flex flex-col space-y-2 h-full w-full">
            <Tiles session=session/>
            <Keyboard/>

            <div class="flex flex-col space-y-2">
                <button
                    on:click=reset_game

                    class="border-2 border-gray-500 rounded-lg w-full p-2 text-gray-700"
                >
                    "Reset Game"
                </button>
            </div>
        </div>
    }
}

