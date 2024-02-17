use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Services {
    pub(crate) service_id: i32,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) price: f64,
    pub(crate) duration: f64,
}

#[derive(Debug, Serialize, Deserialize)] pub struct NewService {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) price: f32,
    pub(crate) duration: f64,
    pub(crate) created_at: Option<DateTime<Utc>>,
}