use diesel::SelectableHelper;

use crate::database;
use crate::database::models::product::Product;
use crate::schema::products::dsl::*;
use diesel::prelude::*;

pub fn get_products() -> Result<Vec<Product>, diesel::result::Error> {
    let conn = &mut database::database_config::get_conection();
    let result = products.limit(5).select(Product::as_select()).load(conn)?;
    Ok(result)
}

///Peguei tudo isso da documentcao do diesel
/// https://github.com/diesel-rs/diesel/blob/2.2.x/examples/mysql/getting_started_step_2/src/lib.rs
pub fn create_post(new_product: Product) -> Product {
    use crate::schema::products;

    let conn = &mut database::database_config::get_conection();

    conn.transaction(|conn| {
        diesel::insert_into(products::table)
            .values(&new_product)
            .execute(conn)?;

        products::table
            .order(products::id.desc())
            .select(Product::as_select())
            .first(conn)
    })
    .expect("erro")
}
