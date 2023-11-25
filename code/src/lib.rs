use leptos::leptos_dom::helpers::window_event_listener;
use leptos::*;
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

mod components;

use components::game::*;
use components::keyboard::*;
use components::session::*;
use components::tile::*;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    pub name: String,
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

    // current tile
    let tile_class = move || match session().selected_letter.as_str() {
        "_" => TILE_EMPTY,
        _ => TILE_OPTION,
    };

    // this grabs the letter from the user, resets the selected and adds the letter as a tile
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

    let submit_letter = move || {
        if session().selected_letter != "_" {
            // lock in letter from the user
            lock_in_letter();

            // get back letter from the computer
            set_session.update(|s| {
                s.tiles.push(Tile {
                    letter: "A".to_string(),
                    author: TileAuthor::Computer,
                });
            })
        }
    };

    // currently this just strips the last letter off until it hits the start
    let remove_letter = move || {
        set_session.update(|s| {
            if s.selected_letter != "_" {
                s.selected_letter = String::from("_");
            } else {
                if s.tiles.len() > word_len {
                    // need to strip off users and the last computers, this is a bit dangerous,
                    // we shouldn't get to an edge case, but it's possible that this could strip off the first few tiles
                    let (_, tiles) = s.tiles.split_last().unwrap();
                    let (_, tiles) = tiles.split_last().unwrap();
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
            submit_letter();
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

            <Keyboard
                set_session=set_session
                enter=Box::new(submit_letter)
                remove=Box::new(remove_letter)
            />

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

