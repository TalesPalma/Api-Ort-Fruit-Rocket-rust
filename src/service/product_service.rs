use crate::{
    database::{models::product::Product, repository::product_repository::*},
    dtos::product_dto::ProductDto,
};

pub fn get_products_service() -> Vec<Product> {
    let products_vec = match get_products() {
        Ok(value) => value,
        Err(_) => vec![],
    };
    products_vec
}

pub fn create_product_service(new_prodcut: &ProductDto) -> Result<Product, String> {
    match create_product(new_prodcut.clone()) {
        Ok(value) => Ok(value),
        Err(error) => Err(format!("Error with creating aplications: {} ", error)),
    }
}

// pub(crate) fn modify_recurso(
//     _id: u32,
//     new_user: Recurso,
// ) -> Result<Product, diesel::result::Error> {
//
// }

// pub(crate) fn delete_recurso(id: u32) -> Result<Product, std::io::Error> {
//
//     }
// }
