use std::error::Error;
use mysql::params;
use mysql::prelude::Queryable;
use chrono::{DateTime, Utc};
use crate::database::db_connection;
use crate::models::services_model::{NewService, Services};

pub async fn get_services() -> Result<Vec<Services>, Box<dyn Error>> {
    let mut conn = db_connection()?;
    let query = "SELECT ServiceID, Name, Description, Price, Duration FROM Services";

    let services = conn.query_map( query,
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
    println!("Successfully retrieved services!");
    Ok(services)
}

pub fn add_services(services: &[NewService]) -> Result<(), Box<dyn Error>> {
    let mut conn = db_connection()?;

    for service in services {
        let query = r"INSERT INTO Services (Name, Description, Price, Duration, CreatedAt)
                      VALUES (:name, :description, :price, :duration, :created_at)";

        let current_datetime: DateTime<Utc> = Utc::now();
        let formatted_datetime = current_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        conn.exec_drop(query, params! {
            "name" => &service.name,
            "description" => &service.description,
            "price" => service.price,
            "duration" => service.duration,
            "created_at" => formatted_datetime,
        })?;
    }
    println!("Successfully created services");
    Ok(())
}