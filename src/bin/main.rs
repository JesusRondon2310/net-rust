use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

struct Data {
    data: String,
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Waiting for output\n");

    let tcp: TcpStream = match TcpStream::connect("10.5.0.3:22") {
        Ok(d) => d,
        Err(e) => {
            let e_msg = format!("error in tcp connection : {e:#?}");
            return Err(e_msg.into());
        }
    };

    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password("root", "password")?;

    let seconds = Duration::from_secs(5);
    loop {
        thread::sleep(seconds); // waits 5 seconds

        let mut channel = sess.channel_session()?;
        let mut data_collection: Data = Data {
            data: String::new(),
        };

        let command: &str = "echo Memory: $(free -h | awk '/Mem:/ {print $3 \"/\" $2}')";
        channel.shell().unwrap();
        channel.write_all(command.as_bytes()).unwrap();
        channel.send_eof().unwrap();
        channel.read_to_string(&mut data_collection.data).unwrap();

        println!("{}", data_collection.data)
    }
}
