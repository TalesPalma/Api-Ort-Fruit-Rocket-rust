use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;

pub fn get_conection() -> MysqlConnection {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
