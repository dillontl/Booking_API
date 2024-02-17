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

#[derive(Debug, Serialize, Deserialize)]
pub struct NewService {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) price: f32,
    pub(crate) duration: f64,
    pub(crate) created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditService {
    pub(crate) service_id: i32,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) price: f32,
    pub(crate) duration: f64,
    pub(crate) updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteService {
    pub(crate) service_id: i32,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum ServicesInput {
    Single(NewService),
    Multiple(Vec<NewService>),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum EditServicesInput {
    Single(EditService),
    Multiple(Vec<EditService>),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum DeleteServicesInput {
    Single(DeleteService),
    Multiple(Vec<DeleteService>),
}