use crate::connector::connect;
use data_models::Chat;
use mysql::prelude::Queryable;

pub async fn get_chats(user_id: i32) -> anyhow::Result<Vec<Chat>> {
    let chats = connect()?.exec_map(
        "call SimpleMessenger.GetAllChats(:user_id);",
        (user_id,),
        |(id, name, owner_id, image, last_read_message_id)| Chat {
            id,
            owner_id,
            name,
            image,
            last_read_message_id,
        },
    )?;

    Ok(chats)
}
pub async fn get_chat(chat_id: i32) -> anyhow::Result<Chat> {
    let chat = connect()?
        .exec_map(
            "call SimpleMessenger.GetChat(:id);",
            (chat_id,),
            |(id, name, owner_id, image)| Chat {
                id,
                owner_id,
                name,
                image,
                last_read_message_id: 0,
            },
        )?
        .first()
        .expect("not found")
        .to_owned();

    Ok(chat)
}
pub async fn create_chat(owner_id: i32, name: &str, image: Option<&str>) -> anyhow::Result<Chat> {
    let image = image.unwrap_or_else(|| "default.png");

    let chat = connect()?
        .exec_map(
            "call SimpleMessenger.CreateChat(:name, :owner_id, :image);",
            (name, owner_id, image),
            |(id, name, owner_id, image)| Chat {
                id,
                owner_id,
                name,
                image,
                last_read_message_id: 0,
            },
        )?
        .first()
        .expect("request return no chat")
        .to_owned();

    Ok(chat)
}

pub async fn update_chat(chat_id: i32, name: &str, image: &str) -> anyhow::Result<Chat> {
    let chat = connect()?
        .exec_map(
            "call SimpleMessenger.UpdateChat(:id, :name, :image);",
            (chat_id, name, image),
            |(id, name, owner_id, image)| Chat {
                id,
                owner_id,
                name,
                image,
                last_read_message_id: 0,
            },
        )?
        .first()
        .expect("request return no chat")
        .to_owned();

    Ok(chat)
}
pub async fn update_chat_name(chat_id: i32, name: &str) -> anyhow::Result<Chat> {
    let chat = connect()?
        .exec_map(
            "call SimpleMessenger.UpdateChatName(:id, :name);",
            (chat_id, name),
            |(id, name, owner_id, image)| Chat {
                id,
                owner_id,
                name,
                image,
                last_read_message_id: 0,
            },
        )?
        .first()
        .expect("request return no chat")
        .to_owned();

    Ok(chat)
}

pub async fn delete_chat(owner_id: i32, chat_id: i32) -> anyhow::Result<()> {
    connect()?.exec_drop(
        "call SimpleMessenger.DeleteChat(:chat_id, :owner_id);",
        (chat_id, owner_id),
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
