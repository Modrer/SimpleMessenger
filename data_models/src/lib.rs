use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub salt: String,
    pub image: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: i32,
    pub name: String,
    pub image: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub chat_id: i32,
    pub sender_id: i32,
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Membership {
    pub id: i32,
    pub chat_id: i32,
    pub user_id: i32,
    pub role: String,
    pub last_read_message_id: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: i32,
    pub owner_id: i32,
    pub contact_id: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Chat {
    pub id: i32,
    pub owner_id: i32,
    pub name: String,
    pub image: String,
    pub last_read_message_id: i32,
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
