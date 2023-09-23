mod handlers;

use handlers::post_message::post_message;

use axum::{
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::fs;

async fn index() -> Redirect {
    Redirect::to("/login")
}

async fn login() -> impl IntoResponse {
    return Html(fs::read_to_string("html/login.html").await.unwrap());
}

async fn get_chat() -> impl IntoResponse {
    return fs::read_to_string("chat.txt").await.unwrap();
}

async fn chat() -> impl IntoResponse {
    return Html(fs::read_to_string("html/chat.html").await.unwrap());
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/post_message", post(post_message))
        .route("/login", get(login))
        .route("/get_chat", get(get_chat))
        .route("/chat", get(chat))
        .nest("/static", axum_static::static_router("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
