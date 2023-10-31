use crate::handlers::articles;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(articles::list))
}
