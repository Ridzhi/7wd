use crate::prelude::*;
use super::*;

#[derive(Deserialize, Validate)]
pub struct Request {
    #[validate(custom(function = "validate_login"))]
    pub login: Login,
    #[validate(custom(function = "validate_password"))]
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

    let user = match state.user_repo().find_with(&options).await {
        Ok(user) => match user {
            Some(user) => user,
            None => return AppError::from(anyhow!(ErrorKind::InvalidCredentials)).into_response(),
        },
        Err(e) => {
            return AppError::from(e).into_response();
        }
    };

    if !state.passwd().verify(req.password.as_str(), user.password.as_str()) {
        return AppError::from(ErrorKind::InvalidCredentials).into_response();
    }

    let headers = match create_session(state.clone(), &user, req.client_id).await {
        Ok(v) => v,
        Err(e) => return e.into_response()
    };

    (headers, StatusCode::OK).into_response()
}