use leptos::{logging::log, svg::view, *};
use serde::{Deserialize, Serialize};

use crate::components::game::{use_game, Game};

use super::session::Session;

pub const TILE_COMP: &str = "";

pub const TILE_YOU: &str = "";

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
pub fn Tiles(game_over: bool, winner: TileAuthor) -> impl IntoView {
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
            .map(move |tile| match game_over {
                true => match winner {
                    TileAuthor::Computer => view! { <LostTile letter=tile.letter/> },
                    TileAuthor::User => view! { <YouTile letter=tile.letter/> },
                },
                false => match tile.author {
                    TileAuthor::Computer => view! { <ComputerTile letter=tile.letter/> },
                    TileAuthor::User => view! { <YouTile letter=tile.letter/> },
                },
            })
            .collect::<Vec<_>>()
    };

    let enter_tile_view = move || match game_over {
        true => view! { <div></div> }.into_view(),
        false => match tile_type.get() {
            TileKind::TileOkay => view! { <OkayTile letter=game().selected_letter/> },
            TileKind::TileEmpty => view! { <EmptyTile letter=game().selected_letter/> },
            TileKind::TileNotOkay => view! { <NotOkayTile letter=game().selected_letter/> },
            _ => view! { <EmptyTile letter=game().selected_letter/> },
        },
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
fn EmptyTile(letter: char) -> impl IntoView {
    view! {
        <li class="w-16 h-20 border-2 border-gray-300 rounded-lg flex justify-center items-center text-gray-300">
            {letter}
        </li>
    }
}

#[component]
fn NotOkayTile(letter: char) -> impl IntoView {
    view! {
        <li class="w-16 h-20 border-2 border-red-600 rounded-lg flex justify-center items-center text-red-600">
            {letter}
        </li>
    }
}

#[component]
fn OkayTile(letter: char) -> impl IntoView {
    view! {
        <li class="w-16 h-20 border-2 border-green-600 rounded-lg flex justify-center items-center text-green-600">
            {letter}
        </li>
    }
}

#[component]
fn YouTile(letter: char) -> impl IntoView {
    view! {
        <li class="w-16 h-20 bg-green-300 border-2 border-green-500 rounded-lg flex justify-center items-center">
            {letter}
        </li>
    }
}

#[component]
fn ComputerTile(letter: char) -> impl IntoView {
    view! {
        <li class="w-16 h-20 bg-gray-300 rounded-lg  flex justify-center items-center border-2 border-gray-400">
            {letter}
        </li>
    }
}

#[component]
fn LostTile(letter: char) -> impl IntoView {
    view! {
        <li class="w-16 h-20 bg-red-300 border-2 border-red-500 rounded-lg flex justify-center items-center">
            {letter}
        </li>
    }
}

