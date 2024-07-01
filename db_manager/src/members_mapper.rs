use crate::connector::connect;
use data_models::UserInfo;
use mysql::prelude::Queryable;

pub async fn get_members(chat_id: i32) -> anyhow::Result<Vec<UserInfo>> {
    let users = connect()?.exec_map(
        "call SimpleMessenger.GetAllMembers(:chat_id);",
        (chat_id,),
        |(user_id, name, image)| UserInfo {
            user_id,
            name,
            image,
        },
    )?;

    Ok(users)
}

pub async fn set_last_read_message(
    chat_id: i32,
    user_id: i32,
    message_id: i32,
) -> anyhow::Result<()> {
    let _users = connect()?.exec_drop(
        "call SimpleMessenger.SetLastReadedMessage(:chat_id,:user_id,:message_id);",
        (chat_id, user_id, message_id),
    )?;

    Ok(())
}

pub async fn add_member(chat_id: i32, user_id: i32) -> anyhow::Result<()> {
    connect()?
        .exec_drop(
            "call SimpleMessenger.AddChatMember(:chat_id,:user_id );",
            (chat_id, user_id),
        )
        .expect("can`t add member");

    Ok(())
}

pub async fn remove_member(chat_id: i32, user_id: i32) -> anyhow::Result<()> {
    connect()?.exec_drop(
        "call SimpleMessenger.RemoveChatMember(:chat_id, :user_id);",
        (chat_id, user_id),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use tokio;

    use super::*;

    // #[async_std::test]
    // async fn it_works() {
    //     match get_vehicles().await {
    //         Ok(a) => {
    //             for chat in a{
    //                 println!("Ok : {}", chat.name)}
    //             }
    //
    //         Err(_) => {}
    //     }
    //     assert_eq!(4, 4);
    // }
}
