use axum_macros::debug_handler;
use super::*;

// мне нужна мидлваря, которая будет чекать наличие куки sid
// пропускать указанные роуты
// добавлять экстрактор сессии

pub async fn handler(Extension(session): Extension<Session>) -> Result<Json<Session>> {
    Ok(Json(session))
    // if let Some(v) = jar.get("sid") {
    //     v.value().to_owned()
    // } else {
    //     "empty".to_string()
    // }



    // надо попробовать постучаться с отсутствующим sid
    // а потом добавить игнор для путей
}