use ssh2::Session;
use std::error::Error;
use std::net::TcpStream;

pub fn ssh_connection(
    addr: &str, // e.g. 127.0.0.1:22, 22 will be the port
    username: &str,
    password: &str,
    mut sesssion: Session,
) -> Result<(), Box<dyn Error>> {
    let tcp: TcpStream = match TcpStream::connect(addr) {
        Ok(d) => d,
        Err(e) => {
            let e_msg = format!("error in tcp connection : {e:#?}");
            return Err(e_msg.into());
        }
    };

    sesssion.set_tcp_stream(tcp);
    sesssion.handshake()?;
    sesssion.userauth_password(username, password)?;
    Ok(())
}
