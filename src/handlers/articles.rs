use axum::response::IntoResponse;

pub async fn list() -> impl IntoResponse {
    "Articles List!".to_string()
}
