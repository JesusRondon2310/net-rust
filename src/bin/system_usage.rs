use rust_code::helpers::ssh::connection::ssh_connection;
use rust_code::helpers::worker::system::specs;
use ssh2::Session;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let commands: &str = "echo $(free -h | awk '/Mem:/ {print $3 \"/\" $2}')";
    let sess = Session::new()?;

    if let Err(e) = ssh_connection("10.5.0.3", "root", "password", sess.clone()) {
        eprintln!("{e}")
    }
    if let Err(e) = specs(commands, sess.clone()).await {
        eprintln!("{e}")
    }
    Ok(())
}
