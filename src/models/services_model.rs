#[derive(Debug, Serialize, Deserialize)]
pub struct Services {
    service_id: i32,
    name: String,
    description: String,
    price: f64,
    duration: f64,
}
