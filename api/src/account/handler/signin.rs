use crate::app::AppError;
use super::*;

#[derive(Deserialize)]
pub struct Request {
    pub login: Login,
    pub password: String,
    pub client_id: Uuid,
}

pub async fn handler(State(state): State<Arc<AppState>>, Json(req): Json<Request>) -> Response {
    let mut options = UserOptions::default();

    match req.login {
        Login::Nickname(v) => {
            options.with_nickname(v);
        }
        Login::Email(v) => {
            options.with_email(v);
        }
    }

    let user = match state.user_repo()
        .find(&options).await
        .map_err(|e| {
            match e.downcast_ref::<Fail>() {
                Some(Fail::UserNotFound) => anyhow!(Fail::InvalidCredentials),
                _ => e,
            }
        }) {
        Ok(v ) => v,
        Err(e) => {
            return AppError::from(e).into_response()
        }
    };

    if !state.passwd().verify(req.password.as_str(), user.password.as_str()) {
        return AppError::from(Fail::InvalidCredentials).into_response();
    }

    let headers = match get_new_session(state.clone(), &user, req.client_id).await {
        Ok(v) => v,
        Err(e) => return e.into_response()
    };

    (headers, StatusCode::OK).into_response()
}