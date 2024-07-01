use rocket::Route;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

use crate::claims::Claims;
use crate::response::Response;
use crate::tied::message_mapper;

pub const PATH: &str = "/Messages";
pub fn routes() -> Vec<Route> {
    routes![get_messages, send_message]
}

#[derive(Serialize, Deserialize)]
pub struct MessageInput {
    pub chat_id: i32,
    pub message: String,
}
#[get("/<chat_id>", format = "json")]
pub async fn get_messages(_user: Claims, chat_id: i32) -> Response {
    let messages = message_mapper::get_messages(chat_id).await;

    if let Err(_) = messages {
        //println!("{:?}", i);
        return Response::Err(());
    }
    let messages = messages.unwrap();
    Response::Ok(serde_json::to_string(&messages).unwrap())
}

#[post("/", format = "json", data = "<input>")]
pub async fn send_message(user: Claims, input: Json<MessageInput>) -> Response {
    let message = message_mapper::send_message(user.id, input.chat_id, &input.message).await;

    if let Err(i) = message {
        println!("{:?}", i);
        return Response::Err(());
    }
    //let message = message.unwrap();
    Response::Ok("Ok".to_string())
}
