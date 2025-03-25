use ssh2::Channel;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

struct Data {
    data: String,
}

pub fn worker(
    commands: &str,
    mut channel: Channel,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let seconds = Duration::from_secs(5);
    loop {
        thread::sleep(seconds); // waits 5 seconds
        let mut data_collection: Data = Data {
            data: String::new(),
        };

        let command: &str = commands;
        channel.shell().unwrap();
        channel.write_all(command.as_bytes()).unwrap();
        channel.send_eof().unwrap();
        channel.read_to_string(&mut data_collection.data).unwrap();

        println!("{}", data_collection.data)
    }
}
