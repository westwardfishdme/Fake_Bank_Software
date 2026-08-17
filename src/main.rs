mod modules;
use modules::{accounts, database, rest};

use axum;
use sled::{self, Db};
use std::rc::Rc;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = format!("{IP}:{PORT}");
    let listener = tokio::net::TcpListener::bind(&server).await?;

    let rest_api = rest::handler::run();
    println!("waiting for tcp connection on {server}");
    axum::serve::serve(listener, rest_api)
        .await
        .expect("failed to start");
    println!("received tcp connection on: {}", server);
    Ok(())
}
