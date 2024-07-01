use crate::connector::connect;
use data_models::{Contact, UserInfo};
use mysql::prelude::Queryable;

pub async fn get_contacts(user_id: i32) -> anyhow::Result<Vec<UserInfo>> {
    let contacts = connect()?.exec_map(
        "call SimpleMessenger.GetAllContacts(:owner_id);",
        (user_id,),
        |(user_id, name, image)| UserInfo {
            user_id,
            name,
            image,
        },
    )?;

    Ok(contacts)
}
pub async fn add_contact(owner_id: i32, contact_id: i32) -> anyhow::Result<Contact> {
    let contacts = connect()?.exec_map(
        "call SimpleMessenger.AddContact(:owner_id, :contact_id);",
        (owner_id, contact_id),
        |(id, owner_id, contact_id)| Contact {
            id,
            owner_id,
            contact_id,
        },
    )?; //.expect("request return no contact").to_owned();

    if contacts.is_empty() {
        return Err(anyhow::Error::msg("returned no contact"));
    }

    let contact = contacts.first().unwrap().to_owned();
    Ok(contact)
}

pub async fn remove_contact(owner_id: i32, contact_id: i32) -> anyhow::Result<()> {
    connect()?.exec_drop(
        "call SimpleMessenger.RemoveContact(:owner_id, :contact_id);",
        (owner_id, contact_id),
    )?;

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use tokio;
//
//     use super::*;
//
//     #[async_std::test]
//     async fn it_works() {
//         match get_vehicles().await {
//             Ok(a) => {
//                 for chat in a{
//                     println!("Ok : {}", chat.name)}
//             }
//
//             Err(_) => {}
//         }
//         assert_eq!(4, 4);
//     }
// }
