use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Home {
    pub title: String,
    pub endpoint: Vec<String>,
}
