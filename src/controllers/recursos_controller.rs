use crate::dtos::recurso_dto::RecursoDTO;
use crate::models::error_json::ErrorJson;
use crate::models::recurso::Recurso;
use crate::service::recursos_service;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[get("/recurso")]
pub fn index() -> Json<Vec<Recurso>> {
    let recursos = recursos_service::get_recursos();
    Json(recursos)
}

#[post("/recurso", data = "<cliente_dto>")]
pub fn create(
    cliente_dto: Json<RecursoDTO>,
) -> Result<status::Custom<Json<RecursoDTO>>, status::Custom<Json<ErrorJson>>> {
    let recurso = cliente_dto.into_inner();

    let new_recurso = recursos_service::create_recurso(Recurso {
        id: 0,
        name: recurso.name.clone(),
        description: recurso.description.clone(),
    });

    match new_recurso {
        Ok(_) => Ok(status::Custom(Status::Created, Json(recurso))),
        Err(_) => Err(status::Custom(
            Status::BadRequest,
            Json(ErrorJson::default()),
        )),
    }
}

#[put("/recurso/<id>", data = "<cliente_dto>")]
pub fn modify(
    id: u32,
    cliente_dto: Json<RecursoDTO>,
) -> Result<status::Custom<Json<RecursoDTO>>, status::Custom<Json<ErrorJson>>> {
    let recurso = cliente_dto.into_inner();

    let new_recurso = recursos_service::modify_recurso(
        id,
        Recurso {
            id: 0,
            name: recurso.name.clone(),
            description: recurso.description.clone(),
        },
    );

    match new_recurso {
        Ok(_) => Ok(status::Custom(Status::Ok, Json(recurso))),
        Err(_) => Err(status::Custom(
            Status::BadRequest,
            Json(ErrorJson::new("Error ao modificar")),
        )),
    }
}

#[delete("/recurso/<id>")]
pub fn delete(id: u32) -> Result<status::NoContent, status::BadRequest<Json<ErrorJson>>> {
    match recursos_service::delete_recurso(id) {
        Ok(_) => Ok(status::NoContent),
        Err(_) => Err(status::BadRequest(Json(ErrorJson::new(
            "Error with delete data",
        )))),
    }
}
