mod ping;
mod secret;
mod signin;
mod signup;

use crate::prelude::*;
use super::model::*;
use std::sync::Arc;
use validator::{Validate, ValidationError, ValidateLength, ValidateRegex, ValidateEmail};
use regex::Regex;

use axum::{async_trait, body::Body, extract::{Extension, Json, Request, State}, http::{
    header::{HeaderMap, HeaderName, HeaderValue, SET_COOKIE},
    StatusCode,
}, middleware::{self, Next}, response::{IntoResponse, Response}, routing::{get, post}, Router};

use anyhow::anyhow;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::cookie::{Cookie, CookieJar};
use futures::prelude::*;
use redis::{AsyncCommands, JsonAsyncCommands};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde_derive::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tower::ServiceBuilder;
use uuid::Uuid;

// test imports
#[cfg(test)]
use tower::ServiceExt;

pub fn build(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/secret", get(secret::handler))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(state.clone(), auth)))
        .route("/cook", get(cook))
        .route("/signup", post(signup::handler))
        .route("/ping", get(ping::handler))
        .route("/signin", post(signin::handler))
        .with_state(state)
}

async fn cook() -> Response {
    let mut headers = HeaderMap::new();
    let cookie = Cookie::build(("secret", "love"))
        // .domain(state.config().host.clone())
        // .path("/")
        // .http_only(true)
        // .secure(true)
        .build();
    
    headers.insert(SET_COOKIE, cookie.to_string().parse().unwrap());
    (headers, StatusCode::OK).into_response()
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
struct Session {
    pub session_id: Uuid,
    pub user_id: UserId,
    pub client_id: Uuid,
    pub created_at: i64,
}

impl Session {
    pub const TTL_DAYS: u8 = 90;
    pub const KEY: &'static str = "sid";
}

struct ExtractUser(User);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractUser
where
    S: Send + Sync
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        unimplemented!()
    }
}

async fn auth(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let jar = CookieJar::from_headers(req.headers());

    let sid = if let Some(v) = jar.get(Session::KEY) {
        v.value()
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let mut rds = if let Ok(v) = state.rds().get_multiplexed_async_connection().await {
        v
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let session: Session = if let Ok(v) = rds.json_get(get_session_key(sid), "$").await {
        v
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let user = state.user_repo().find(&session.user_id).await.unwrap().unwrap();
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

pub async fn create_session(
    state: Arc<AppState>,
    user: &User,
    client_id: Uuid,
) -> Result<HeaderMap> {
    // @TODO удалить старые?

    let session = Session {
        session_id: Uuid::new_v4(),
        user_id: user.id,
        client_id,
        created_at: OffsetDateTime::now_utc().unix_timestamp(),
    };

    let key = get_session_key(&session.session_id.to_string());

    let mut rds = state.rds().get_multiplexed_async_connection().await?;

    rds.json_set(&key, "$", &session).await?;
    rds.expire(
        &key,
        Duration::days(Session::TTL_DAYS as i64).as_seconds_f64() as i64,
    )
    .await?;

    let cookie = Cookie::build((Session::KEY, session.session_id.to_string()))
        .domain(state.config().host.clone())
        .path("/")
        .http_only(true)
        // @TODO fix with rapid api resolving
        // .secure(true)
        .build();

    let mut headers = HeaderMap::new();

    headers.insert(SET_COOKIE, cookie.to_string().parse()?);

    Ok(headers)
}

pub fn validate_login(login: &Login) -> Result<(), ValidationError> {
    match login {
        Login::Nickname(v) => {
            if !v.validate_length(Some(3), Some(15), None) {
                return Err(ValidationError::new("invalid nickname length"));
            }

            if !v.validate_regex(Regex::new(r"^[a-zA-Z]+[a-zA-Z0-9]*$").unwrap()) {
                return Err(ValidationError::new("nickname should starts from letter and contains letters/numbers"));
            }

            Ok(())
        },
        Login::Email(v) => {
            if !v.validate_email() {
                return Err(ValidationError::new("invalid email format"));
            }

            Ok(())
        },
    }
}

fn get_session_key(id: &str) -> String {
    format!("session:{}", id)
}

