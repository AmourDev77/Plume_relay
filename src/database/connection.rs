use std::env;
use dotenv::dotenv;
use mysql::*;

pub async fn connection_database() -> std::result::Result<PooledConn, Box<dyn std::error::Error>>{
    dotenv().ok();

    let database_url =env::var("DATABASE_URL").expect("Database url non set");
    
    println!("{}", database_url);
    
    let opts = Opts::from_url(&database_url).expect("Url invalide");

    let pool = Pool::new(opts)?;

    let plume = pool.get_conn()?;

    return Ok(plume);
}
