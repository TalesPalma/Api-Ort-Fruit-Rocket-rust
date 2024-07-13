use crate::schema::products;
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Insertable, Clone)]
#[diesel(table_name = products)]
pub struct ProductDto {
    pub name: String,
    pub description: String,
    pub price: i32,
}
