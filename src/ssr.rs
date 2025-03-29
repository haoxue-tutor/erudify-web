use crate::app::App;
use axum::Extension;
use axum::{routing::post, Router};
use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos_meta::MetaTags;
use std::sync::Arc;
use worker::{event, Context, Env, HttpRequest, Result};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
      <!DOCTYPE html>
      <html lang="en">
        <head>
          <title>Erudify</title>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />

          <AutoReload options=options.clone() />
          <HydrationScripts options />
          <MetaTags />
        </head>
        <body class="bg-gray-100">
          <App />
        </body>
      </html>
    }
}

fn router(env: Env) -> Router {
    let leptos_options = LeptosOptions::builder()
        .output_name("client")
        .site_pkg_dir("pkg")
        .build();
    let routes = generate_route_list(App);

    // build our application with a route
    let app: axum::Router<()> = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .with_state(leptos_options)
        .layer(Extension(Arc::new(env)));
    app
}

#[event(start)]
fn register() {
    use crate::server::*;
    server_fn::axum::register_explicit::<ShoutingText>();
    server_fn::axum::register_explicit::<GetGithubUserInfo>();
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    _ = console_log::init_with_level(log::Level::Debug);
    use tower_service::Service;

    console_error_panic_hook::set_once();

    Ok(router(env).call(req).await?)
}
