use crate::database;
use crate::database::models::product::Product;
use crate::dtos::product_dto::ProductDto;
use crate::schema::products;
use crate::schema::products::dsl::*;
use database::database_config::get_conection;
use diesel::prelude::*;

pub fn get_products() -> Result<Vec<Product>, diesel::result::Error> {
    let conn = &mut get_conection();
    let result = products.limit(5).select(Product::as_select()).load(conn)?;
    Ok(result)
}

pub fn get_products_by_id(prod_id: i32) -> Result<Product, diesel::result::Error> {
    let conn = &mut get_conection();
    let result = products
        .filter(id.eq(prod_id))
        .limit(5)
        .select(Product::as_select())
        .first(conn)?;

    Ok(result)
}

///Peguei tudo isso da documentcao do diesel
/// https://github.com/diesel-rs/diesel/blob/2.2.x/examples/mysql/getting_started_step_2/src/lib.rs
pub fn create_product(new_product: ProductDto) -> Result<Product, diesel::result::Error> {
    let conn = &mut get_conection();
    conn.transaction(|conn| {
        diesel::insert_into(products::table)
            .values(&new_product)
            .execute(conn)?;

        products::table
            .order(products::id.desc())
            .select(Product::as_select())
            .first(conn)
    })
}

pub fn delete_product(prod_id: i32) -> Result<(), diesel::result::Error> {
    let conn = &mut get_conection();
    diesel::delete(products.filter(id.eq(prod_id))).execute(conn)?;
    Ok(())
}

pub fn update_product(
    prod_id: i32,
    new_product: ProductDto,
) -> Result<ProductDto, diesel::result::Error> {
    let conn = &mut get_conection();

    diesel::update(products.filter(id.eq(prod_id)))
        .set((
            name.eq(new_product.name.clone()),
            price.eq(new_product.price.clone()),
            description.eq(new_product.description.clone()),
        ))
        .execute(conn)?;

    Ok(new_product)
}
