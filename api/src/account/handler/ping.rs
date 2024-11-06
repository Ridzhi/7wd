use super::*;
pub async fn handler() -> &'static str {
    "pong"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn get_pong() {
        let app = build(Arc::new(AppState::default()));
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/ping")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
    }
}