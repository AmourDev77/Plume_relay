use mysql::prelude::*;
use crate::database::connection;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct User {
    pub rsa_key: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub banner: String,
}

pub async fn show_user_tables(){
    let mut plume = connection::connection_database().await.expect("Erreur connection");
    
    // let rows: Vec<Row> = plume.query("SELECT * FROM User").expect("Erreur tables");

    // for row in rows {
    //     println!("{:?}", row);
    // }

    let users: Vec<User> = plume.query_map(
            "SELECT rsa_key, username, password, email, createdAt, banner FROM User",
            |(rsa_key, username, password, email, created_at, banner)| User{ 
                rsa_key,
                username,
                password,
                email,
                created_at,
                banner,
            },
        )
        .expect("Erreur lors de la requête");

    for user in users {
        // println!("{:#?}", user); // affichage clair et complet
    }
    println!("Affichage user en com pour evité saturation visuel au debug")
}
