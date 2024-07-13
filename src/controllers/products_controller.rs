use crate::database::models::product::Product;
use crate::dtos::product_dto::ProductDto;
use crate::models::error_json::ErrorJson;
use crate::service::product_service;
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

// #[put("/recurso/<id>", data = "<cliente_dto>")]
// pub fn modify(
//     id: u32,
//     cliente_dto: Json<RecursoDTO>,
// ) -> Result<status::Custom<Json<RecursoDTO>>, status::Custom<Json<ErrorJson>>> {
//     let recurso = cliente_dto.into_inner();
//
//     let new_recurso = recursos_service::modify_recurso(
//         id,
//         Recurso {
//             id: 0,
//             name: recurso.name.clone(),
//             description: recurso.description.clone(),
//         },
//     );
//
//     match new_recurso {
//         Ok(_) => Ok(status::Custom(Status::Ok, Json(recurso))),
//         Err(_) => Err(status::Custom(
//             Status::BadRequest,
//             Json(ErrorJson::new("Error ao modificar")),
//         )),
//     }
// }
//
// #[delete("/recurso/<id>")]
// pub fn delete(id: u32) -> Result<status::NoContent, status::BadRequest<Json<ErrorJson>>> {
//     match recursos_service::delete_recurso(id) {
//         Ok(_) => Ok(status::NoContent),
//         Err(_) => Err(status::BadRequest(Json(ErrorJson::new(
//             "Error with delete data",
//         )))),
//     }
// }
