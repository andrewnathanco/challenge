use std::time::Duration;

use chrono::Local;
use leptos::*;

#[component]
pub fn StatsDialog(
    is_open: ReadSignal<bool>,
    set_dialog_status: WriteSignal<bool>,
) -> impl IntoView {
    // Get the current local time
    let (curr_time, set_curr_time) = create_signal(Local::now());

    let display_class =
        move || {
            if is_open() {
                "block"
            } else {
                "hidden"
            }
        };

    view! {
        <div class=display_class>
            <div class="z-10 absolute top-0 left-0 right-0 bottom-0 justify-center items-center bg-black flex opacity-70"></div>
            <div class="z-20 absolute top-0 left-0 right-0 bottom-0 bg-white flex m-4 rounded-lg">
                <div id="dialog-content" class="p-8 flex flex-col space-y-2 w-full">
                    <div
                        id="dialog-header"
                        class="flex justify-between items-center text-3xl w-full"
                    >
                        <div>"Statistics"</div>
                        <button on:click=move |_| { set_dialog_status(false) }>
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="currentColor"
                                class="w-6 h-6"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M5.47 5.47a.75.75 0 011.06 0L12 10.94l5.47-5.47a.75.75 0 111.06 1.06L13.06 12l5.47 5.47a.75.75 0 11-1.06 1.06L12 13.06l-5.47 5.47a.75.75 0 01-1.06-1.06L10.94 12 5.47 6.53a.75.75 0 010-1.06z"
                                    clip-rule="evenodd"
                                ></path>
                            </svg>
                        </button>
                    </div>
                    <div class="flex-1"></div>
                    <div class="flex flex-col space-y-2">
                        <div id="timer" class="flex flex-col space-y-1 items-center">
                            <div id="time" class="text-3xl">
                                "10:22:32"
                            </div>
                            <div id="text" class="text-2xl">
                                "Next Challenge"
                            </div>
                        </div>
                        <button
                            on:click=move |_| {
                                set_dialog_status(true);
                            }

                            class="border-2 border-green-400 rounded-lg w-full p-2 text-gray-700 bg-green-400 flex items-center justify-center space-x-2"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="currentColor"
                                class="w-6 h-6"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M15.75 4.5a3 3 0 11.825 2.066l-8.421 4.679a3.002 3.002 0 010 1.51l8.421 4.679a3 3 0 11-.729 1.31l-8.421-4.678a3 3 0 110-4.132l8.421-4.679a3 3 0 01-.096-.755z"
                                    clip-rule="evenodd"
                                ></path>
                            </svg>

                            <div>"Share"</div>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

