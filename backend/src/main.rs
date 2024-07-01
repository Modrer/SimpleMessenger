#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use once_cell::sync::Lazy;
use rocket::data::{Limits, ToByteUnit};
use rocket::fs::{FileServer};

use crate::handlers::{authorization, chat, contact, members, messages, user};

mod claims;
mod file_saver;
pub mod handlers;
pub mod messages_stream;
mod response;
pub(crate) mod streams_array;
mod tied;

static mut STREAM_ARRAY: Lazy<streams_array::StreamsArray> =
    Lazy::new(|| streams_array::StreamsArray::new());

// #[get("/<id>")]
// pub async fn index(id: i32) -> &'static str {
//
//     unsafe{
//         STREAM_ARRAY.send(id, Message{
//             message_type: "Test get".to_string(),
//             message: "Message".to_string(),
//
//         })
//     }.await;
//
//     // println!("{:?}", claims);
//     "Hello, world!"
// }
//
// #[get("/")]
// pub async fn index() ->  Option<NamedFile> {
//     let mut path = Path::new(relative!("static")).join("file");
//     if path.is_dir() {
//         path.push("index. html");
//     }
//
//     NamedFile::open(path).await.ok().unwrap().metadata().await.unwrap().
// }

#[launch]
fn rocket() -> _ {
    dotenv().unwrap();

    let figment = rocket::Config::figment().merge(("port", 8000)).merge((
        "limits",
        Limits::default().limit("file/jpeg", 5.mebibytes()),
    ));
    rocket::custom(figment)
        //.mount("/", routes![index])
        //.mount("/test/", routes![index])
        .mount(authorization::PATH, authorization::routes())
        .mount(chat::PATH, chat::routes())
        .mount(contact::PATH, contact::routes())
        .mount(members::PATH, members::routes())
        .mount(messages::PATH, messages::routes())
        .mount(user::PATH, user::routes())
        .mount("/ws/", routes![handlers::ws::hello])
        .mount("/images", FileServer::from("./static"))
    //.attach(AdHoc::config::<Config>())
}

//
// use std::collections::HashMap;
// use std::convert::Infallible;
// use std::sync::Arc;
// use tokio::sync::{mpsc, Mutex};
// use warp::{Filter, Rejection};
// use warp::ws::Message;
// use data_models;
// #[derive(Clone)]
// pub struct Client {
//     pub user_id: usize,
//     pub topics: Vec<String>,
//     pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
// }
//
// #[derive(serde::Deserialize, serde::Serialize)]
// pub struct RegisterRequest {
//     user_id: usize,
// }
//
// #[derive(serde::Deserialize, serde::Serialize)]
// pub struct RegisterResponse {
//     url: String,
// }
//
// #[derive(serde::Deserialize, serde::Serialize)]
// pub struct Event {
//     topic: String,
//     user_id: Option<usize>,
//     message: String,
// }
//
// #[derive(serde::Deserialize, serde::Serialize)]
// pub struct TopicsRequest {
//     topics: Vec<String>,
// }
//
// mod handler;
// mod ws;
//
// type Result<T> = std::result::Result<T, Rejection>;
// type Clients = Arc<Mutex<HashMap<String, Client>>>;
//
// #[tokio::main]
// async fn main() {
//     let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
//
//     let health_route = warp::path!("health").and_then(handler::health_handler);
//
//     let register = warp::path("register");
//     let register_routes = register
//         .and(warp::post())
//         .and(warp::body::json())
//         .and(with_clients(clients.clone()))
//         .and_then(handler::register_handler)
//         .or(register
//             .and(warp::delete())
//             .and(warp::path::param())
//             .and(with_clients(clients.clone()))
//             .and_then(handler::unregister_handler));
//
//     let publish = warp::path!("publish")
//         .and(warp::body::json())
//         .and(with_clients(clients.clone()))
//         .and_then(handler::publish_handler);
//
//     let ws_route = warp::path("ws")
//         .and(warp::ws())
//         .and(warp::path::param())
//         .and(with_clients(clients.clone()))
//         .and_then(handler::ws_handler);
//
//     let ROUTES = health_route
//         .or(register_routes)
//         .or(ws_route)
//         .or(publish)
//         .with(warp::cors().allow_any_origin());
//
//     warp::serve(ROUTES).run(([127, 0, 0, 1], 8000)).await;
// }
//
// fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
//     warp::any().map(move || clients.clone())
// }
