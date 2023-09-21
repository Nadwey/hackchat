#[macro_use]
extern crate rocket;

use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::{form::Form, tokio::fs::read_to_string};

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

#[get("/chdata")]
async fn chdata() -> String {
    read_to_string("chat.txt").await.unwrap()
}

#[get("/login")]
async fn login() -> RawHtml<String> {
    return RawHtml(format!(
        "{}",
        read_to_string("html/login.html").await.unwrap()
    ));
}

#[get("/get_chat")]
async fn get_chat() -> RawHtml<String> {
    return RawHtml(format!(
        "{}",
        read_to_string("chat.txt").await.unwrap()
    ));
}

#[get("/chat")]
async fn chat() -> RawHtml<String> {
    return RawHtml(format!(
        "{}",
        read_to_string("html/chat.html").await.unwrap()
    ));
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![login])
        .mount("/", routes![post_message])
        .mount("/", routes![chdata])
        .mount("/", routes![get_chat])
        .mount("/", routes![chat])
}
