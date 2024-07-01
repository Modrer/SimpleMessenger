use data_models::User;

pub mod authorization;
pub mod chat;
pub mod contact;
mod inputs;
pub mod members;
pub mod messages;
pub mod user;
pub mod ws;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Authorize {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub token: String,
    pub image: String,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Authorization<'a> {
    pub id: i32,
    pub name: &'a str,
    pub email: &'a str,
    pub image: &'a str,
}
impl Authorize {
    pub fn from_user(user: User) -> Self {
        Authorize {
            token: authentification::registrator::generate_token(
                user.user_id,
                &user.name,
                &user.email,
                &user.image,
            ),
            id: user.user_id,
            name: user.name,
            email: user.email,
            image: user.image,
        }
    }
}
