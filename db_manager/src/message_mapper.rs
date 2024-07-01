use crate::connector::connect;
use data_models::Message;
use mysql::prelude::Queryable;

pub async fn get_messages(chat_id: i32) -> anyhow::Result<Vec<Message>> {
    let messages = connect()?.exec_map(
        "call SimpleMessenger.GetMessages(:chat_id);",
        (chat_id,),
        |(id, chat_id, sender_id, message)| Message {
            id,
            chat_id,
            sender_id,
            message,
        },
    )?;

    Ok(messages)
}

pub async fn send_message(sender_id: i32, chat_id: i32, message: &str) -> anyhow::Result<Message> {
    let message = connect()?
        .exec_map(
            "call SimpleMessenger.SendMessage(:sender_id,:chat_id,:message);",
            (sender_id, chat_id, message),
            |(id, chat_id, sender_id, message)| Message {
                id,
                chat_id,
                sender_id,
                message,
            },
        )?
        .first()
        .expect("cant get message")
        .to_owned();

    Ok(message)
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
