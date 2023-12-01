use std::thread::available_parallelism;

use leptos::{logging::log, *};
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

use crate::components::{game::*, keyboard::Keyboard, tile::*};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SessionStatus {
    Current,
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
            status: SessionStatus::Current,
            tiles: Vec::new(),
        }
    }
}

pub fn use_session() -> (Signal<Session>, WriteSignal<Session>) {
    let (session, set_session, _) = use_local_storage("session", Session::default());
    (session, set_session)
}

#[component]
pub fn SessionView(
    game_info_dialog_status: ReadSignal<bool>,
    set_game_info_dialog_status: WriteSignal<bool>,
) -> impl IntoView {
    let (game, _) = use_game();

    let tiles = move || {
        view! { <Tiles tiles=game().current_tiles.clone() read_only=false/> }
    };

    view! {
        <div class="flex flex-col space-y-2 h-full w-full">
            {tiles} <Keyboard game_info_dialog_status set_game_info_dialog_status/>
            <div class="flex flex-col space-y-2">
                <button
                    on:click=move |_| set_game_info_dialog_status(true)
                    class="border-2 border-gray-500 rounded-lg w-full p-2 text-gray-700"
                >
                    "Stats"
                </button>
            </div>
        </div>
    }
}

