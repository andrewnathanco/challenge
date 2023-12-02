use leptos::*;
use serde::{Deserialize, Serialize};

use super::session::SessionStatus;
use crate::components::{game::use_game, session::use_session};

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
pub fn Tiles(tiles: Vec<Tile>, read_only: bool) -> impl IntoView {
    let (game, _) = use_game();
    let (session, _) = use_session();
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
    let tiles_view = move || {
        tiles
            .clone()
            .into_iter()
            .map(move |tile| match read_only {
                false => match tile.author {
                    TileAuthor::Computer => view! { <ComputerTile letter=tile.letter/> },
                    TileAuthor::User => view! { <YouTile letter=tile.letter/> },
                },
                true => {
                    match session().status {
                        SessionStatus::UserWon => view! { <YouTile letter=tile.letter/> },
                        SessionStatus::ComputerWon => view! { <LostTile letter=tile.letter/> },
                        SessionStatus::Current => match tile.author {
                            TileAuthor::Computer => view! { <ComputerTile letter=tile.letter/> },
                            TileAuthor::User => view! { <YouTile letter=tile.letter/> },
                        },
                    }
                }
            })
            .collect::<Vec<_>>()
    };

    let enter_tile_view = move || match session().status {
        SessionStatus::Current => match tile_type.get() {
            TileKind::TileOkay => view! { <OkayTile letter=game().selected_letter/> },
            TileKind::TileEmpty => view! { <EmptyTile letter=game().selected_letter/> },
            TileKind::TileNotOkay => view! { <NotOkayTile letter=game().selected_letter/> },
            _ => view! { <EmptyTile letter=game().selected_letter/> },
        },
        _ => view! { <div></div> }.into_view(),
    };

    view! {
        <div class="flex-1 flex items-center justify-center">
            <ul class="flex flex-wrap gap-y-1 gap-x-1 max-w-screen text-2xl justify-center items-center uppercase">
                {tiles_view} {enter_tile_view}
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

