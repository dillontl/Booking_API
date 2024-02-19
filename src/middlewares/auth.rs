use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{self, request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use chrono::Duration;
use firebase_auth::{FirebaseAuthState};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tracing::debug;

#[derive(Debug)]
pub struct JwkConfiguration {
    pub jwk_url: String,
    pub audience: String,
    pub issuer: String,
}

#[derive(Debug, Deserialize)]
pub struct KeyResponse {
    pub keys: Vec<JwkKey>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JwkKey {
    pub e: String,
    pub alg: String,
    pub kty: String,
    pub kid: String,
    pub n: String,
}

/// The Jwt claims decoded from the user token. Can also be viewed as the Firebase User
/// information.
#[derive(Serialize, Deserialize, Clone)]
pub struct FirebaseUser {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
    pub auth_time: u64,
    pub user_id: String,
    pub provider_id: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub firebase: FirebaseProvider,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FirebaseProvider {
    sign_in_provider: String,
    identities: Map<String, Value>,
}

#[derive(Debug, Clone)]
pub struct JwkKeys {
    pub keys: Vec<JwkKey>,
    pub max_age: Duration,
}

#[derive(Debug)]
pub enum PublicKeysError {
    NoCacheControlHeader,
    MaxAgeValueEmpty,
    NonNumericMaxAge,
    NoMaxAgeSpecified,
    CannotParsePublicKey,
}
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