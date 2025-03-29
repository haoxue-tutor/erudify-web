use crate::components::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Exercise {
    pub segments: Vec<Segment>,
    pub english: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    pub chinese: String,
    pub pinyin: String,
}

#[allow(unused)]
pub fn ex_1() -> Exercise {
    Exercise {
        segments: vec![
            Segment {
                chinese: String::from("我"),
                pinyin: String::from("wǒ"),
            },
            Segment {
                chinese: String::from("是"),
                pinyin: String::from("shì"),
            },
            Segment {
                chinese: String::from("学生"),
                pinyin: String::from("xué sheng"),
            },
            Segment {
                chinese: String::from("。"),
                pinyin: String::from(""),
            },
        ],
        english: String::from("I am a student."),
    }
}

pub fn ex_2() -> Exercise {
    Exercise {
        segments: vec![
            Segment {
                chinese: String::from("明天"),
                pinyin: String::from("míng tiān"),
            },
            Segment {
                chinese: String::from("我"),
                pinyin: String::from("wǒ"),
            },
            Segment {
                chinese: String::from("会"),
                pinyin: String::from("huì"),
            },
            Segment {
                chinese: String::from("去"),
                pinyin: String::from("qù"),
            },
            Segment {
                chinese: String::from("图书馆"),
                pinyin: String::from("tú shū guǎn"),
            },
            Segment {
                chinese: String::from("。"),
                pinyin: String::from(""),
            },
        ],
        english: String::from("I will go to the library tomorrow."),
    }
}

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
    // view! { "Empty" }
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
                // {move || if segment.0 == 0 {
                //     view! { <p>"Input"</p> }
                // } else {
                //     view! { <p>{segment.1.chinese}</p> }
                // }}
                // {segment.1.chinese}
            </For>
        // for (nth , segment) in exercise.segments.iter().enumerate() {
        // if nth < index || segment.pinyin.is_empty() {
        // span {
        // class: "done",
        // style: "width: {segment.chinese.chars().count() as i64}em",
        // "{segment.chinese} "
        // }
        // } else if nth == index {
        // input {
        // style: "width: {segment.chinese.chars().count() as i64}em;",
        // name: "input",
        // onmounted: move |cx| {
        // spawn(async move {
        // let _ = cx.data().set_focus(true).await;
        // });
        // },
        // value: "{input}",
        // oninput: move |event| oninput.call(event)
        // }
        // } else {
        // span {
        // class: "future",
        // style: "width: {segment.chinese.chars().count() as i64}em",
        // "{segment.chinese} "
        // }
        // }
        // }
        </form>
    }
}
// #[component]
// fn AudioExercise(
//     exercise: Exercise,
//     index: usize,
//     input: Signal<String>,
//     onsubmit: EventHandler<FormEvent>,
//     oninput: EventHandler<FormEvent>,
// ) -> Element {
//     rsx! {
//         form { class: "exercise", onsubmit: move |event| onsubmit.call(event),
//             for (nth , segment) in exercise.segments.iter().enumerate() {
//                 if nth < index || segment.pinyin.is_empty() {
//                     span {
//                         class: "done",
//                         style: "width: {segment.chinese.chars().count() as i64}em",
//                         "{segment.chinese} "
//                     }
//                 } else if nth == index {
//                     input {
//                         style: "width: {segment.chinese.chars().count() as i64}em;",
//                         name: "input",
//                         onmounted: move |cx| {
//                             spawn(async move {
//                                 let _ = cx.data().set_focus(true).await;
//                             });
//                         },
//                         value: "{input}",
//                         oninput: move |event| oninput.call(event)
//                     }
//                 } else {
//                     span {
//                         class: "future",
//                         style: "width: {segment.chinese.chars().count() as i64}em",
//                         "{segment.chinese} "
//                     }
//                 }
//             }
//         }
//         button { class: "btn btn-blue", "Replay audio" }
//         button { class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
//             "Next exercise"
//         }
//     }
// }

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet href="/style.css" />
        <Link rel="icon" type_="image/x-icon" href="/favicon.ico" />
        <Router>
            <div class="min-h-screen bg-gray-50">
                <nav::NavBar />
                <main class="container mx-auto px-4 py-8">
                    <Routes fallback=|| {
                        view! { <div class="p-4 text-center">"Page Not Found"</div> }
                    }>
                        <Route path=path!("/") view=pages::StartPage />
                        <Route path=path!("/study") view=pages::StudyPage />
                        <Route path=path!("/signin") view=auth::SignInPage />
                        <Route path=path!("/oauth/github") view=auth::GithubOAuth2Callback />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
