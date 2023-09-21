#[macro_use]
extern crate rocket;

use rocket::response::content::RawHtml;
use rocket::{form::Form, tokio::fs::read_to_string};

use std::{fs::OpenOptions, io::Write};

#[derive(FromForm)]
struct PostMsg {
    content: String,
}

#[post("/chat/<username>", data = "<msg>")]
async fn post_message(username: String, msg: Form<PostMsg>) -> RawHtml<String> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("chat.txt")
        .unwrap();
    let _stamp = chrono::offset::Utc::now().to_string();
    let timestamp = _stamp.split(".").next().unwrap();
    if !msg.content.is_empty() {
        writeln!(
            file,
            "<span class=\"timestamp\">{timestamp}</span> <span class=\"{username}-username username\">[{username}]</span> <a class=\"{username}-msg msgbox\">{}</a><br>",
            msg.content
        )
        .unwrap();
    }
    return RawHtml(format!(
        "ok"
    ));
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
        .mount("/", routes![login])
        .mount("/", routes![post_message])
        .mount("/", routes![chdata])
        .mount("/", routes![get_chat])
        .mount("/", routes![chat])
}
