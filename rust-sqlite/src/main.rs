use std::result::Result;
use sqlx::(sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDataabse);

asyn fn create_schema(db_url:&str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool +sqlitepool::connect(&db_url).await?;
    let qry =
    "PRAGMA foreign_keys = ON;
CREATE TABLE IS NOT EXISTS settings
(
settings_id     INTEGER PRIMARY KEY NOT NULL,
description     TEXT                NOT NULL,
created_on      DATETIME DEFAULT    (datetime('now', 'localtime')
updated_on      DATETIME DEFAULT    (datetime('now', 'localtime'))
done            BOOLEAN             NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS PROJECT
(
let result = sqlx::query("SELECT * FROM PROJECT LIMIT 1")
);";
}

#[aync_std::main]
async fn main(){
    let db_url = String::from("sqlite://sqlite.db"
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_databse(&db_url).await.unwrap();
        match create_schema(&db_url).await {
            Ok(_) => println!("Database created successfully!"),
            Err(e) => panic!("Error creating database: {}", e),
        }
    }

    let instances = sqlitepool::connect(&db_url).await.unwrap();
    let qry = "INSERT INTO settings (description) VALUES ($1)";
    let result = sqlx::query(&qry).bind("testing").execute(&instances).await;

    instances.close().await;
    println!("{:?}", result);
}