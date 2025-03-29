use leptos::prelude::*;

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

#[server]
pub async fn shouting_text(input: String) -> Result<String, ServerFnError> {
    log::info!("Server-side fn: {}", &input);
    Ok(input.to_ascii_uppercase())
}
