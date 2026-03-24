use axum::{routing::get, Router}; // Used to build routes in Axum.
use leptos::*;
use tokio::main;

#[tokio::main]
async fn main() {
    // Setup Axum router
    let app = Router::new().route("/", get(root_handler));

    // Run the Axum server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// A simple Axum handler
async fn root_handler() -> &'static str {
    "Hello from Axum and Leptos!"
}
