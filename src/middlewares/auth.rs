use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{self, request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use firebase_auth::{FirebaseAuthState};
use tracing::debug;
pub use crate::models::users_model::FirebaseUser;

fn get_bearer_token(header: &str) -> Option<String> {
    let prefix_len = "Bearer ".len();

    match header.len() {
        l if l < prefix_len => None,
        _ => Some(header[prefix_len..].to_string()),
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for FirebaseUser
    where
        FirebaseAuthState: FromRef<S>,
        S: Send + Sync,
{
    type Rejection = UnauthorizedResponse;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = FirebaseAuthState::from_ref(state);

        let auth_header = parts
            .headers
            .get(http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");

        let bearer = get_bearer_token(auth_header).map_or(
            Err(UnauthorizedResponse {
                msg: "Unauthorized Request".to_string(),
            }),
            Ok,
        )?;

        debug!("Got bearer token {}", bearer);

        match store.firebase_auth.verify(&bearer) {
            Err(e) => Err(UnauthorizedResponse {
                msg: format!("Failed to verify Token: {}", e),
            }),
            Ok(current_user) => Ok(current_user),
        }
    }
}

pub struct UnauthorizedResponse {
    msg: String,
}

impl IntoResponse for UnauthorizedResponse {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, format!("{}", self.msg)).into_response()
    }
}