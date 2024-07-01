use crate::connector::connect;
use anyhow::Error;
use data_models::{User, UserInfo};
use mysql::prelude::Queryable;

pub async fn get_users_by_name(user_name: &str) -> anyhow::Result<Vec<UserInfo>> {
    let users = connect()?
        .exec_map(
            "call SimpleMessenger.FindUsersByName(:user_name);",
            (user_name,),
            |(user_id, name, email, password_hash, salt, image)| User {
                user_id,
                name,
                email,
                password_hash,
                salt,
                image,
            },
        )?
        .iter()
        .map(|user| UserInfo {
            user_id: user.user_id,
            name: user.name.to_owned(),
            image: user.image.to_owned(),
        })
        .collect();

    Ok(users)
}

pub async fn get_user_by_name(user_name: &str) -> anyhow::Result<User> {
    let user = connect()?
        .exec_map(
            "call SimpleMessenger.FindUserByName(:user_name);",
            (user_name,),
            |(user_id, name, email, password_hash, salt, image)| User {
                user_id,
                name,
                email,
                password_hash,
                salt,
                image,
            },
        )?
        .first()
        .ok_or(Error::msg("user not found"))?
        .to_owned();

    Ok(user)
}

pub async fn get_user_by_id(id: i32) -> anyhow::Result<Option<UserInfo>> {
    let users = connect()?
        .exec_map(
            "call SimpleMessenger.FindUserById(:id);",
            (id,),
            |(user_id, name, email, password_hash, salt, image)| User {
                user_id,
                name,
                email,
                password_hash,
                salt,
                image,
            },
        )?
        .first()
        .map(|user| UserInfo {
            user_id: user.user_id,
            name: user.name.to_owned(),
            image: user.image.to_owned(),
        });

    Ok(users)
}

struct Number {
    num: i32,
}
pub async fn contain_user(name: &str) -> anyhow::Result<bool> {
    let contain = connect()?
        .exec_map("Select 1 from Users where Name = :name", (name,), |num| {
            Number { num }
        })?
        .len()
        != 0;

    Ok(contain)
}
pub async fn add_user(
    name: &str,
    email: &str,
    password_hash: &str,
    salt: &str,
) -> anyhow::Result<bool> {
    connect()?.exec_drop(
        "call SimpleMessenger.AddUser(:name, :email, :password_hash, :salt)",
        (name, email, password_hash, salt),
    )?;

    Ok(true)
}

pub async fn update_user(id: i32, name: &str, email: &str, image: &str) -> anyhow::Result<bool> {
    connect()?.exec_drop(
        "call SimpleMessenger.UpdateUser(:id, :name, :email, :image)",
        (id, name, email, image),
    )?;

    Ok(true)
}
pub async fn change_password(id: i32, password: &str, salt: &str) -> anyhow::Result<bool> {
    connect()?.exec_drop(
        "call SimpleMessenger.ChangePassword(:id, :password, :salt);",
        (id, password, salt),
    )?;

    Ok(true)
}

// pub async fn create_chat(owner_id: i32, name: &str) -> anyhow::Result<Chat> {
//
//     let chat = connect()?
//         .exec_map(
//             "call SimpleMessenger.CreateChat(:name, :owner_id);",
//             (name, owner_id),
//             |(id, name, owner_id)| {
//                 Chat { id, owner_id, name }
//             }
//         )?.first().expect("request return no chat").to_owned();
//
//     Ok(chat)
// }

// pub async fn delete_chat(owner_id: i32, chat_id: i32) -> anyhow::Result<()> {
//
//     connect()?
//         .exec_drop(
//             "call SimpleMessenger.DeleteChat(:chat_id, :owner_id);",
//             (chat_id, owner_id)
//         )?;
//
//     Ok(())
// }

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
