use leptos::*;
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

const TILE_COMP: &str = "w-16 h-20 bg-gray-300 rounded-lg";
const TILE_YOU: &str = "w-16 h-20 bg-green-300 rounded-lg";
const KEY: &str = "w-8 h-16 bg-gray-300 rounded-lg cursor-pointer";
const BACK: &str = "w-12 h-16 bg-red-300 rounded-lg cursor-pointer";
const ENTER: &str = "w-12 h-16 bg-green-300 rounded-lg cursor-pointer";

#[derive(Serialize, Deserialize, Clone)]
struct User {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Game {
    pub game_key: i32,
}

#[component]
pub fn App() -> impl IntoView {
    let user_state = User {
        name: String::from("Andrew"),
    };

    let game_state = Game { game_key: 1 };

    let (user, set_user, _) = use_local_storage::<User, User>("user", user_state);
    let (game, set_game, _) = use_local_storage::<Game, Game>("game", game_state);

    view! {
        <main class="flex text-xl  flex-col space-y-4 m-4">
            <div class="text-4xl">"Challenge: #" {move || game().game_key}</div>

            <input
                class="block rounded-lg border-2 border-gray-300 p-2"
                placeholder="Enter your name..."
                prop:value=move || user.get().name
                on:input=move |e| set_user.update(|s| s.name = event_target_value(&e))
                type="text"
            />

            <div class="flex space-x-2">
                <div class=TILE_COMP></div>
                <div class=TILE_YOU></div>
                <div class=TILE_COMP></div>
                <div class=TILE_YOU></div>
                <div class=TILE_COMP></div>
                <div class=TILE_YOU></div>
                <div class=TILE_COMP></div>
                <div class=TILE_YOU></div>
                <div class=TILE_COMP></div>
                <div class=TILE_YOU></div>
            </div>

            <div id="keyboard" class="flex flex-col space-y-1 w-full items-center">
                <div id="top-row" class="flex flex-row space-between space-x-1">
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>

                </div>
                <div id="middle-row" class="flex flex-row space-between space-x-1">
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                </div>
                <div id="bottom-row" class="flex flex-row space-between space-x-1">
                    <button class=ENTER></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=KEY></button>
                    <button class=BACK></button>
                </div>
            </div>

            <button
                class="bg-gray-300 rounded-lg w-full p-2 text-gray-900"
                on:click=move |_| { set_game.update(|g| g.game_key += 1) }
            >

                "Reset Game"
            </button>
            <button
                class="border-2 border-gray-300 rounded-lg w-full p-2 text-gray-700"
                on:click=move |_| { set_game.update(|g| g.game_key += 1) }
            >

                "Stats"
            </button>
        </main>
    }
}

