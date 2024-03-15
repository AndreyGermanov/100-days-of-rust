use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get, Router
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use chrono::{Datelike};

use twitter_v2::authorization::{Oauth2Client, Oauth2Token, Scope};
use twitter_v2::oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier};
use twitter_v2::TwitterApi;

pub struct Oauth2Ctx {
    client: Oauth2Client,
    verifier: Option<PkceCodeVerifier>,
    state: Option<CsrfToken>,
    token: Option<Oauth2Token>,
}

async fn login(Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>) -> impl IntoResponse {
    let mut ctx = ctx.lock().unwrap();

    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();

    let (url, state) = ctx.client.auth_url(
        challenge,
        [
            Scope::TweetRead,
            Scope::TweetWrite,
            Scope::UsersRead,
            Scope::OfflineAccess,
        ],
    );

    ctx.verifier = Some(verifier);
    ctx.state = Some(state);

    Redirect::to(url.to_string().as_str())
}

#[derive(Deserialize)]
pub struct CallbackParams {
    code: AuthorizationCode,
    state: CsrfToken,
}

async fn callback(
    Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>,
    Query(CallbackParams { code, state }): Query<CallbackParams>,
) -> impl IntoResponse {
    let (client, verifier) = {
        let mut ctx = ctx.lock().unwrap();

        let saved_state = ctx.state.take().ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No previous state found".to_string(),
            )
        })?;

        if state.secret() != saved_state.secret() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Invalid state returned".to_string(),
            ));
        }

        let verifier = ctx.verifier.take().ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No PKCE verifier found".to_string(),
            )
        })?;
        let client = ctx.client.clone();
        (client, verifier)
    };

    let token = client
        .request_token(code, verifier)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    ctx.lock().unwrap().token = Some(token);

    Ok(Redirect::to("/tweets"))
}

async fn level(Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>) -> impl IntoResponse {
    let (mut oauth_token, oauth_client) = {
        let ctx = ctx.lock().unwrap();
        let token = ctx
            .token
            .as_ref()
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "User not logged in!".to_string()))?
            .clone();
        let client = ctx.client.clone();
        (token, client)
    };

    if oauth_client
        .refresh_token_if_expired(&mut oauth_token)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        ctx.lock().unwrap().token = Some(oauth_token.clone());
    }
    let api = TwitterApi::new(oauth_token);
    let user_data = api.get_user_by_username("germanov_dev").send().await.unwrap().into_data().unwrap();
    let mut score = 0_i32;
    if user_data.verified.unwrap_or_default() {
        score -= 1;
    }
    if user_data.profile_image_url.is_none() {
        score += 1;
    }
    if user_data.description.unwrap().trim() == "" {
        score += 1;
    }
    for word in ["patriota", "bolsonaro", "direita", "familia", "usa", "bolsonarista", "17", "mito"] {
        if user_data.name.to_lowercase().contains(word) {
            score +=1;
            break;
        }
    };

    let folowers = api.get_list_followers(user_data.id).send().await.unwrap().data.clone().unwrap();
    if folowers.len() <= 50 || folowers.len() == 2001 {
        score +=1;
    }

    let tweet1 = api.get_tweet(0).send().await.unwrap();
    let tweet2 = api.get_tweet(1).send().await.unwrap();

    if tweet1.data.clone().unwrap().text == tweet2.data.clone().unwrap().text {
        score +=1;
    }
    if tweet1.data.clone().unwrap().created_at.unwrap().date().year() < chrono::Utc::now().year() {
        score +=1;
    }

    let result =
        if score <= 20 {
            "Not a cattle"
        } else if score >=20 && score <= 80 {
            "Sounds like a cattle (muhhhh)"
        } else if score > 80 && score <=100 {
            "It's definitely a cattle ! Avoid it !"
        } else {
            "Too much cattle! 🐮"
    }.to_string();

    println!("Result: {}", result);

    Ok::<_, (StatusCode, String)>(result)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let oauth_ctx = Oauth2Ctx {
        client: Oauth2Client::new(
            std::env::var("CLIENT_ID").expect("could not find CLIENT_ID"),
            std::env::var("CLIENT_SECRET").expect("could not find CLIENT_SECRET"),
            format!("http://{addr}/callback").parse().unwrap(),
        ),
        verifier: None,
        state: None,
        token: None,
    };

    let app = Router::new()
        .route("/login", get(login))
        .route("/callback", get(callback))
        .route("/level", get(level))
        .layer(Extension(Arc::new(Mutex::new(oauth_ctx))));

    println!("\nOpen http://{}/login in your browser\n", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
