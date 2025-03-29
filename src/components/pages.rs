use crate::app::ex_2;
use crate::components::exercise::AudioExercise;
use crate::server::shouting_text;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn StartPage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold mb-6">"Welcome to Erudify"</h1>
            <div class="bg-white rounded-lg shadow-md p-6">
                <h2 class="text-xl font-semibold mb-4">"Test Section"</h2>
                <button
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4"
                    on:click=move |_| {
                        spawn_local(async move {
                            let val = shouting_text(String::from("test")).await.ok();
                            log::info!("Got answer from server: {:?}", val);
                        });
                    }
                >
                    "Test"
                </button>
                <div class="mt-4">
                    <h3 class="text-lg font-medium mb-2">"Exercise Preview"</h3>
                    <AudioExercise exercise=ex_2() index=1 />
                </div>
            </div>
        </div>
    }
}
