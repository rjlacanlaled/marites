use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use sqlx::{ Pool, Postgres, postgres::PgPoolOptions };
use sqlx::{ Error, query };
use tracing::info;

type Db = Pool<Postgres>;

// Hardcode to prevent deployed system db update
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

// SQL files
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - {}", "DEV ONLY", "init_dev_db");

    // Recreate db - DEV ONLY
    {
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        pexec(&root_db, SQL_RECREATE_DB).await?;
    }

    // Get other sql files
    let mut paths: Vec<PathBuf> = fs
        ::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    paths.sort();

    // Execute SQL files
    let app_db = new_db_pool(PG_DEV_APP_URL).await?;

    for path in paths {
        if let Some(path) = path.to_str() {
            let path = path.replace('\\', "/"); // for windows

            // Only take .sql and skip SQL recreate
            if path.ends_with(".sql") && path != SQL_RECREATE_DB {
                pexec(&app_db, &path).await?;
            }
        }
    }

    Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), Error> {
    info!("{:<12} - {} -- {file}", "DEV ONLY", "pexec");

    // READ THE FILE
    let content = fs::read_to_string(file)?;

    // TODO: Make the split more sql proof
    let sqls: Vec<&str> = content.split(";").collect();

    for sql in sqls {
        query(sql).execute(db).await?;
    }

    Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, Error> {
    info!("{:<12} - {}", "DEV ONLY", "new_db_pool");
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_con_url).await
}
