use dotenv::dotenv;
use crate::chat_mapper::get_chat;

mod chat_mapper;
mod connector;
mod contact_mapper;
mod members_mapper;
mod message_mapper;
mod user_mapper;

async fn chat_testing() -> anyhow::Result<()> {
    println!("get chats");
    let chats1 = chat_mapper::get_chats(1).await?;
    println!("create chat");
    let chat = chat_mapper::create_chat(1, "22.02.2024", None).await?;
    println!("{} {} {}", chat.name, chat.id, chat.owner_id);
    println!("get chats");
    let chats2 = chat_mapper::get_chats(1).await?;
    println!("get filter");
    let chats3 = chats2
        .iter()
        .filter(|chat2| chats1.iter().all(|chat1| chat1.id != chat2.id));

    chats3.for_each(|chat| println!("{} {} {}", chat.name, chat.id, chat.owner_id));

    Ok(())
}

async fn contacts_testing() -> anyhow::Result<()> {
    println!("get contacts");
    let contacts1 = contact_mapper::get_contacts(1).await?;
    contacts1
        .iter()
        .for_each(|contact| println!("{} {} {}", contact.user_id, contact.name, contact.image));
    println!("create contact");
    let contact = contact_mapper::add_contact(1, 5).await;
    //println!("{} {} {}", contact.id, contact.owner_id, contact.contact_id);
    println!("get chats");
    let contacts2 = contact_mapper::get_contacts(1).await?;
    println!("get filter");
    let contacts3 = contacts2.iter().filter(|contact2| {
        contacts1
            .iter()
            .all(|contact1| contact1.user_id != contact2.user_id)
    });

    contacts3
        .for_each(|contact| println!("{} {} {}", contact.user_id, contact.name, contact.image));
    Ok(())
}

async fn members_testing() -> anyhow::Result<()> {
    println!("get members");
    let members1 = members_mapper::get_members(30).await?;
    println!("add member");
    members_mapper::add_member(30, 5).await?;
    //println!("{} {} {}", contact.id, contact.owner_id, contact.contact_id);
    println!("get members");
    let members2 = members_mapper::get_members(30).await?;
    println!("get filter");
    let members3 = members2.iter().filter(|member2| {
        members1
            .iter()
            .all(|member1| member1.user_id != member2.user_id)
    });

    members3.for_each(|member| println!("{} {}", member.name, member.user_id));
    members_mapper::remove_member(30, 5).await?;

    Ok(())
}
async fn dotenv_testing() {
    match dotenv() {
        Ok(a) => println!("Ok : {:?}", a),
        Err(e) => {
            println!("Err :{:?}", e)
        }
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv_testing().await;
    let result = contacts_testing().await;

    if let Err(e) = result {
        println!("{}", e);
    }
    let result = get_chat(102).await.unwrap();

    println!("{:?} {:?} {:?}", &result.id, &result.name, &result.image);
    // println!("get members");
    // let messages1 = message_mapper::get_messages(29).await?;
    //
    // let message =
    //     message_mapper::send_message(1,29, "new message 22.02.2024").await?;
    //
    // println!("{}  {}", message.id, message.message);
    //
    // let messages2 = message_mapper::get_messages(29).await?;
    //
    // let messages3 =
    //     messages2.iter()
    //         .filter(|message2| messages1.iter()
    //             .all(|message1| message1.id != message2.id)
    //         );
    //
    // messages3
    //     .for_each(|message|
    //                   println!("{}  {}", message.id, message.message)
    //     );

    //   let c = contact_mapper::get_contacts(43).await;
    //
    //   if let Ok(i) = c{
    //       println!("test{:?}", i.len());
    //   }
    //   else if let Err(i) = c{
    //       println!("test{:?}", i);
    //   }
    //
    //   user_mapper::get_users_by_name("a").await?.iter()
    //       .for_each(|user|
    //
    //                println!("{}  {}", user.user_id, user.name)
    //
    // );
    //
    //   // println!("{} ", user_mapper::add_user({
    //   //     User{
    //   //         user_id: 0,
    //   //         name: "test".to_string(),
    //   //         email: "test@testedTest.com".to_string(),
    //   //         password_hash: "null".to_string(),
    //   //         salt: "null".to_string(),
    //   //         role: "User".to_string(),
    //   //     }
    //   // }).await.unwrap());
    //   println!("{} ", user_mapper::contain_user("test").await.unwrap());
    //   members_testing().await;

    Ok(())
}
