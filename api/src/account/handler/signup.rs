use crate::prelude::*;
use super::*;

#[derive(Deserialize, Validate)]
pub struct Request {
    #[validate(custom(function = "validate_email"))]
    pub email: Email,
    #[validate(custom(function = "validate_nickname"))]
    pub nickname: Nickname,
    #[validate(custom(function = "validate_password"))]
    pub password: String,
    pub client_id: Uuid,
}

pub async fn handler(State(state): State<Arc<AppState>>, Json(req): Json<Request>) -> Response {
    let mut o = UserOptions::default();

    if state.user_repo().find_with(o.with_email(req.email.clone())).await.is_ok() {
        return AppError::from(ErrorKind::EmailAlreadyInUse).into_response();
    };

    if state.user_repo().find_with(o.with_nickname(req.nickname.clone())).await.is_ok() {
        return AppError::from(ErrorKind::NicknameAlreadyInUse).into_response();
    };

    let password = match state.passwd().hash(req.password) {
        Ok(v) => v,
        Err(e) => return e.into_response()
    };

    let u = User {
        id: 0,
        nickname: req.nickname,
        rating: STARTING_RATING,
        email: req.email,
        password,
        settings: Settings::default(),
        created_at: UtcDateTime::default(),
    };

    let user = match state.user_repo().save(u).await {
        Ok(v) => v,
        Err(e) => return AppError::from(e).into_response()
    };

    let headers = match create_session(state.clone(), &user, req.client_id).await {
        Ok(v) => v,
        Err(e) => return e.into_response()
    };

    (headers, StatusCode::OK).into_response()

    // @TODO онбоардинг емейл
}