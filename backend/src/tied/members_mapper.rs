use data_models::UserInfo;
use db_manager::chat_mapper::get_chat;
use db_manager::members_mapper;
use futures_util::future;
use serde::Serialize;

use crate::messages_stream::Message;
use crate::STREAM_ARRAY;
use crate::tied::chat_mapper::{send_add_chat, send_remove_chat};

const ADD_MEMBER: &str = "add member";
const REMOVE_MEMBER: &str = "remove member";

#[derive(Serialize)]
struct MemberChange {
    pub chat_id: i32,
    pub user_id: i32,
}

impl MemberChange {
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub async fn get_members(chat_id: i32) -> anyhow::Result<Vec<UserInfo>> {
    members_mapper::get_members(chat_id).await
}
pub async fn set_last_read_message(
    chat_id: i32,
    user_id: i32,
    message_id: i32,
) -> anyhow::Result<()> {
    members_mapper::set_last_read_message(chat_id, user_id, message_id).await
}
pub async fn add_member(chat_id: i32, user_id: i32) -> anyhow::Result<()> {
    let result = members_mapper::add_member(chat_id, user_id).await;

    if let Err(_) = result {
        return result;
    }

    let _result = add_or_remove_member(chat_id, user_id, ADD_MEMBER).await;

    send_add_chat(user_id, &get_chat(chat_id).await.unwrap()).await;

    return result;
}
pub async fn remove_member(chat_id: i32, user_id: i32) -> anyhow::Result<()> {
    let result = members_mapper::remove_member(chat_id, user_id).await;

    if let Err(_) = result {
        return result;
    }

    let _result = add_or_remove_member(chat_id, user_id, REMOVE_MEMBER).await;

    send_remove_chat(user_id, chat_id).await;

    return result;
}

pub async fn add_or_remove_member(
    chat_id: i32,
    user_id: i32,
    add_or_remove: &'static str,
) -> Vec<()> {
    let result: Vec<_> = members_mapper::get_members(chat_id)
        .await
        .unwrap()
        .iter()
        .filter(|member| member.user_id != user_id)
        .map(|member| unsafe {
            STREAM_ARRAY.send(
                member.user_id,
                Message {
                    message_type: add_or_remove.to_string(),
                    message: MemberChange { chat_id, user_id }.serialize(),
                },
            )
        })
        .collect();
    future::join_all(result).await
}
