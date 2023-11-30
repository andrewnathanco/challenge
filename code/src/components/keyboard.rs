use crate::components::game::use_game;
use crate::components::tile::{Tile, TileAuthor};
use crate::components::word::get_available_letters;
use leptos::leptos_dom::helpers::window_event_listener;
use leptos::{*, logging::log};

use super::computer::get_tile_from_computer;
use super::session::Session;
use super::game::Game;


const TOP: [&str; 10] = ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"];
const MIDDLE: [&str; 9] = ["A", "S", "D", "F", "G", "H", "J", "K", "L"];
const BOTTOM: [&str; 7] = ["Z", "X", "C", "V", "B", "N", "M"];

pub fn get_all_letters() -> Vec<String> {
    let mut all_letters: Vec<String> = Vec::new();

    // Add letters from TOP array
    for &letter in TOP.iter() {
        all_letters.push(letter.to_string());
    }

    // Add letters from MIDDLE array
    for &letter in MIDDLE.iter() {
        all_letters.push(letter.to_string());
    }

    // Add letters from BOTTOM array
    for &letter in BOTTOM.iter() {
        all_letters.push(letter.to_string());
    }

    all_letters
}

#[component]
pub fn Keyboard(
) -> impl IntoView {
    let (game, set_game) = use_game();


    let set_tile_from_comp = 
        create_action(move |tiles: &Vec<Tile>| {
            let new_tiles = tiles.clone();
            let mut new_game = game();
            async move {
                let tile = get_tile_from_computer(new_tiles.clone()).await.unwrap();
                new_game.current_tiles.push(tile);
                let avail_lett = get_available_letters(new_game.current_tiles.clone()).await.unwrap();
                new_game.available_letters = avail_lett;
                set_game.set(new_game)
            }
    });

    let set_available_letters = 
        create_action(move |tiles: &Vec<Tile>| {
            let new_tiles = tiles.clone();
            let mut new_game = game();
            async move {
                let avail_lett = get_available_letters(new_tiles).await.unwrap();
                new_game.available_letters = avail_lett;
                set_game.set(new_game)
            }
    });

    // this grabs the letter from the user, resets the selected and adds the letter as a tile
    let lock_in_letter = move || {
        let selected_letter = game().selected_letter;
        if game().selected_letter != '_' {
            // update the selected letter
            set_game.update(|g| {
                g.current_tiles.push(Tile {
                    letter: selected_letter,
                    author: TileAuthor::User,
                });

                g.selected_letter = '_';
            });
        }
    };

    let submit_letter = move |valid_letter: bool| {
        if valid_letter {
            if game().selected_letter != '_' {
                // lock in letter from the user
                lock_in_letter();

                set_tile_from_comp.dispatch(game().current_tiles)
            }
        }
    };

    // currently this just strips the last letter off until it hits the start
    let remove_letter = move || {
        if game().selected_letter != '_' {
            set_game.update(|g| {
                g.selected_letter = '_';
            });
            return
        }
        let curr_tiles = game().current_tiles.clone();
        let len_curr = game().current_tiles.len();
        let len_starting = game().starting_tiles.len();

        // this makes sure we don't go past the starting tiles
        let enough_tiles_to_strip = len_curr  > len_starting;
        if enough_tiles_to_strip {
            set_game.update(|g| {
                let (_, tiles) = curr_tiles.split_last().unwrap();
                let (_, tiles) = tiles.split_last().unwrap();
                g.current_tiles = tiles.to_vec();
            });

            set_available_letters.dispatch(game().current_tiles)
        }
     };

    let top_keys = move || {
        TOP.map(|k| {
            view! { <Key letter={ k }.to_string() set_game=set_game/> }
        })
    };

    let middle_keys = move || {
        MIDDLE.map(|k| {
            view! { <Key letter={ k }.to_string() set_game=set_game/> }
        })
    };

    let bottom_keys = move || {
        BOTTOM.map(|k| {
            view! { <Key letter={ k }.to_string() set_game=set_game/> }
        })
    };

    // this makes sure that users can use their keyboard instead of the on-screen one
    let handle_keyboard = window_event_listener(ev::keydown, move |ev| {
        ev.prevent_default();
        let code = ev.code();
        if code.starts_with("Key") {
            let key = code.strip_prefix("Key").unwrap();
            set_game.update(|game| 
                game.selected_letter = key.chars().nth(0).unwrap_or_default()
            )
        }

        if code == "Enter" {
            submit_letter(game().available_letters.contains(&game().selected_letter));
        }

        if code == "Backspace" {
            remove_letter();
        }
    });

    on_cleanup(move || {
        handle_keyboard.remove();
    });

    view! {
        <div
            id="keyboard"
            class="flex flex-col space-y-1 w-full items-center text-sm text-gray-700"
        >
            <div id="top-row" class="flex flex-row space-between space-x-1">
                {top_keys}
            </div>
            <div id="middle-row" class="flex flex-row space-between space-x-1">
                {middle_keys}
            </div>
            <div id="bottom-row" class="flex flex-row space-between space-x-1">
                <button
                    on:click=move |_| {
                        submit_letter(game().available_letters.contains(&game().selected_letter))
                    }

                    class="p-1 h-16 bg-green-300 rounded-lg cursor-pointer items-center justify-center flex"
                >
                    ENTER
                </button>
                {bottom_keys}
                <button
                    on:click=move |_| { remove_letter() }

                    class="w-12 h-16 bg-red-300 rounded-lg cursor-pointer justify-center items-center flex"
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
    }
}

#[component]
pub fn Key(letter: String, set_game: WriteSignal<Game>) -> impl IntoView {
    // needed because we are doing some cloning
    let lett = letter.clone();
    view! {
        <button
            on:click=move |_| {
                set_game.update(|g| g.selected_letter = lett.chars().nth(0).unwrap_or_default())
            }

            class="w-8 h-16 bg-gray-300 rounded-lg cursor-pointer"
        >
            {letter}
        </button>
    }
}






















































































































































































