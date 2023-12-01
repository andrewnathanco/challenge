use leptos::*;
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
    let (game_info_dialog_status, set_game_info_dialog_status) = create_signal(false);
    view! {
        <div class="flex text-xl justify-between flex-col p-4 space-y-4 h-full text-gray-700">
            <div class="flex space-y-2 flex-col items-center h-full">
                <GameHeader/>
                <div class="text-base text-center">
                    "Alternating with the computer, try to make the longest word one letter at a time. You lose if you are the one to finish the word."
                </div>
                <SessionView game_info_dialog_status set_game_info_dialog_status/>
            </div>
        </div>
    }
}

