use std::thread::available_parallelism;

use leptos::{logging::log, *};
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

use crate::components::{game::*, keyboard::Keyboard, tile::*};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SessionStatus {
    UserWon,
    ComputerWon,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub status: SessionStatus,
    pub tiles: Vec<Tile>,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            status: SessionStatus::ComputerWon,
            tiles: Vec::new(),
        }
    }
}

#[component]
pub fn SessionView(
    game_info_dialog_status: ReadSignal<bool>,
    set_game_info_dialog_status: WriteSignal<bool>,
) -> impl IntoView {
    // create the initial session
    let (game, set_game) = use_game();

    view! {
        <div class="flex flex-col space-y-2 h-full w-full">
            <Tiles game_over=false winner=TileAuthor::User/>
            <Keyboard game_info_dialog_status set_game_info_dialog_status/>

            <div class="flex flex-col space-y-2">
                <button
                    on:click=move |_| set_game_info_dialog_status(true)
                    class="border-2 border-gray-500 rounded-lg w-full p-2 text-gray-700"
                >
                    "Game Info"
                </button>
            </div>
        </div>
    }
}

