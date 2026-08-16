mod modules;
use modules::accounts;
use modules::database;
use std::path::PathBuf;

use axum;
use sled::{self, Db};
use tokio::io::AsyncReadExt;

/// location of the AUTHENTICATION_DATABASE.
const AUTHENTICATION_DATABASE_PATH: &str = "/tmp/auth_database";
/// location of the USER_DATABASE
const USER_DATABASE_PATH: &str = "/tmp/user_database";

// rest API details
/// IP should be set to 0.0.0.0 for global listens.
const IP: &str = "0.0.0.0";
/// PORT is set to 8080 to listen for API requests,
/// nothing too special here about it.
const PORT: u16 = 8080;

async fn listen_server(
    ip_and_port: &str,
    listener: &tokio::net::TcpListener,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("waiting for tcp connection on {ip_and_port}");
    let (_, address) = listener.accept().await?;
    println!(
        "received tcp connection on {ip_and_port}:\n\tfrom: {}\n\t  on: {}",
        address, ip_and_port
    );
    Ok(())
}

async fn handle_api_req() {
    std::process::exit(0);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = format!("{IP}:{PORT}");
    let listener = tokio::net::TcpListener::bind(&server).await?;
    loop {
        listen_server(server.as_str(), &listener).await?;
    }
}
