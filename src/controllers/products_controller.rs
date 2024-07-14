use crate::database::models::product::Product;
use crate::dtos::product_dto::ProductDto;
use crate::models::error_json::ErrorJson;
use crate::service::product_service::{self, update_product_service};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[get("/products")]
pub fn index() -> Json<Vec<Product>> {
    let product = product_service::get_products_service();
    Json(product)
}

#[post("/products", data = "<product_dto>")]
pub fn create(
    product_dto: Json<ProductDto>,
) -> Result<status::Custom<Json<ProductDto>>, status::Custom<Json<ErrorJson>>> {
    let new_product = product_dto.into_inner();
    match product_service::create_product_service(&new_product) {
        Ok(_) => Ok(status::Custom(Status::Created, Json(new_product))),
        Err(message) => Err(status::Custom(
            Status::BadRequest,
            Json(ErrorJson::new(message.as_str())),
        )),
    }
}

#[delete("/products/<id_user>")]
pub fn delete(id_user: i32) -> Result<status::NoContent, status::Custom<Json<ErrorJson>>> {
    match product_service::delete_product_service(id_user) {
        Ok(_) => Ok(status::NoContent),
        Err(msg_error) => Err(status::Custom(
            Status::BadRequest,
            Json(ErrorJson::new(msg_error.as_str())),
        )),
    }
}

#[put("/products/<id_user>", data = "<new_product_dto>")]
pub fn update(
    id_user: i32,
    new_product_dto: Json<ProductDto>,
) -> Result<(), status::Custom<Json<ErrorJson>>> {
    let new = new_product_dto.into_inner();
    match update_product_service(id_user, new) {
        Ok(_) => Ok(()),
        Err(msg_error) => Err(status::Custom(
            Status::BadRequest,
            Json(ErrorJson::new(msg_error.as_str())),
        )),
    }
}
