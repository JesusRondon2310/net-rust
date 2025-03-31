use crate::helpers::definitions::models::System;
use std::error::Error;

use sqlx::{
    postgres::{PgConnectOptions, PgRow},
    PgPool, Row,
};

pub async fn postgres_connection() -> Result<PgPool, Box<dyn Error + Send + Sync>> {
    let opt = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("password")
        .database("main_db");

    match PgPool::connect_with(opt).await {
        Ok(d) => return Ok(d),
        Err(e) => {
            let e_msg = format!("error in connection to db : {e:#?}");
            return Err(e_msg.into());
        }
    }
}

pub async fn create_table(connection: &PgPool) -> Result<(), Box<dyn Error + Send + Sync>> {
    match sqlx::query("CREATE TABLE IF NOT EXISTS system (memory VARCHAR(20), timestamp TIMESTAMP)")
        .execute(connection)
        .await
    {
        Ok(_d) => return Ok(()),
        Err(e) => {
            let e_msg = format!("error in connection to db : {e:#?}");
            return Err(e_msg.into());
        }
    }
}

pub async fn add_column(
    connection: &PgPool,
    system_data: &System,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match sqlx::query("INSERT INTO system (memory, timestamp) VALUES ($1, $2)")
        .bind(system_data.memory.clone())
        .bind(system_data.timestamp.clone())
        .fetch_all(connection)
        .await
    {
        Ok(_d) => return Ok(()),
        Err(e) => {
            let e_msg = format!("error in adding rows : {e:#?}");
            return Err(e_msg.into());
        }
    }
}

pub async fn fetch_data(connection: &PgPool) -> Result<Vec<System>, Box<dyn Error + Send + Sync>> {
    let rows: Vec<PgRow> = sqlx::query("SELECT memory, timestamp FROM system")
        .fetch_all(connection)
        .await
        .expect("error when fetching data");

    if rows.is_empty() {
        return Err("no system row found".into());
    }

    let data: Vec<System> = rows
        .iter()
        .map(|row: &PgRow| System {
            memory: row.get(0),
            timestamp: row.get(1),
        })
        .collect();

    Ok(data)
}
