use leptos::*;

use super::session::Session;

const KEY: &str = "w-8 h-16 bg-gray-300 rounded-lg cursor-pointer";
const BACK: &str =
    "w-12 h-16 bg-red-300 rounded-lg cursor-pointer justify-center items-center flex";
const ENTER: &str =
    "p-1 h-16 bg-green-300 rounded-lg cursor-pointer items-center justify-center flex";

const TOP: [&str; 10] = ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"];
const MIDDLE: [&str; 9] = ["A", "S", "D", "F", "G", "H", "J", "K", "L"];
const BOTTOM: [&str; 7] = ["Z", "X", "C", "V", "B", "N", "M"];

#[component]
pub fn Keyboard(
    set_session: WriteSignal<Session>,
    enter: Box<dyn Fn() + 'static>,
    remove: Box<dyn Fn() + 'static>,
) -> impl IntoView {
    let top_keys = move || {
        TOP.map(|k| {
            view! { <Key letter={ k }.to_string() set_session=set_session/> }
        })
    };

    let middle_keys = move || {
        MIDDLE.map(|k| {
            view! { <Key letter={ k }.to_string() set_session=set_session/> }
        })
    };

    let bottom_keys = move || {
        BOTTOM.map(|k| {
            view! { <Key letter={ k }.to_string() set_session=set_session/> }
        })
    };
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
                    on:click=move |_| { enter() }

                    class=ENTER
                >
                    ENTER
                </button>
                {bottom_keys}
                <button
                    on:click=move |_| { remove() }

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
    }
}

#[component]
pub fn Key(letter: String, set_session: WriteSignal<Session>) -> impl IntoView {
    // needed because we are doing some cloning
    let lett = letter.clone();
    view! {
        <button
            on:click=move |_| { set_session.update(|s| s.selected_letter = lett.clone()) }

            class=KEY
        >
            {letter}
        </button>
    }
}

