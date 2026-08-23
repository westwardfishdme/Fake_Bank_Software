mod modules;
use modules::rest;

use modules::{
    accounts::authenticate::AuthenticationDetails,
    database::utilities::{open_authentication_database, open_user_database},
};
// rest API details
/// IP should be set to 0.0.0.0 for global listens.
const IP: &str = "0.0.0.0";
/// PORT is set to 8080 to listen for API requests,
/// nothing too special here about it.
const PORT: u16 = 8080;
const DEFAULT_ADMIN_PASSWORD: &str = "password";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = format!("{IP}:{PORT}");
    let listener = tokio::net::TcpListener::bind(&server).await?;
    let auth_db = open_authentication_database();
    let user_db = open_user_database();

    if auth_db
        .clone()
        .get("admin")
        .expect("failed to open auth_db")
        .is_none()
    {
        AuthenticationDetails::create_admin_account(DEFAULT_ADMIN_PASSWORD, &auth_db);
    }

    let rest_api = rest::handler::run(auth_db, user_db);
    println!("starting RESTAPI @ {server}");
    axum::serve::serve(listener, rest_api)
        .await
        .expect("failed to start REST API");
    Ok(())
}
