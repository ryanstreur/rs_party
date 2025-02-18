//! Main module
//!

use std::sync::Arc;

use axum::{routing::get, Router};

use rs_party::{
    db::get_pool,
    routes::{root_handler, AppState},
};

#[tokio::main]
async fn main() {
    let db = get_pool().await.expect("Could not connect to database");

    let app_state = Arc::new(AppState { db });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let app = Router::new()
        .route("/", get(root_handler))
        .with_state(app_state);

    axum::serve(listener, app).await.unwrap();
}
