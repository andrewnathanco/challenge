use leptos::*;
use views::index::Index;

mod components;
mod views;

#[component]
pub fn App() -> impl IntoView {
    view! { <Index/> }
}

