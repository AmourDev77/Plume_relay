use std::process::{Command, Output};

use mysql::PooledConn;
pub mod connection;
pub mod commande; 

pub struct Database {
    pub plume : PooledConn,
}

// let mut plume = connection::connection_database().await.expect("Erreur connection");

pub async fn init(){
    match connection::connection_database().await {
        Ok(_) => println!("Connection Database plume : OK !"),
        Err(_) => {
            println!("Connection echouer tentative demarage database...");
            launch_database().await;
        }
    }    
}

pub async fn launch_database() -> std::result::Result<Output, Box<dyn std::error::Error>>{
    let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
        .args(["-d", "docker compose up"])
        .output()
        .expect("Execution du process raté")
    } else {
    Command::new("sh")
        .arg("-d")
        .arg("docker compose up")
        .output()
        .expect("Execution du process raté")
    };

    Ok(output)
}

pub async fn creation_database() -> std::result::Result<(), Box<dyn std::error::Error>> {
    match connection::connection_database().await {
        Ok(_) => {
            println!("Connection Database : OK\nCréation des tables...");
            Ok(())
        }
        Err(_) => {
            println!("Connection impossible...");
            Err("Erreur de connexion".into())
        }
    }
}
        // TODO: Faire les base de la base de donnée mdp user root etc avant la creation de la bdd
        // plume
