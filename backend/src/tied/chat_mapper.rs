use data_models::Chat;
use db_manager::{chat_mapper, members_mapper};
use futures_util::future;
use rocket::serde::Serialize;

use crate::messages_stream::Message;
use crate::STREAM_ARRAY;

const ADD_CHAT: &str = "add chat";
const REMOVE_CHAT: &str = "remove chat";
const UPDATE_CHAT: &str = "update chat";

pub async fn send_add_chat(user_id: i32, chat: &Chat) {
    unsafe {
        STREAM_ARRAY
            .send(
                user_id,
                Message {
                    message_type: ADD_CHAT.to_string(),
                    message: serde_json::to_string(chat).unwrap(),
                },
            )
            .await;
    }
}

pub async fn send_remove_chat(user_id: i32, chat_id: i32) {
    unsafe {
        STREAM_ARRAY
            .send(
                user_id,
                Message {
                    message_type: REMOVE_CHAT.to_string(),
                    message: chat_id.to_string(),
                },
            )
            .await;
    }
}
pub async fn create_chat(owner_id: i32, name: &str, image: Option<&str>) -> anyhow::Result<Chat> {
    let chat = chat_mapper::create_chat(owner_id, name, image).await;

    if let Ok(chat) = chat {
        send_add_chat(owner_id, &chat).await;
        return Ok(chat);
    };

    chat
}
pub async fn delete_chat(owner_id: i32, chat_id: i32) -> anyhow::Result<()> {
    let chat = chat_mapper::delete_chat(owner_id, chat_id).await;

    if let Ok(_) = chat {
        send_remove_chat(owner_id, chat_id).await;
    };

    chat
}

pub async fn update_chat(chat_id: i32, name: &str, image: Option<&str>) -> anyhow::Result<Chat> {
    let chat = match image {
        Some(image) => chat_mapper::update_chat(chat_id, name, image).await,
        None => chat_mapper::update_chat_name(chat_id, name).await,
    };

    if let Ok(chat) = chat {
        let cloned_chat = chat.clone();
        send_update_chat(&ChatChange {
            id: chat.id,
            owner_id: chat.owner_id,
            name: chat.name,
            image: chat.image,
        })
        .await;

        return Ok(cloned_chat);
    };

    return chat;
}

pub async fn get_chats(user_id: i32) -> anyhow::Result<Vec<Chat>> {
    chat_mapper::get_chats(user_id).await
}

#[derive(Serialize)]
pub struct ChatChange {
    pub id: i32,
    pub owner_id: i32,
    pub name: String,
    pub image: String,
}
impl ChatChange {
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub async fn send_update_chat(chat: &ChatChange) -> Vec<()> {
    let result: Vec<_> = members_mapper::get_members(chat.id)
        .await
        .unwrap()
        .iter()
        .map(|member| unsafe {
            STREAM_ARRAY.send(
                member.user_id,
                Message {
                    message_type: UPDATE_CHAT.to_string(),
                    message: chat.serialize(),
                },
            )
        })
        .collect();
    future::join_all(result).await
}
