use axum::{
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use std::{fs::OpenOptions, io::Write, net::SocketAddr};
use tokio::fs;

#[derive(TryFromMultipart)]
struct PostMsg {
    username: String,
    content: String,
}

async fn index() -> Redirect {
    Redirect::to("/login")
}

async fn post_message(
    TypedMultipart(PostMsg { username, content }): TypedMultipart<PostMsg>,
) -> impl IntoResponse {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("chat.txt")
        .unwrap();

    let stamp = chrono::offset::Utc::now().to_string();
    let timestamp = stamp.split(".").next().unwrap();

    if !content.is_empty() {
        writeln!(
            file,
            "<span class=\"timestamp\">{timestamp}</span> <span class=\"{username}-username username\">[{username}]</span> <a class=\"{username}-msg msgbox\">{}</a><br>",
            content
        ).unwrap();
    }

    return "ok";
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
