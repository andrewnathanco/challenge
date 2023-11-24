use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (game_key, set_game_key) = create_signal(0);

    view! {
        <main class="flex text-xl  flex-col space-y-4 m-4 font-bold">
            <div class="text-4xl">"Challenge: #" {move || game_key()}</div>
            <button
                class="bg-gray-300 rounded-lg w-full p-2"
                on:click=move |_| {
                    set_game_key.update(|n| *n += 1);
                }
            >

                "Reset Game"
            </button>
        </main>
    }
}

