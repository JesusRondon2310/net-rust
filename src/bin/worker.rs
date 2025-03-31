use rust_code::helpers::ssh::connection::ssh_connection;
use rust_code::helpers::worker::system::specs;
use ssh2::Session;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let commands: &str = "echo $(free -h | awk '/Mem:/ {print $3 \"/\" $2}')";
    let sess = Session::new()?;

    if let Err(e) = ssh_connection("127.0.0.1:22", "jesus", "1542", sess.clone()) {
        eprintln!("{e}")
    }
    if let Err(e) = specs(commands, sess.clone()).await {
        eprintln!("{e}")
    }
    Ok(())
}

// use rust_code::helpers::ssh::connection;
// use rust_code::helpers::database::columns::add_column;
// use rust_code::helpers::database::connection::postgres_connection;
// use rust_code::helpers::database::table::create_table;
// use tokio::runtime::Handle;
// use sqlx::{prelude::FromRow, Connection};

// #[derive(FromRow, Debug)]
// struct System {
//     memory: String,
//     timestamp: String,
// }

// use rust_code::helpers::{definitions::models::System, worker::system};
// use rust_code::helpers::database::connection;
// use sqlx::{Connection, PgConnection};\
// use rust_code::helpers::{
//     database::{add_column, create_table, fetch_data, postgres_connection},
//     definitions::models::System,
// };

// #[tokio::main]
// async fn main() {
//     // let opt = sqlx::postgres::PgConnectOptions::new()
//     //     .host("localhost")
//     //     .port(5432)
//     //     .username("postgres")
//     //     .password("password")
//     //     .database("main_db");

//     // let mut connection = sqlx::postgres::PgConnection::connect_with(&opt)
//     //     .await
//     //     .unwrap();
//     // let handle = Handle::current();

// add_column(
//     &pool,
//     &System {
//         memory: "test".to_string(),
//         timestamp: chrono::Utc::now().naive_local(),
//     },
// )
// .await
// .expect("error to connecting to db");

// let data: Vec<System> = fetch_data(&pool).await.expect("error fetching data");

// println!("{data:#?}")

//     // sqlx::query("CREATE TABLE IF NOT EXISTS system (Memory VARCHAR(5), Timestamp VARCHAR(20))")
//     //     .execute(&mut connection)
//     //     .await
//     //     .unwrap();

//     // sqlx::query("INSERT INTO system (memory, timestamp) VALUES ($1, $2)")
//     //     .bind(&"6.27")
//     //     .bind(&"2025-05-29 06:48:57")
//     //     .execute(&mut connection)
//     //     .await
//     //     .unwrap();

//     // let rows: Vec<System> = sqlx::query_as("SELECT * FROM system")
//     //     .fetch_all(&mut postgres_connection())
//     //     .await
//     //     .unwrap();

//     // for row in rows {
//     //     println!("{:#?}", row)
//     // }
// }

// // use ssh2::Session;
// // use std::error::Error;
// // use std::io::prelude::*;
// // use std::net::TcpStream;
// // use std::thread;
// // use std::time::Duration;

// // struct Data {
// //     data: String,
// // }

// // fn main() -> Result<(), Box<dyn Error>> {
// //     let tcp: TcpStream = match TcpStream::connect("127.0.0.1:22") {
// //         Ok(d) => d,
// //         Err(e) => {
// //             let e_msg = format!("error in tcp connection : {e:#?}");
// //             return Err(e_msg.into());
// //         }
// //     };

// //     let mut sess = Session::new().expect("");
// //     sess.set_tcp_stream(tcp);
// //     sess.handshake().expect("");
// //     sess.userauth_password("jesus", "1542").expect("");

// //     let command: &str = "echo Memory: $(free -h | awk '/Mem:/ {print $3 \"/\" $2}')";

// //     let seconds = Duration::from_secs(5);
// //     loop {
// //         let mut data_collection: Data = Data {
// //             data: String::new(),
// //         };
// //         thread::sleep(seconds); // waits 5 seconds

// //         let mut channel = sess.channel_session().expect("");
// //         channel.exec(command)?;
// //         channel.read_to_string(&mut data_collection.data)?;

// //         println!("{}", data_collection.data)
// //     }
// }
