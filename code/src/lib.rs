use leptos::*;
use leptos_use::storage::use_local_storage;
use views::index::Index;

use crate::components::game::{get_new_game, Game};

mod components;
mod views;

#[component]
pub fn App() -> impl IntoView {
    view! { <Index/> }
}

