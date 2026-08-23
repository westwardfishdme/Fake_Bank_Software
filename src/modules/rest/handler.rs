use crate::modules::accounts::authenticate::{AuthenticationDetails, User};
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::{Value, json};
use sled::Db;

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
     * we want to create a user here and have users
     * be able to grab the hash and salt values from just providing
     * a valid user name.
     */

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
    match auth_payload.clone().new_password(&auth_db) {
        Ok(_) => Ok(Json(json!({"update_password":"success"}))),
        Err(e) => Err(APIError::BadRequest(e)),
    }
}

async fn post_create(
    Json(auth_payload): Json<User>,
    auth_db: Db,
    user_db: Db,
) -> Result<Json<Value>, APIError> {
    //! creates an account in the database
    /* [ intentionally vulnerable ]
     * this will be used by pentesters and competitors
     * to create a bad account
     */
    if auth_payload
        .clone()
        .login(&auth_db)
        .expect("failed to authenticate in the DB")
        .is_none()
    {
        auth_payload
            .create(&auth_db, &user_db)
            .expect("failed to create an account");
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
    let auth_payload = &auth_payload;

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
                move |create| post_create(create, auth_db, user_db)
            }),
        )
        .route("/", get(health))
}
