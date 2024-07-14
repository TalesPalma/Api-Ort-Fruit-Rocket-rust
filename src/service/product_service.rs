use crate::{
    database::{models::product::Product, repository::product_repository::*},
    dtos::product_dto::ProductDto,
};

pub fn get_products_service() -> Vec<Product> {
    match get_products() {
        Ok(value) => value,
        Err(_) => vec![],
    }
}

pub fn create_product_service(new_prodcut: &ProductDto) -> Result<Product, String> {
    match create_product(new_prodcut.clone()) {
        Ok(value) => Ok(value),
        Err(error) => Err(format!("Error with creating aplications: {} ", error)),
    }
}

pub(crate) fn delete_product_service(prod_id: i32) -> Result<(), String> {
    match delete_product(prod_id) {
        Ok(_) => Ok(()),
        Err(value) => Err(format!("Error with delete {}", value)),
    }
}

pub fn update_product_service(prod_id: i32, new_product: ProductDto) -> Result<ProductDto, String> {
    match update_product(prod_id, new_product) {
        Ok(value) => Ok(value),
        Err(msg_error) => Err(format!("Error with update {}", msg_error)),
    }
}
