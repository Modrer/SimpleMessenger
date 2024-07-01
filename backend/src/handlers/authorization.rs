use rocket::Route;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use serde::Deserialize;

use crate::claims::Claims;
use crate::handlers::{Authorization, Authorize};
use crate::handlers::authorization::ChangePasswordResult::{ServerError, WrongPassword};
use crate::response::{AuthorizeResponder, RegistrationResult, Response};

pub const PATH: &str = "/Authorization";
pub fn routes() -> Vec<Route> {
    routes![authorize, sign_up, check_authorization, change_password]
}

#[derive(Deserialize)]
pub struct PasswordData<'a> {
    pub new_password: &'a str,
    pub old_password: &'a str,
}
#[derive(Serialize, Deserialize)]
pub struct Login<'a> {
    pub login: &'a str,
    pub password: &'a str,
}
#[derive(Serialize, Deserialize)]
pub struct Registration<'a> {
    #[serde(rename = "name")]
    pub login: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Responder)]
pub enum ChangePasswordResult {
    //todo codes
    #[response(status = 401, content_type = "application/json")]
    WrongPassword(()),
    #[response(status = 401, content_type = "application/json")]
    Unauthorized(()),
    #[response(status = 500, content_type = "application/json")]
    ServerError(()),
    #[response(status = 200, content_type = "application/json")]
    Ok(()),
}

#[patch("/", data = "<password>")]
pub async fn change_password(
    user: Claims,
    password: Json<PasswordData<'_>>,
) -> ChangePasswordResult {
    let authorized_user =
        authentification::registrator::login(&user.name, password.old_password).await;

    if let Some(authorized_user) = authorized_user {
        if authorized_user.user_id != user.id {
            return WrongPassword(());
        }

        if !authentification::registrator::change_password(user.id, password.new_password).await {
            return ServerError(());
        }

        return ChangePasswordResult::Ok(());
    }

    return ChangePasswordResult::Unauthorized(());
}
#[put("/", data = "<user>")]
pub async fn sign_up(user: Json<Registration<'_>>) -> RegistrationResult {
    if authentification::registrator::register(user.email, user.login, user.password).await {
        return RegistrationResult::Ok(());
    }

    RegistrationResult::Err(())
}
#[post("/", data = "<user>")]
pub async fn authorize(user: Json<Login<'_>>) -> AuthorizeResponder {
    let user = authentification::registrator::login(user.login, user.password).await;

    return match user {
        Some(user) => {
            AuthorizeResponder::Ok(serde_json::to_string(&Authorize::from_user(user)).unwrap())
        }
        None => AuthorizeResponder::Unauthorized(()),
    };
}

#[get("/")]
pub async fn check_authorization(user: Claims) -> Response {
    Response::Ok(
        serde_json::to_string(&Authorization {
            id: user.id,
            name: &user.name,
            email: &user.email,
            image: &user.image,
        })
        .unwrap(),
    )
}
