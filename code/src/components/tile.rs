use leptos::*;
use serde::{Deserialize, Serialize};

use super::session::Session;

pub const TILE_COMP: &str =
    "w-16 h-20 bg-gray-300 rounded-lg  flex justify-center items-center border-2 border-gray-400";

pub const TILE_YOU: &str =
    "w-16 h-20 bg-green-300 border-2 border-green-500 rounded-lg flex justify-center items-center";

pub const TILE_EMPTY: &str =
    "w-16 h-20 border-2 border-gray-300 rounded-lg flex justify-center items-center text-gray-300";

pub const TILE_OKAY: &str =
    "w-16 h-20 border-2 border-green-600 rounded-lg flex justify-center items-center text-green-600";

pub const TILE_NOT_OKAY: &str =
    "w-16 h-20 border-2 border-red-600 rounded-lg flex justify-center items-center text-red-600";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TileAuthor {
    Computer,
    User,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tile {
    pub letter: String,
    pub author: TileAuthor,
}

#[component]
pub fn Tiles(session: Signal<Session>) -> impl IntoView {
    // current tile
    let tile_class = move || match session().selected_letter.to_uppercase().as_str() {
        "_" => TILE_EMPTY,
        "A" => TILE_NOT_OKAY,
        _ => TILE_OKAY,
    };

    let tiles = move || {
        session
            .get()
            .tiles
            .into_iter()
            .map(|n| {
                view! {
                    <li class=match n.author {
                        TileAuthor::Computer => TILE_COMP,
                        TileAuthor::User => TILE_YOU,
                    }>

                        {n.letter}
                    </li>
                }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="flex-1 flex items-center justify-center">
            <ul class="flex flex-wrap gap-y-1 gap-x-1 max-w-screen text-2xl justify-center items-center uppercase">
                {tiles} <li class=tile_class>{move || session.get().selected_letter}</li>
            </ul>
        </div>
    }
}

