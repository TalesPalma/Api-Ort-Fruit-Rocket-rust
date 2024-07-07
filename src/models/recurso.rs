use ::serde::Serialize;

#[derive(Debug, Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Recurso {
    pub id: i32,
    pub name: String,
    pub description: String,
}
