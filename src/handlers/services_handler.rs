use axum::{
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use crate::services::services_service::{add_services, get_services};
use crate::models::services_model::{NewService};

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum ServicesInput {
    Single(NewService),
    Multiple(Vec<NewService>),
}

pub async fn get_services_handler() -> impl IntoResponse {
    match get_services().await {
        Ok(services) => (StatusCode::OK, Json(services)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response(),
    }
}

pub async fn add_services_handler(Json(services_input): Json<ServicesInput>) -> impl IntoResponse {
    let services = match services_input {
        ServicesInput::Single(service) => vec![service],
        ServicesInput::Multiple(services) => services,
    };

    match add_services(&services) {
        Ok(_) => (StatusCode::OK, "Services added successfully").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to add services").into_response(),
    }
}
