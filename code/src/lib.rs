use leptos::html::P;
use leptos::{leptos_dom::helpers::window_event_listener, logging::log, *};
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

mod components;

use components::game::*;
use components::tile::*;

const KEY: &str = "w-8 h-16 bg-gray-300 rounded-lg cursor-pointer";
const BACK: &str =
    "w-12 h-16 bg-red-300 rounded-lg cursor-pointer justify-center items-center flex";
const ENTER: &str =
    "p-1 h-16 bg-green-300 rounded-lg cursor-pointer items-center justify-center flex";

#[derive(Serialize, Deserialize, Clone)]
struct User {
    pub name: String,
}
#[derive(Serialize, Deserialize, Clone)]
struct Session {
    pub tiles: Vec<Tile>,
    pub selected_letter: String,
}

#[component]
pub fn App() -> impl IntoView {
    let word =
        [
            Tile {
                letter: String::from("c"),
                author: TileAuthor::Computer,
            },
            Tile {
                letter: String::from("a"),
                author: TileAuthor::Computer,
            },
        ];

    let word_len = word.len();

    let game_state = Game {
        game_key: 1,
        start_tiles: word.to_vec(),
    };

    let session_state = Session {
        tiles: word.to_vec(),
        selected_letter: String::from("_"),
    };

    let (game, _, _) = use_local_storage::<Game, Game>("game", game_state);
    let (session, set_session, _) = use_local_storage::<Session, Session>("session", session_state);
    println!("test");

    let tile_class = move || match session.get().selected_letter.as_str() {
        "_" => TILE_EMPTY,
        _ => TILE_OPTION,
    };

    let lock_in_letter = move || {
        if session().selected_letter != "_" && session().selected_letter != "" {
            set_session.update(|s| {
                s.tiles.push(Tile {
                    letter: s.selected_letter.clone(),
                    author: TileAuthor::User,
                });
                s.selected_letter = String::from("_");
            })
        }
    };

    let remove_letter =
        move || {
            set_session.update(|s| {
                if s.selected_letter != "_" {
                    s.selected_letter = String::from("_");
                } else {
                    if s.tiles.len() > word_len {
                        let (_, tiles) = s.tiles.split_last().unwrap();
                        s.tiles = tiles.to_vec();
                    }
                }
            })
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

    let handle_keyboard = window_event_listener(ev::keydown, move |ev| {
        // ev is typed as KeyboardEvent automatically,
        // so .code() can be called
        ev.prevent_default();
        let code = ev.code();
        if code.starts_with("Key") {
            let key = code.strip_prefix("Key").unwrap();
            set_session.update(|s| s.selected_letter = String::from(key))
        }

        if code == "Enter" {
            lock_in_letter();
        }

        if code == "Backspace" {
            remove_letter();
        }
    });

    on_cleanup(move || {
        handle_keyboard.remove();
    });

    view! {
        <main class="flex text-xl justify-between flex-col p-4 space-y-4 h-full text-gray-700">
            <div class="flex space-y-2 flex-col items-center">
                <div class="text-4xl">"Challenge: #" {move || game().game_key}</div>
                <div class="text-base text-center">
                    "Between you and the computer, make the longest word without being the one to enter the last letter."
                </div>
            </div>
            <div class="flex-1 flex items-center justify-center">
                <ul class="flex flex-wrap gap-y-1 gap-x-1 max-w-screen text-2xl justify-center items-center uppercase">
                    {tiles} <li class=tile_class>{move || session.get().selected_letter}</li>
                </ul>
            </div>

            <div
                id="keyboard"
                class="flex flex-col space-y-1 w-full items-center text-sm text-gray-700"
            >
                <div id="top-row" class="flex flex-row space-between space-x-1">
                    <button
                        on:click=move |_| {
                            set_session.update(|s| s.selected_letter = String::from("Q"))
                        }

                        class=KEY
                    >
                        "Q"
                    </button>
                    <button
                        on:click=move |_| {
                            set_session.update(|s| s.selected_letter = String::from("W"))
                        }

                        class=KEY
                    >
                        "W"
                    </button>
                    <button class=KEY>"E"</button>
                    <button class=KEY>"R"</button>
                    <button class=KEY>"T"</button>
                    <button class=KEY>"Y"</button>
                    <button class=KEY>"U"</button>
                    <button class=KEY>"I"</button>
                    <button class=KEY>"O"</button>
                    <button class=KEY>"P"</button>
                </div>
                <div id="middle-row" class="flex flex-row space-between space-x-1">
                    <button class=KEY>"A"</button>
                    <button class=KEY>"S"</button>
                    <button class=KEY>"D"</button>
                    <button class=KEY>"F"</button>
                    <button class=KEY>"G"</button>
                    <button class=KEY>"H"</button>
                    <button class=KEY>"J"</button>
                    <button class=KEY>"K"</button>
                    <button class=KEY>"L"</button>
                </div>
                <div id="bottom-row" class="flex flex-row space-between space-x-1">
                    <button
                        on:click=move |_| {
                            lock_in_letter();
                        }

                        class=ENTER
                    >
                        ENTER
                    </button>
                    <button class=KEY>"Z"</button>
                    <button class=KEY>"X"</button>
                    <button class=KEY>"C"</button>
                    <button class=KEY>"V"</button>
                    <button class=KEY>"B"</button>
                    <button class=KEY>"N"</button>
                    <button class=KEY>"M"</button>
                    <button
                        on:click=move |_| {
                            remove_letter();
                        }

                        class=BACK
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            class="w-6 h-6"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M2.515 10.674a1.875 1.875 0 000 2.652L8.89 19.7c.352.351.829.549 1.326.549H19.5a3 3 0 003-3V6.75a3 3 0 00-3-3h-9.284c-.497 0-.974.198-1.326.55l-6.375 6.374zM12.53 9.22a.75.75 0 10-1.06 1.06L13.19 12l-1.72 1.72a.75.75 0 101.06 1.06l1.72-1.72 1.72 1.72a.75.75 0 101.06-1.06L15.31 12l1.72-1.72a.75.75 0 10-1.06-1.06l-1.72 1.72-1.72-1.72z"
                                clip-rule="evenodd"
                            ></path>
                        </svg>
                    </button>
                </div>
            </div>

            <div class="flex flex-col space-y-2">
                <button
                    on:click=move |_| {
                        set_session
                            .update(|s| {
                                s.selected_letter = String::from("_");
                                s.tiles = word.to_vec();
                            })
                    }

                    class="border-2 border-gray-500 rounded-lg w-full p-2 text-gray-700"
                >
                    "Reset Game"
                </button>
            </div>
        </main>
    }
}

