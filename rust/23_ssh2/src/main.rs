use std::net::ToSocketAddrs;

use crate::ssh::SshClient;
use eyre::Result;

mod ssh;

fn main() -> Result<()> {
    let mut username = String::new();
    println!("Enter your username:");
    std::io::stdin().read_line(&mut username).unwrap();
    username = username.trim().to_string();

    let password = rpassword::prompt_password("Your password: ").unwrap();

    let sock_addr = "localhost:22".to_socket_addrs()?.next().unwrap();

    let mut client = SshClient::from(username, sock_addr, password);
    client.connect()?;
    println!(
        "Output of command: {:?}",
        String::from_utf8_lossy(&client.execute("ls")?.stdout)
    );

    Ok(())
}
