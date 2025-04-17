use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");


}
