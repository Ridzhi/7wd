mod ping;
mod secret;
mod signin;
mod signup;

use super::model::*;
use crate::prelude::*;
use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Extension, Json, Request, State},
    http::{
        header::{HeaderMap, HeaderName, HeaderValue, SET_COOKIE},
        StatusCode,
    },
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};

use anyhow::anyhow;
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

    req.extensions_mut().insert(session);

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

fn get_session_key(id: &str) -> String {
    format!("session:{}", id)
}
