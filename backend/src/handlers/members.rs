use rocket::Route;

use crate::claims::Claims;
use crate::response::Response;
use crate::tied::members_mapper;

pub const PATH: &str = "/Members";
pub fn routes() -> Vec<Route> {
    routes![set_message, add_member, remove_member, get_members]
}

#[post("/<chat_id>/<message_id>")]
pub async fn set_message(user: Claims, chat_id: i32, message_id: i32) -> Response {
    let result = members_mapper::set_last_read_message(chat_id, user.id, message_id).await;

    if let Err(_) = result {
        //println!("{:?}", i);
        return Response::Err(());
    }

    Response::Ok(String::from("Ok"))
}
#[put("/<chat_id>/<member_id>")]
pub async fn add_member(_user: Claims, chat_id: i32, member_id: i32) -> Response {
    let result = members_mapper::add_member(chat_id, member_id).await;

    if let Err(_) = result {
        //println!("{:?}", i);
        return Response::Err(());
    }

    Response::Ok(String::from("Ok"))
}

#[delete("/<chat_id>/<member_id>")]
pub async fn remove_member(_user: Claims, chat_id: i32, member_id: i32) -> Response {
    let result = members_mapper::remove_member(chat_id, member_id).await;

    if let Err(_) = result {
        //println!("{:?}", i);
        return Response::Err(());
    }

    Response::Ok(String::from("Ok"))
}
#[get("/<id>")]
pub async fn get_members(_user: Claims, id: i32) -> Response {
    let members = members_mapper::get_members(id).await;
    //println!("{:?}", chats);
    if let Err(_) = members {
        return Response::Err(());
    }

    let members = members.unwrap();

    Response::Ok(serde_json::to_string(&members).unwrap())
}
