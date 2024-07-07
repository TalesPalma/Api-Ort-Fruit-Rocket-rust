#[macro_use]
extern crate rocket;

use rocket::serde::{self, json::Json, Serialize};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Recurso {
    id: i32,
    name: String,
    description: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Home {
    title: String,
    endpoint: Vec<String>,
}

#[get("/")]
fn home() -> Json<Home> {
    Json(Home {
        title: "Wecome to Rocket".to_string(),
        endpoint: vec!["/recurso".to_string()],
    })
}

#[get("/recurso")]
fn recurso() -> Json<Vec<Recurso>> {
    Json(vec![
        Recurso {
            id: 1,
            name: "Recurso 1".to_string(),
            description: "Descrição do recurso 1".to_string(),
        },
        Recurso {
            id: 2,
            name: "Recurso 2".to_string(),
            description: "Descrição do recurso 2".to_string(),
        },
    ])
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![home, recurso])
}
