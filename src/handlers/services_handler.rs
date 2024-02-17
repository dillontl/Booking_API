use axum::{
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use crate::services::services_service::get_services;

pub async fn get_services_handler() -> impl IntoResponse {
    match get_services().await {
        Ok(services) => (StatusCode::OK, Json(services)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response(),
    }
}
