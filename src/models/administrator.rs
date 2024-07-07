use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Administrator {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub token: String,
}
