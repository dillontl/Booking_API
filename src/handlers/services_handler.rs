use axum::{
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use log::error;
use crate::models::services_model::{DeleteServicesInput, EditServicesInput, ServicesInput};
use crate::services::services_service::{add_services, delete_services, edit_services, get_services};
use crate::middlewares::auth::FirebaseUser;

pub async fn get_services_handler(_user: FirebaseUser) -> impl IntoResponse {
    match get_services().await {
        Ok(services) => (StatusCode::OK, Json(services)).into_response(),
        Err(e) => {
            error!("Failed to get services: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get services: {}", e)).into_response()
        }
    }
}

pub async fn add_services_handler(
    _user: FirebaseUser,
    Json(services_input): Json<ServicesInput>
) -> impl IntoResponse {
    let services = match services_input {
        ServicesInput::Single(service) => vec![service],
        ServicesInput::Multiple(services) => services,
    };

    match add_services(&services) {
        Ok(_) => (StatusCode::OK, "Services added successfully").into_response(),
        Err(e) => {
            error!("Failed to add services: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to add services: {}", e)).into_response()
        }
    }
}

pub async fn edit_services_handler(
    _user: FirebaseUser,
    Json(edit_services_input): Json<EditServicesInput>
) -> impl IntoResponse {
    let services = match edit_services_input {
        EditServicesInput::Single(service) => vec![service],
        EditServicesInput::Multiple(services) => services,
    };

    match edit_services(&services) {
        Ok(_) => (StatusCode::OK, "Services edited successfully").into_response(),
        Err(e) => {
            error!("Failed to edit services: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to edit services: {}", e)).into_response()
        }
    }
}

pub async fn delete_services_handler(
    _user: FirebaseUser,
    Json(delete_services_input): Json<DeleteServicesInput>
) -> impl IntoResponse {
    let services = match delete_services_input {
        DeleteServicesInput::Single(service) => vec![service],
        DeleteServicesInput::Multiple(services) => services,
    };

    match delete_services(&services) {
        Ok(_) => (StatusCode::OK, "Services deleted successfully").into_response(),
        Err(e) => {
            error!("Failed to delete services: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete services: {}", e)).into_response()
        }
    }
}