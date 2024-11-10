use super::*;

// мне нужна мидлваря, которая будет чекать наличие куки sid
// пропускать указанные роуты
// добавлять экстрактор сессии

pub async fn handler(Extension(user): Extension<User>) -> Result<Json<User>> {
    Ok(Json(user))
    // if let Some(v) = jar.get("sid") {
    //     v.value().to_owned()
    // } else {
    //     "empty".to_string()
    // }



    // надо попробовать постучаться с отсутствующим sid
    // а потом добавить игнор для путей
}