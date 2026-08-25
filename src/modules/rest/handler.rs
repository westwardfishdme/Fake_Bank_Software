use crate::log;
use crate::modules::{accounts::authenticate::User, rest::handler::APIError::BadRequest};
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::{Value, json};
use sled::Db;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
enum APIError {
    NotFound,
    FailedAuthentication,
    AlreadyExists,
    InternalError,
    BadRequest(&'static str),
}

/// The IntoResponse
impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        let (status, e): (StatusCode, String) = match self {
            APIError::NotFound => (StatusCode::NOT_FOUND, "could not query request".into()),
            APIError::BadRequest(v) => (StatusCode::BAD_REQUEST, v.into()),
            APIError::FailedAuthentication => (
                StatusCode::UNAUTHORIZED,
                "failed to authenticate user. access was denied".into(),
            ),
            APIError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "server failed internally.".into(),
            ),
            APIError::AlreadyExists => (StatusCode::BAD_REQUEST, "User already exists".into()),
        };
        log!("an error occurred when processing a request: {}", e);
        let body = axum::Json(json!({
            "error": e
        }));
        (status, body).into_response()
    }
}
async fn health() -> impl IntoResponse {
    //! checks the health and status of the REST API
    Json(json!({
        "status":400,
        "message": "server is active and running"
    }))
}

/* logins */
async fn get_login(Json(auth_payload): Json<User>, auth_db: Db) -> Result<Json<Value>, APIError> {
    //! gets the user login from passing username

    /*
     * [intentionally vulnerable]
     * we want to  have users be able to grab the hash and salt values from
     * just providing a valid user name.
     */
    log!("Received GET request at /login");
    log!("getting user information for: {}", auth_payload.username());
    match auth_payload
        .login(&auth_db)
        .expect("[Authentication] failed")
    {
        Some(v) => Ok(Json(serde_json::json!(v))),
        None => Err(APIError::NotFound),
    }
}

async fn change_password(
    Json(auth_payload): Json<User>,
    auth_db: Db,
) -> Result<Json<Value>, APIError> {
    log!("Received PATCH request at /login");
    log!("Changing the password for: {}", auth_payload.username());
    match auth_payload.new_password(&auth_db) {
        Ok(_) => Ok(Json(json!({"update_password":"success"}))),
        Err(e) => Err(APIError::BadRequest(e)),
    }
}

async fn create_account(
    Json(auth_payload): Json<User>,
    auth_db: Db,
    user_db: Db,
) -> Result<Json<Value>, APIError> {
    //! creates an account in the database
    /* [ intentionally vulnerable ]
     * this will be used by pentesters and competitors
     * to create a bad account
     */
    log!("Received POST request at /account/create");
    log!("Attempting to create account: {}", auth_payload.username());
    let login = match auth_payload.login(&auth_db) {
        Ok(v) => v,
        Err(_) => return Err(APIError::FailedAuthentication),
    };
    if login.is_none() {
        match auth_payload.create(&auth_db, &user_db) {
            Ok(_) => (),
            Err(e) => return Err(BadRequest(e)),
        }
        log!(
            "successfully pulled login details for {}",
            auth_payload.username()
        );
        return Ok(Json(json!({"user_creation":"success"})));
    }
    Err(APIError::AlreadyExists)
}

async fn get_account(
    Json(auth_payload): Json<User>,
    auth_db: Db,
    user_db: Db,
) -> Result<Json<Value>, APIError> {
    //! gets the user's account from authenticating.
    log!("Received GET request at /account");
    log!(
        "attempting to get account details for: {}",
        auth_payload.username()
    );
    let account = match auth_payload
        .clone()
        .login(&auth_db)
        .expect("failed to authenticate in the REST API")
    {
        Some(v) => v,
        None => return Err(APIError::NotFound),
    };

    match account.authenticate_to_account(auth_payload.password(), &user_db) {
        Ok(Some(v)) => Ok(Json(json!(v))),
        Ok(None) => Err(APIError::FailedAuthentication),
        Err(_) => Err(APIError::NotFound),
    }
}

pub fn run(auth_db: Db, user_db: Db) -> Router {
    Router::new()
        .route(
            "/login",
            get({
                let auth_db = auth_db.clone();
                move |login| get_login(login, auth_db)
            })
            .patch({
                let auth_db = auth_db.clone();
                move |login| change_password(login, auth_db)
            }),
        )
        .route(
            "/account",
            get({
                let auth_db = auth_db.clone();
                let user_db = user_db.clone();
                move |login| get_account(login, auth_db, user_db)
            }),
        )
        .route(
            "/account/create",
            get(()).post({
                let auth_db = auth_db.clone();
                move |create| create_account(create, auth_db, user_db)
            }),
        )
        .route("/", get(health))
}
