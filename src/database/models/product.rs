use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: i32,
}
