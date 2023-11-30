use leptos::{logging::log, svg::view, *};
use serde::{Deserialize, Serialize};

use crate::components::game::{use_game, Game};

use super::session::Session;

pub const TILE_COMP: &str =
    "w-16 h-20 bg-gray-300 rounded-lg  flex justify-center items-center border-2 border-gray-400";

pub const TILE_YOU: &str =
    "w-16 h-20 bg-green-300 border-2 border-green-500 rounded-lg flex justify-center items-center";

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum TileKind {
    TileEmpty,
    TileYou,
    TileComp,
    TileOkay,
    TileNotOkay,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum TileAuthor {
    Computer,
    User,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Tile {
    pub letter: char,
    pub author: TileAuthor,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            letter: 'c',
            author: TileAuthor::Computer,
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.letter == other.letter
    }
}

pub fn convert_tiles_to_word(tiles: Vec<Tile>) -> String {
    tiles
        .into_iter()
        .map(|t| t.letter.to_lowercase().to_string())
        .collect()
}

#[component]
pub fn Tiles(session: Signal<Session>) -> impl IntoView {
    let (game, _) = use_game();

    let (tile_type, set_tile_type) = create_signal(TileKind::TileEmpty);

    create_effect(move |_| {
        set_tile_type.set(if game().selected_letter == '_' {
            TileKind::TileEmpty
        } else {
            match game().available_letters.contains(&game().selected_letter) {
                true => TileKind::TileOkay,
                false => TileKind::TileNotOkay,
            }
        })
    });

    // current tile

    let tiles = move || {
        game.get()
            .current_tiles
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

    let enter_tile_view = move || match tile_type.get() {
        TileKind::TileOkay => view! { <OkayTile game/> },
        TileKind::TileEmpty => view! { <EmptyTile game/> },
        TileKind::TileNotOkay => view! { <NotOkayTile game/> },
        _ => view! { <EmptyTile game/> },
    };

    view! {
        <div class="flex-1 flex items-center justify-center">
            <ul class="flex flex-wrap gap-y-1 gap-x-1 max-w-screen text-2xl justify-center items-center uppercase">
                {tiles} {enter_tile_view}
            </ul>
        </div>
    }
}

#[component]
fn EmptyTile(game: Signal<Game>) -> impl IntoView {
    view! {
        <li class="w-16 h-20 border-2 border-gray-300 rounded-lg flex justify-center items-center text-gray-300">
            {move || game.get().selected_letter}
        </li>
    }
}

#[component]
fn NotOkayTile(game: Signal<Game>) -> impl IntoView {
    view! {
        <li class="w-16 h-20 border-2 border-red-600 rounded-lg flex justify-center items-center text-red-600">
            {move || game.get().selected_letter}
        </li>
    }
}

#[component]
fn OkayTile(game: Signal<Game>) -> impl IntoView {
    view! {
        <li class="w-16 h-20 border-2 border-green-600 rounded-lg flex justify-center items-center text-green-600">
            {move || game.get().selected_letter}
        </li>
    }
}

