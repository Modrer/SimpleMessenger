use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::Route;
use rocket::serde::json::Json;

use crate::claims::Claims;
use crate::file_saver::save_file;
use crate::handlers::inputs::IdInput;
use crate::response::Response;
use crate::tied::chat_mapper;

pub const PATH: &str = "/Chat";
pub fn routes() -> Vec<Route> {
    routes![create_chat, delete_chat, get_chats, update_chat]
}

#[derive(FromForm)]
pub struct CreateChatForm<'a> {
    image: Option<TempFile<'a>>,
    name: &'a str,
}
#[derive(FromForm)]
pub struct UpdateChatForm<'a> {
    id: i32,
    image: Option<TempFile<'a>>,
    name: &'a str,
}

#[put("/", data = "<form>")]
pub async fn create_chat<'a>(mut form: Form<CreateChatForm<'a>>, user: Claims) -> Response {
    let image = form.image.take();

    let mut user_image = None;

    if let Some(mut image) = image {

        let result = save_file(&mut image).await;

        if let Ok(info) = result {
            user_image = Some(info.name);
        }
    }

    let image: Option<&str> = user_image.as_deref();

    let chat = chat_mapper::create_chat(user.id, form.name, image).await;

    if let Err(_) = chat {
        //println!("{:?}", i);
        return Response::Err(());
    }

    let chat = chat.unwrap();

    Response::Ok(serde_json::to_string(&chat).unwrap())
}

#[patch("/", data = "<form>")]
pub async fn update_chat<'a>(mut form: Form<UpdateChatForm<'a>>, _user: Claims) -> Response {
    let image = form.image.take();

    let mut user_image = None;

    if let Some(mut image) = image {
        let saved = save_file(&mut image).await;



        user_image = Some(saved.unwrap().name);
    }

    let image: Option<&str> = user_image.as_deref();

    let chat = chat_mapper::update_chat(form.id, form.name, image).await;

    if let Err(i) = chat {
        println!("{:?}", i);
        return Response::Err(());
    }

    let chat = chat.unwrap();

    Response::Ok(serde_json::to_string(&chat).unwrap())
}
// fn get_file_name<'a>(file: &'a TempFile) -> Result<&'a str, anyhow::Error>{
//     file.raw_name().ok_or(anyhow::Error::msg(""))?.as_str().ok_or(anyhow::Error::msg(""))
// }
// #[patch("/", data = "<form>")]
// pub async fn update_user<'a>(user: Claims, mut form: Form<UpdateUserForm<'a>>) -> Response {
//
//     let image = form.image.take();
//
//     let mut user_image = user.image;
//
//     if let Some(mut image) = image
//     {
//
//         let id = Uuid::new_v4();
//         let file_name = id.to_string();
//
//         let path = Path::new("./static/");
//         let path = path.join(&file_name);
//
//         let saved = image.persist_to(&path).await.unwrap();
//
//
//         user_image = file_name;
//
//         //return Response::Ok(format!("{} {}",image.name().unwrap(), image.content_type().unwrap().to_string() ))
//     }
//
//     let _ = db_manager::user_mapper::update_user(user.id, form.name, form.email, &user_image).await;
//
//     let token = authentification::registrator::generate_token(user.id, form.name,form.email,&user_image);
//
//     Response::Ok( serde_json::to_string(&Authorize {
//         id: user.id,
//         name: form.name.to_string(),
//         email: form.email.to_string(),
//         token,
//         image: user_image,
//     }).unwrap())
//
//
// }

#[delete("/", format = "json", data = "<id>")]
pub async fn delete_chat(id: Json<IdInput>, user: Claims) -> Response {
    let result = chat_mapper::delete_chat(user.id, id.id).await;
    println!("{:?}", result);
    if let Err(_) = result {
        return Response::Err(());
    }

    Response::Ok(String::from("Ok"))
}
#[get("/")]
pub async fn get_chats(user: Claims) -> Response {
    let chats = chat_mapper::get_chats(user.id).await;
    //println!("{:?}", chats);
    if let Err(_) = chats {
        return Response::Err(());
    }

    let chats = chats.unwrap();

    Response::Ok(serde_json::to_string(&chats).unwrap())
}
