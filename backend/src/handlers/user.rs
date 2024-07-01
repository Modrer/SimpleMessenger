use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::Route;

use crate::claims::Claims;
use crate::file_saver::save_file;
use crate::handlers::Authorize;
use crate::response::Response;
use crate::tied::members_mapper;

pub const PATH: &str = "/User";
pub fn routes() -> Vec<Route> {
    routes![get_user, search_user, update_user]
}

#[get("/<user_id>")]
pub async fn get_user(user_id: i32) -> Response {
    let result = db_manager::user_mapper::get_user_by_id(user_id).await;

    if let Err(_) = result {
        return Response::Err(());
    }

    let result = result.unwrap();

    if let None = result {
        //println!("{:?}", i);
        return Response::Err(());
    }

    let result = result.unwrap();

    Response::Ok(serde_json::to_string(&result).unwrap())
}
#[post("/<name>")]
pub async fn search_user(name: &str) -> Response {
    let result = db_manager::user_mapper::get_users_by_name(name).await;

    if let Err(i) = result {
        println!("{:?}", i);
        return Response::Err(());
    }

    let result = result.unwrap();

    Response::Ok(serde_json::to_string(&result).unwrap())
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

#[derive(FromForm)]
pub struct UpdateUserForm<'a> {
    image: Option<TempFile<'a>>,
    name: &'a str,
    email: &'a str,
}
#[patch("/", data = "<form>")]
pub async fn update_user<'a>(user: Claims, mut form: Form<UpdateUserForm<'a>>) -> Response {
    let image = form.image.take();

    let mut user_image = user.image;

    if let Some(mut image) = image {
        let result = save_file(&mut image).await;

        if let Ok(info) = result {
            user_image = info.name;
        }
    }

    let _ = db_manager::user_mapper::update_user(user.id, form.name, form.email, &user_image).await;

    let token =
        authentification::registrator::generate_token(user.id, form.name, form.email, &user_image);

    Response::Ok(
        serde_json::to_string(&Authorize {
            id: user.id,
            name: form.name.to_string(),
            email: form.email.to_string(),
            token,
            image: user_image,
        })
        .unwrap(),
    )
}
