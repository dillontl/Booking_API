use std::error::Error;
use mysql::prelude::Queryable;
use crate::database::db_connection;
use crate::models::services_model::Services;

// use mysql::*;
pub async fn get_services() -> Result<Vec<Services>, Box<dyn Error>> {
    let mut conn = db_connection()?;
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
    Ok(services)
}
