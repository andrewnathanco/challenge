use leptos::{logging::log, *};
use serde::{Deserialize, Serialize};

// use crate::components::game::{get_game, GetGame};
use crate::components::game::*;
use crate::components::session::*;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    pub name: String,
}

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <div class="flex text-xl justify-between flex-col p-4 space-y-4 h-full text-gray-700">
            <div class="flex space-y-2 flex-col items-center h-full">
                <GameHeader/>
                <div class="text-base text-center">
                    "Between you and the computer, make the longest word without being the one to enter the last letter."
                </div>
                <SessionView/>
            </div>
        </div>
    }
}

