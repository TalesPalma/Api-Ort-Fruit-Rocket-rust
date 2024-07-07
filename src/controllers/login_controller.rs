use crate::dtos::login_dto::LoginDTO;
use crate::models::administrator::Administrator;
use crate::service;
use rocket::serde::json::Json;

#[post("/login", data = "<user_login>")]
pub fn login(user_login: Json<LoginDTO>) -> Result<Json<Administrator>, Json<String>> {
    let user = user_login.into_inner();
    let email = user.email;
    let password = user.password;

    match service::administrator_service::login(email, password) {
        Ok(value) => Ok(Json(value)),
        Err(value) => Err(Json(value)),
    }
}
