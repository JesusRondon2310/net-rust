use ssh2::Session;
use std::error::Error;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use crate::helpers::database::{add_column, create_table, postgres_connection};
use crate::helpers::definitions::models::System;

struct Data {
    data: String,
}

pub async fn specs(commands: &str, session: Session) -> Result<(), Box<dyn Error>> {
    let seconds = Duration::from_secs(5);
    let pool = postgres_connection()
        .await
        .expect("error to connecting to db");

    create_table(&pool)
        .await
        .expect("error to connecting to db");

    loop {
        thread::sleep(seconds); // waits 5 seconds

        let mut data_collection: Data = Data {
            data: String::new(),
        };

        let command: &str = commands;
        let mut channel = session.channel_session()?;
        channel.exec(command)?;
        channel.read_to_string(&mut data_collection.data)?;

        let now: chrono::NaiveDateTime = chrono::Utc::now().naive_local();

        let my_system: System = System {
            memory: data_collection.data.trim_end().to_string().clone(),
            timestamp: now,
        };

        add_column(&pool, &my_system)
            .await
            .expect("error to connecting to db");
    }
}
