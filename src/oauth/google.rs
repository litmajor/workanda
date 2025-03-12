use oauth2::{ 
    basic::BasicClient, 
    AuthUrl, 
    ClientId, 
    ClientSecret, 
    RedirectUrl, 
    TokenUrl 
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct GoogleOAuth {
    client: BasicClient,
}

impl GoogleOAuth {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_url: String
    ) -> Self {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
            Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap())
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

        Self { client }
    }

    pub fn get_authorize_url(&self) -> String {
        self.client
            .authorize_url(oauth2::CsrfToken::new_random)
            .add_scope(oauth2::Scope::new("email".to_string()))
            .add_scope(oauth2::Scope::new("profile".to_string()))
            .url()
            .to_string()
    }

    pub async fn exchange_code(&self, code: String) -> Result<GoogleUser, OAuthError> {
        let token = self.client
            .exchange_code(oauth2::AuthorizationCode::new(code))
            .request_async(oauth2::reqwest::async_http_client)
            .await?;

        let client = reqwest::Client::new();
        let user_info = client
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(token.access_token().secret())
            .send()
            .await?
            .json::<GoogleUserInfo>()
            .await?;

        Ok(GoogleUser::from(user_info))
    }
}

#[derive(Deserialize)]
struct GoogleUserInfo {
    sub: String,
    email: String,
    name: String,
    picture: Option<String>,
}

#[derive(Serialize)]
pub struct GoogleUser {
    pub oauth_id: String,
    pub email: String,
    pub name: String,
    pub avatar: Option<String>,
}
