use data_models::Message;
use db_manager::message_mapper;
use futures_util::future;

use crate::STREAM_ARRAY;

const MESSAGE: &str = "message";

trait MySerialize {
    fn serialize(&self) -> String;
}
impl MySerialize for Message {
    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
pub async fn get_messages(chat_id: i32) -> anyhow::Result<Vec<Message>> {
    message_mapper::get_messages(chat_id).await
}

pub async fn send_message(sender_id: i32, chat_id: i32, message: &str) -> anyhow::Result<Message> {
    let message = message_mapper::send_message(sender_id, chat_id, message).await;

    if let Err(_) = message {
        return message;
    }

    let message = message.unwrap();

    let members = db_manager::members_mapper::get_members(chat_id)
        .await
        .unwrap();

    let sending: Vec<_> = members
        .iter()
        .map(|member| unsafe {
            STREAM_ARRAY.send(
                member.user_id,
                crate::messages_stream::Message {
                    message_type: MESSAGE.to_string(),
                    message: MySerialize::serialize(&message),
                },
            )
        })
        .collect();
    future::join_all(sending).await;

    return Ok(message);
}
