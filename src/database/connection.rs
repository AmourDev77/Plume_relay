use std::env;
use dotenv::dotenv;
use mysql::*;
use mysql::prelude::*;
use std::fs;
use std::path::PathBuf;

pub async fn connection_database() -> std::result::Result<PooledConn, Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL non définie");
    println!("Tentative de connexion à : {}", database_url);

    // Extraire l'URL sans le nom de la base de données
    let base_url = database_url
        .rsplitn(2, '/')
        .nth(1)
        .ok_or("Impossible d'extraire l'URL de base sans le nom de la DB")?;

    // Tenter de se connecter au serveur MySQL sans base spécifique
    let base_opts = Opts::from_url(base_url)?;
    let base_pool = Pool::new(base_opts)?;
    let mut base_conn = base_pool.get_conn()?;

    println!("Connexion au serveur principal réussie");

    // Créer ou restaurer la base de données
    restore_db(&mut base_conn)?;

    // Connexion finale à la base de données ciblée
    let opts = Opts::from_url(&database_url)?;
    let pool = Pool::new(opts)?;
    let mut plume = pool.get_conn()?;

    // Vérification de l'architecture des tables
    verification_archi_tables(&mut plume).await?;

    Ok(plume)
}


pub async fn verification_archi_tables(plume: &mut PooledConn) -> Result<(), Box<dyn std::error::Error>> {
    let db_name = plume.query_first::<String, _>("SELECT DATABASE()")?
        .ok_or("Impossible de récupérer le nom de la base de données")?;

    // Liste des tables attendues
    let expected_tables = vec!["Messages", "User"];

    // Récupérer la liste des tables dans la base
    let existing_tables: Vec<String> = plume.query_map(
        format!(
            "SELECT table_name FROM information_schema.tables WHERE table_schema = '{}'",
            db_name
        ),
        |table_name| table_name,
    )?;

    // Vérifier que toutes les tables attendues sont présentes
    for table in &expected_tables {
        if !existing_tables.iter().any(|t| t == table) {
            println!("La table '{}' est manquante, restauration nécessaire !", table);
            restore_db(plume)?;
            return Ok(());
        }
    }

    // Vérifier que toutes les tables existantes sont valides
    for table in &expected_tables {
        let res = plume.query::<(String, String, String, String, Option<String>, String), _>(
            format!("DESCRIBE `{}`", table)
        );
        if res.is_err() {
            println!("La table '{}' est malformée, restauration nécessaire !", table);
            restore_db(plume)?;
            return Ok(());
        }
    }

    println!("Toutes les tables attendues sont présentes et valides !");
    Ok(())
}

fn restore_db(plume: &mut PooledConn) -> Result<(), Box<dyn std::error::Error>> {
    // D'abord, créer la base de données si elle n'existe pas
    plume.query_drop("CREATE DATABASE IF NOT EXISTS plume")
        .map_err(|e| format!("Erreur création database: {}", e))?;
    
    // Sélectionner la base de données
    plume.query_drop("USE plume")
        .map_err(|e| format!("Erreur sélection database: {}", e))?;
    
    println!("Base de données 'plume' créée et sélectionnée");

    // Construire le chemin vers plume.sql
    let path = PathBuf::from("plume.sql");
    
    let sql_dump = fs::read_to_string(&path)
        .map_err(|e| format!("Erreur lecture dump SQL : {}", e))?;

    // Filtrer les commandes USE et CREATE DATABASE du dump (si elles existent)
    for statement in sql_dump.split(';') {
        let stmt = statement.trim();
        if stmt.is_empty() {
            continue;
        }
        
        // Ignorer les commandes USE et CREATE DATABASE du dump
        let stmt_upper = stmt.to_uppercase();
        if stmt_upper.starts_with("USE ") || stmt_upper.starts_with("CREATE DATABASE") {
            continue;
        }
        
        plume.query_drop(stmt).map_err(|e| {
            format!("Erreur lors de l'exécution de la requête SQL '{}': {}", stmt, e)
        })?;
    }

    println!("Restauration terminée !");
    Ok(())
}