use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NameInput {
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct IdInput {
    pub id: i32,
}
