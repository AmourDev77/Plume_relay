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
    
    // Extraire l'URL de base (sans le nom de la DB)
    let base_url = if let Some(pos) = database_url.rfind('/') {
        &database_url[..pos]
    } else {
        return Err("URL de base de données invalide".into());
    };
    
    // D'abord se connecter au serveur principal (sans spécifier de DB)
    match Opts::from_url(base_url) {
        Ok(base_opts) => {
            let base_pool = Pool::new(base_opts)?;
            match base_pool.get_conn() {
                Ok(mut base_conn) => {
                    println!("Connexion au serveur principal réussie");
                    // Créer/restaurer la DB sur le serveur principal
                    restore_db(&mut base_conn)?;
                    
                    // Maintenant se connecter à la DB spécifique
                    let opts = Opts::from_url(&database_url)?;
                    let pool = Pool::new(opts)?;
                    let mut plume = pool.get_conn()?;
                    
                    verification_archi_tables(&mut plume).await?;
                    Ok(plume)
                }
                Err(e) => {
                    println!("Connexion serveur principal échouée: {}. Tentative fallback...", e);
                    use_fallback_connection().await
                }
            }
        }
        Err(e) => {
            println!("URL principale invalide: {}. Tentative fallback...", e);
            use_fallback_connection().await
        }
    }
}

async fn use_fallback_connection() -> std::result::Result<PooledConn, Box<dyn std::error::Error>> {
    let fallback_url = "mysql://remote:remote@127.0.0.1:3306";
    println!("Tentative de connexion fallback: {}", fallback_url);

    let fallback_opts = Opts::from_url(fallback_url)?;
    let fallback_pool = Pool::new(fallback_opts)?;
    let mut fallback_conn = fallback_pool.get_conn()?;

    // Restauration de la base de données sur le fallback
    restore_db(&mut fallback_conn)?;
    println!("Restauration sur fallback terminée");

    // Se reconnecter à la DB spécifique sur le fallback
    let fallback_db_url = "mysql://remote:remote@127.0.0.1:3306/plume";
    let fallback_db_opts = Opts::from_url(fallback_db_url)?;
    let fallback_db_pool = Pool::new(fallback_db_opts)?;
    let mut final_conn = fallback_db_pool.get_conn()?;
    
    verification_archi_tables(&mut final_conn).await?;
    Ok(final_conn)
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