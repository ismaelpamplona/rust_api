use axum::{routing::get, Router};

use super::handlers;
use super::routes::articles;

#[tokio::main]
pub async fn run() {
    println!("running");
    let app = Router::new()
        .route("/", get(handlers::get_root))
        .nest("/articles", articles::routes());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
