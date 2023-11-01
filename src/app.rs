use super::handlers;
use super::routes::articles;
use axum::{routing::get, Router};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
pub async fn run() {
    println!("running");

    // Load environment variables
    dotenv().ok();

    // Database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // // Attempt to connect to the database
    // match PgPool::connect(&database_url).await {
    //     Ok(pool) => println!("Successfully connected to the database"),
    //     Err(err) => println!("Failed to connect to the database: {:?}", err),
    // }

    let app = Router::new()
        .route("/", get(handlers::get_root))
        .nest("/articles", articles::routes());

    // Retrieve the port number from the environment variables
    let port: u16 = env::var("APP_PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be a valid u16");

    // Run the app
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
