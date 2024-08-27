use leptos::*;
use leptos_meta::*;
use leptos_router::use_query_map;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Exercise {
    pub segments: Vec<Segment>,
    pub english: String,
}

#[allow(unused)]
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

#[server]
pub async fn shouting_text(input: String) -> Result<String, ServerFnError> {
    log::info!("Server-side fn: {}", &input);
    Ok(input.to_ascii_uppercase())
}

#[component]

pub fn SizedInput() -> impl IntoView {
    let (value, set_value) = create_signal(String::new());
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
                        }.into_view()
                    } else {
                        view! { {segment.1.chinese.to_string()} }.into_view()
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

use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet href="/pkg/style.css" />
        <Link rel="icon" type_="image/x-icon" href="/pkg/favicon.ico" />
        <Router>
            <Routes>
                <Route path="/" view=StartPage />
                <Route path="/signin" view=SignInPage />
                <Route path="/oauth/github" view=GithubOAuth2Callback />
            </Routes>
        </Router>
    }
}

#[server]
pub async fn get_github_user_info(code: String) -> Result<(String, String), ServerFnError> {
    use oauth2::reqwest::async_http_client;
    use oauth2::TokenResponse;
    use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
    use reqwest::Client;
    use send_wrapper::SendWrapper;

    SendWrapper::new(async move {
        let client_secret = std::env::var("GITHUB_OAUTH_SECRET")
            .ok()
            .ok_or_else(|| ServerFnError::new("GITHUB_OAUTH_SECRET not set"))?;
        let client = BasicClient::new(
            ClientId::new("Ov23li9etcgfYorMCgM1".to_string()),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap(),
            Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap()),
        )
        .set_redirect_uri(
            RedirectUrl::new("http://127.0.0.1:6767/oauth/github".to_string()).unwrap(),
        );

        let token_result = client
            .exchange_code(oauth2::AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(ServerFnError::new)?;

        let access_token = token_result.access_token().secret();

        let client = Client::new();
        let user_info = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "Erudify")
            .send()
            .await
            .map_err(ServerFnError::new)?
            .text()
            .await
            .map_err(ServerFnError::new)?;

        let user_email = client
            .get("https://api.github.com/user/emails")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "Erudify")
            .send()
            .await
            .map_err(ServerFnError::new)?
            .text()
            .await
            .map_err(ServerFnError::new)?;

        Ok((user_info, user_email))
    })
    .await
}

#[component]
fn GithubOAuth2Callback() -> impl IntoView {
    use leptos::*;

    let (user_info, set_user_info) = create_signal(String::new());
    let (user_email, set_user_email) = create_signal(String::new());
    let query_map = use_query_map();
    let code = query_map.with(|query_map| query_map.get("code").cloned().unwrap_or_default());

    create_effect(move |_| {
        let code = code.clone();
        spawn_local(async move {
            let (user_info, user_email) = get_github_user_info(code).await.unwrap();

            set_user_info.set(user_info);

            set_user_email.set(user_email);
        });
    });

    view! {
        <div>
            <h2>"Github OAuth2 Callback"</h2>
            <p>"User Info: " {user_info}</p>
            <p>"User Email: " {user_email}</p>
        </div>
    }
}

#[component]
fn StartPage() -> impl IntoView {
    view! {
        <p>Leptos</p>
        <button on:click=move |_| {
            spawn_local(async move {
                let val = shouting_text(String::from("test")).await.ok();
                log::info!("Got answer from server: {:?}", val);
            });
        }>Test</button>
        <AudioExercise exercise=ex_2() index=1 />
    }
}

#[component]
fn SignInPage() -> impl IntoView {
    use leptos::*;
    use oauth2::{basic::BasicClient, AuthUrl, ClientId, RedirectUrl, TokenUrl};

    let client_id = ClientId::new("Ov23li9etcgfYorMCgM1".to_string());
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");

    let client = BasicClient::new(client_id, None, auth_url, Some(token_url)).set_redirect_uri(
        RedirectUrl::new("http://127.0.0.1:6767/oauth/github".to_string())
            .expect("Invalid redirect URL"),
    );

    let (auth_url, _csrf_token) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("user:email".to_string()))
        .url();

    view! {
        <div class="flex flex-col items-center justify-center min-h-screen bg-gray-100">
            <div class="p-6 bg-white rounded shadow-md">
                <h1 class="text-2xl font-bold mb-4">Sign In with GitHub</h1>
                <a
                    href={auth_url.to_string()}
                    class="bg-gray-800 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded inline-flex items-center"
                >
                    <svg class="w-4 h-4 mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                    </svg>
                    Sign in with GitHub
                </a>
            </div>
        </div>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

#[cfg(feature = "ssr")]
mod ssr_imports {
    use crate::App;
    use axum::http::{HeaderValue, StatusCode};
    use axum::{
        extract::Path,
        response::IntoResponse,
        routing::{get, post},
        Router,
    };
    use include_dir::{include_dir, Dir};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use worker::{event, Context, Env, HttpRequest, Result};

    static PKG_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/pkg/");

    async fn serve_static(Path(path): Path<String>) -> impl IntoResponse {
        let mime_type = mime_guess::from_path(&path).first_or_text_plain();
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_str(mime_type.as_ref()).unwrap(),
        );
        match PKG_DIR.get_file(path) {
            None => (StatusCode::NOT_FOUND, headers, "File not found.".as_bytes()),
            Some(file) => (StatusCode::OK, headers, file.contents()),
        }
    }

    fn router() -> Router {
        let leptos_options = LeptosOptions::builder()
            .output_name("client")
            .site_pkg_dir("pkg")
            .build();
        let routes = generate_route_list(App);

        // build our application with a route
        let app: axum::Router<()> = Router::new()
            .leptos_routes(&leptos_options, routes, App)
            .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
            .route("/pkg/*file_name", get(serve_static))
            .with_state(leptos_options);
        app
    }

    #[event(start)]
    fn register() {
        server_fn::axum::register_explicit::<super::ShoutingText>();
        server_fn::axum::register_explicit::<super::GetGithubUserInfo>();
    }

    #[event(fetch)]
    async fn fetch(
        req: HttpRequest,
        _env: Env,
        _ctx: Context,
    ) -> Result<axum::http::Response<axum::body::Body>> {
        _ = console_log::init_with_level(log::Level::Debug);
        use tower_service::Service;

        console_error_panic_hook::set_once();

        Ok(router().call(req).await?)
    }
}
