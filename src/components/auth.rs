use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_query_map;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, RedirectUrl, TokenUrl};

#[component]
pub fn SignInPage() -> impl IntoView {
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
                <h1 class="text-2xl font-bold mb-4">"Sign In with GitHub"</h1>
                <a
                    href={auth_url.to_string()}
                    class="bg-gray-800 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded inline-flex items-center"
                >
                    <svg class="w-4 h-4 mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234
                        c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729
                        1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604
                        -2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176
                        0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404
                        2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221
                        0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576
                        4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                        />
                    </svg>
                    "Sign in with GitHub"
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn GithubOAuth2Callback() -> impl IntoView {
    let (user_info, set_user_info) = signal(String::new());
    let (user_email, set_user_email) = signal(String::new());
    let query_map = use_query_map();
    let code = query_map.with(|query_map| query_map.get("code").unwrap_or_default());

    Effect::new(move || {
        let code = code.clone();
        spawn_local(async move {
            let (user_info, user_email) = crate::server::get_github_user_info(code).await.unwrap();
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
