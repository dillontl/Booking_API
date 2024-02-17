use crate::database::db_connection;
use axum::Json;
mod se

// use mysql::*;
pub async fn get_services() -> Result<Json<Vec<models::Services>>, Box<dyn std::error::Error>> {
    println!("Retrieving services...");
    let conn = db_connection();
    let services = conn.query_map(
        "SELECT ServiceID, Name, Description, Price, Duration FROM Services",
        |(
             service_id,
             name,
             description,
             price,
             duration,
         )| {
            Services {
                service_id,
                name,
                description,
                price,
                duration,
            }
        },
    )?;
    for service in &services {
        println!("Result: {:?}", service);
    }
    Ok(Json(services))
}