use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use serde::{Deserialize, Serialize};

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));
// const _STYLE_URL: &str = manganis::mg!(file("public/style.css"));

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();
    launch(app);
}

// #[component]
// fn DarkModeToggle() -> Element {
//     rsx! {
//         div {
//             onmounted: |cx| {
//                 let win = web_sys::window().unwrap();
//                 let _ = win.media_matches("(prefers-color-scheme: dark)");
//             }
//         }
//     }
// }

// Create component for rendering an audio exercise
//    Chinese segments: [String]
//    Completed segments: usize
//    Show English translation: bool
//    Entered text: Signal<String>

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Exercise {
    pub segments: Vec<Segment>,
    pub english: String,
}

fn ex_1() -> Exercise {
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
fn ex_2() -> Exercise {
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    pub chinese: String,
    pub pinyin: String,
}

#[component]
fn AudioExercise(
    exercise: Exercise,
    index: usize,
    input: Signal<String>,
    onsubmit: EventHandler<FormEvent>,
    oninput: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        form { class: "exercise", onsubmit: move |event| onsubmit.call(event),
            for (nth , segment) in exercise.segments.iter().enumerate() {
                if nth < index || segment.pinyin.is_empty() {
                    span {
                        class: "done",
                        style: "width: {segment.chinese.chars().count() as i64}em",
                        "{segment.chinese} "
                    }
                } else if nth == index {
                    input {
                        style: "width: {segment.chinese.chars().count() as i64}em;",
                        name: "input",
                        onmounted: move |cx| {
                            spawn(async move {
                                let _ = cx.data().set_focus(true).await;
                            });
                        },
                        value: "{input}",
                        oninput: move |event| oninput.call(event)
                    }
                } else {
                    span {
                        class: "future",
                        style: "width: {segment.chinese.chars().count() as i64}em",
                        "{segment.chinese} "
                    }
                }
            }
        }
        button { class: "btn btn-blue", "Replay audio" }
        button { class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
            "Next exercise"
        }
    }
}

fn app() -> Element {
    let mut nth = use_signal(|| 0_usize);
    let mut txt = use_signal(|| "".to_string());

    rsx! {
        style { {include_str!("../public/style.css")} }
        AudioExercise {
            exercise: ex_2(),
            index: nth(),
            input: txt,
            onsubmit: move |event: FormEvent| {
                info!("Submit: {:?}", event.values());
                nth += 1;
                txt.set(String::new());
            },
            oninput: move |event: FormEvent| {
                info!("Event: {:?}", event.value());
                txt.set(format!("{}", event.value().to_lowercase()))
            }
        }
    }
}
