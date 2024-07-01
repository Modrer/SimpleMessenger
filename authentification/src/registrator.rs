use data_models::User;
use db_manager::user_mapper;
use std::collections::BTreeMap;
use std::env;

use hmac::Mac;
use jwt::{AlgorithmType, Header, Token, Verified};
use once_cell::sync::Lazy;
use serde_json::Value;

use crate::authentication::Authenticator;
use crate::hasher::{check_password, hash_password};
use crate::models::claims::Claims;
use crate::models::UserIdentity;

static AUTHENTICATION: Lazy<Authenticator> = Lazy::new(|| {
    Authenticator::new(
        &*env::var("Key").expect("Key must be set"),
        AlgorithmType::Hs512,
    )
});

pub async fn contain_user(user_name: &str) -> bool {
    user_mapper::contain_user(user_name).await.unwrap()
}

pub async fn register<'a>(email: &'a str, name: &'a str, password: &'a str) -> bool {
    let hashed = hash_password(password);

    user_mapper::add_user(name, email, &hashed.hashed_password, &hashed.salt)
        .await
        .is_ok()
}
pub async fn login(login: &str, password: &str) -> Option<User> {
    let user = user_mapper::get_user_by_name(login).await;

    if let Err(_) = user {
        return None;
    }

    let user = user.unwrap();

    if !check_password(password, &user.salt, &user.password_hash) {
        return None;
    }

    Some(user)
}
pub async fn change_password(id: i32, password: &str) -> bool {
    let hashed = hash_password(password);

    user_mapper::change_password(id, &hashed.hashed_password, &hashed.salt)
        .await
        .is_ok()
}
pub fn generate_token(id: i32, name: &str, email: &str, image: &str) -> String {
    let user = Claims {
        id,
        email,
        name,
        image,
    };

    AUTHENTICATION.generate_token(
        &user,
        15,
        Some("127.0.0.1:8000".to_string()),
        Some("127.0.0.1:8000".to_string()),
    )
}
pub fn verify_token(
    token: &str,
) -> Result<Token<Header, BTreeMap<String, Value>, Verified>, jwt::Error> {
    AUTHENTICATION.verify_token(token)
}
pub fn get_user(claims: Claims) -> UserIdentity {
    UserIdentity {
        id: claims.id,
        email: claims.email,
        name: claims.name,
        image: claims.image,
    }
}
