use leptos::*;
use serde::{Deserialize, Serialize};

// use crate::components::game::{get_game, GetGame};
use crate::components::game::*;
use crate::components::session::*;
use crate::components::stats_dialog::*;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    pub name: String,
}

#[component]
pub fn Index() -> impl IntoView {
    let (is_dialog_open, set_dialog_status) = create_signal(false);

    view! {
        <div class="flex text-xl justify-between flex-col p-4 space-y-4 h-full text-gray-700">
            <div class="flex space-y-2 flex-col items-center h-full">
                <GameHeader/>
                <div class="text-base text-center">
                    "Alternating with the computer, try to make the longest word one letter at a time. You lose if you are the one to finish the word."
                </div>
                <SessionView/>
                <button
                    on:click=move |_| {
                        set_dialog_status(true);
                    }

                    class="border-2 border-gray-500 rounded-lg w-full p-2 text-gray-700"
                >
                    "Stats"
                </button>
            </div>
        </div>
        <StatsDialog is_open=is_dialog_open set_dialog_status/>
    }
}

