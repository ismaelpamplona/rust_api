pub mod articles;

use axum::response::IntoResponse;

pub async fn get_root() -> impl IntoResponse {
    "Welcome to the API!".to_string()
}
