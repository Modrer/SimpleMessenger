pub mod claims;

use crate::models::claims::Claims;
use data_models::User;
use std::string::String;

pub struct Registration<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub password: &'a str,
}
pub struct HashedPassword {
    pub hashed_password: String,
    pub salt: String,
}

// pub trait FromRegistration {
//     fn from_registration(registration: Registration) -> Self;
// }
// impl FromRegistration for User{
//     fn from_registration(registration: Registration) -> Self {
//         let hashed = hash_password(registration.password);
//
//         User{
//             user_id: 0,
//             name: registration.name.to_string(),
//             email: registration.email.to_string(),
//             password_hash: hashed.hashed_password,
//             salt: hashed.salt,
//             image: Roles::User.to_str().to_string(),
//         }
//     }
// }
pub trait ToClaims {
    fn to_claims(&self) -> Claims;
}
impl ToClaims for User {
    fn to_claims(&self) -> Claims {
        Claims {
            id: self.user_id,
            email: &self.email,
            name: &self.name,
            image: &self.image,
        }
    }
}

pub struct UserIdentity<'a> {
    pub id: i32,
    pub email: &'a str,
    pub name: &'a str,
    pub image: &'a str,
}
impl UserIdentity<'_> {
    pub fn from_claims(claims: Claims) -> UserIdentity {
        UserIdentity {
            id: claims.id,
            email: claims.email,
            name: claims.name,
            image: claims.image,
        }
    }
}
