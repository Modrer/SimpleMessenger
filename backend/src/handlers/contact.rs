use rocket::Route;
use rocket::serde::json::Json;

use crate::claims::Claims;
use crate::handlers::inputs::IdInput;
use crate::response::Response;

pub const PATH: &str = "/Contact";
pub fn routes() -> Vec<Route> {
    routes![add_contact, remove_contact, get_contacts]
}
#[put("/", format = "json", data = "<id>")]
pub async fn add_contact(id: Json<IdInput>, user: Claims) -> Response {
    let contact = db_manager::contact_mapper::add_contact(user.id, id.id).await;

    if let Err(_) = contact {
        //println!("{:?}", i);
        return Response::Err(());
    }

    let contact = contact.unwrap();

    Response::Ok(serde_json::to_string(&contact).unwrap())
}

#[delete("/", format = "json", data = "<id>")]
pub async fn remove_contact(id: Json<IdInput>, user: Claims) -> Response {
    let contact = db_manager::contact_mapper::remove_contact(user.id, id.id).await;

    if let Err(_) = contact {
        //println!("{:?}", i);
        return Response::Err(());
    }

    Response::Ok(String::from("Ok"))
}
#[get("/")]
pub async fn get_contacts(user: Claims) -> Response {
    let chats = db_manager::contact_mapper::get_contacts(user.id).await;
    //println!("{:?}", chats);
    if let Err(e) = chats {
        println!("{:?}", e);
        return Response::Err(());
    }

    let chats = chats.unwrap();

    Response::Ok(serde_json::to_string(&chats).unwrap())
}
