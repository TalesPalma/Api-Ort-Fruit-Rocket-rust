use crate::models_view::home::Home;
use rocket::serde::json::Json;

#[get("/")]
pub fn index() -> Json<Home> {
    Json(Home {
        title: "Wecome to Rocket".to_string(),
        endpoint: vec!["/recurso".to_string()],
    })
}
