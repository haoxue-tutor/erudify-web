use crate::app::Exercise;
use leptos::prelude::*;

#[component]
pub fn SizedInput() -> impl IntoView {
    let (value, set_value) = signal(String::new());
    view! {
        <input type="text" style:width="5em" autofocus on:input=move |ev| {
            ev.prevent_default();
            log::info!("Input: {}", event_target_value(&ev));
            set_value.set(event_target_value(&ev).to_uppercase());
        }
            prop:value=value/>
    }
}

#[component]
pub fn AudioExercise(exercise: Exercise, index: usize) -> impl IntoView {
    log::info!("index: {index}");
    view! {
        <form>
            <For
                each=move || exercise.segments.clone().into_iter().enumerate()
                key=|(_idx, segment)| segment.chinese.clone()
                let:segment
            >
                {move || {
                    let _width = segment.1.chinese.chars().count();
                    let x = if segment.0 == index {
                        view! {
                            <SizedInput/>
                        }.into_any()
                    } else {
                        view! { {segment.1.chinese.to_string()} }.into_any()
                    };
                    view! { {x} }
                }}
            </For>
        </form>
    }
}
