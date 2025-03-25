pub mod helpers {
    pub mod ssh;
    pub mod worker;
}

use helpers::ssh::connection::ssh_connection;
use helpers::worker::worker::worker;
use ssh2::Session;

pub fn code() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let commands: &str = "echo Memory: $(free -h | awk '/Mem:/ {print $3 \"/\" $2}')";
    let mut sess = Session::new()?;
    let ssh_channel = sess.channel_session()?;

    ssh_connection("127.0.0.1:2l2", "root", "password", sess);
    worker(commands, ssh_channel)
}
