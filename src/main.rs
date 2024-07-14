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
                products_controller::index,
                products_controller::create,
                products_controller::delete,
                products_controller::update,
                login_controller::login,
            ],
        )
}
