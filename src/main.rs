#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
use rocket::form::Form;
use rocket::fs::NamedFile;

use std::{fs::OpenOptions, io::Write};

#[derive(FromForm)]
struct PostMsg {
    content: String,
    username: String,
}

#[get("/")]
async fn index() -> Redirect {
    Redirect::to("/login")
}

#[post("/post_message", data = "<msg>")]
fn post_message(msg: Form<PostMsg>) -> String {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("chat.txt")
        .unwrap();

    let _stamp = chrono::offset::Utc::now().to_string();
    let timestamp = _stamp.split(".").next().unwrap();

    let username = &msg.username;
    let message = &msg.content;

    if !msg.content.is_empty() {
        writeln!(
            file,
            "<span class=\"timestamp\">{timestamp}</span> <span class=\"{username}-username username\">[{username}]</span> <a class=\"{username}-msg msgbox\">{}</a><br>",
            message
        )
        .unwrap();
    }

    return String::from("Hello, world!");
}

#[get("/login")]
async fn login() -> Option<NamedFile> {
    NamedFile::open("html/login.html").await.ok()
}

#[get("/get_chat")]
async fn get_chat() -> Option<NamedFile> {
    NamedFile::open("chat.txt").await.ok()
}

#[get("/chat")]
async fn chat() -> Option<NamedFile> {
    NamedFile::open("html/chat.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![index, login, post_message, get_chat, chat],
    )
}
