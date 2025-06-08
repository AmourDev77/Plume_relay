use std::process::{Command, Output};
pub mod connection;
pub mod commande; 


pub async fn init(){
    match connection::connection_database().await {
        Ok(_) => println!("Connection Database plume : OK !"),
        Err(_) => {
            println!("Connection echouer tentative demarage database...");
            launch_database().await;

            match connection::connection_database().await{
                Ok(_) => println!("Connection database plume : OK !"),
                
                Err(_) => match creation_database().await{
                    Ok(_) => {
                        println!("état database plume : OK !");
                        match connection::connection_database().await{
                            Ok(_) => println!("Connection database plume : OK !"),
                            Err(_) => println!("Creation de la base de donnée impossible..."),
                        }
                    }
                    Err(_) => {
                        panic!("Creation de la base de donnée impossible...");
                        // TODO: Faire le module de la creation de la base de donnée avec docker
                    }
                }
            }
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

pub async fn creation_database(){
    // TODO: Faire les base de la base de donnée mdp user root etc avant la creation de la bdd
    // plume
}
