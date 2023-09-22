use std::{io::Write, fs::OpenOptions};
use axum::response::IntoResponse;
use axum_typed_multipart::{TypedMultipart, TryFromMultipart};

#[derive(TryFromMultipart)]
pub struct PostMsg {
    username: String,
    content: String,
}

pub async fn post_message(
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
