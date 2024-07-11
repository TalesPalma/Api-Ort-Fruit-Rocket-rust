#[macro_use]
extern crate rocket;
mod controllers;
mod database;
mod dtos;
mod middlewares;
mod models;
mod models_view;
mod schema;
mod service;
mod tests;
use controllers::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        //.attach(middlewares::auth_guard::JwtFairing)
        .mount(
            "/",
            routes![
                home_controller::index,
                home_controller::not_authorized,
                recursos_controller::index,
                recursos_controller::create,
                recursos_controller::modify,
                recursos_controller::delete,
                login_controller::login,
            ],
        )
}
