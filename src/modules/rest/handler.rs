use crate::modules::accounts;
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;

#[derive(Debug, PartialEq)]
pub enum APIError {
    NotFound,
    FailedAuthentication,
    InvalidInput(String),
    InternalError,
}

/// The IntoResponse
impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        let (status, e) = match self {
            APIError::NotFound => (StatusCode::NOT_FOUND, "could not query request".into()),
            APIError::InvalidInput(message) => (StatusCode::BAD_REQUEST, message),
            APIError::FailedAuthentication => (
                StatusCode::UNAUTHORIZED,
                "failed to authenticate user. access wqas denied".into(),
            ),
            APIError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "server failed internally; check logs".into(),
            ),
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

async fn login() -> impl IntoResponse {
    Json(json!({
        "status":400,
        "message": "this would attempt to authenticate the user"
    }))
}

pub fn run() -> Router {
    Router::new()
        .route("/login", get(login))
        .route("/health", get(health))
}
