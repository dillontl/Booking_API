use std::error::Error;
use mysql::params;
use mysql::prelude::Queryable;
use chrono::{DateTime, Utc};
use log::{error, info};
use crate::database::db_connection;
use crate::models::services_model::{EditService, NewService, Services};

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
        println!("Successfully created services: {}", service.name);
    }
    Ok(())
}

pub fn edit_services( services: &[EditService]) -> Result<(), Box<dyn Error>> {
    let mut conn = db_connection()?;

    for service in services {
        let query = r"UPDATE Services SET Name = :name, Description = :description, Price = :price, Duration = :duration
        WHERE ServiceID = :service_id";
        let current_datetime: DateTime<Utc> = Utc::now();
        let formatted_datetime = current_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        match conn.exec_drop(query, params! {
            "service_id" => &service.service_id,
            "name" => &service.name,
            "description" => &service.description,
            "price" => service.price,
            "duration" => service.duration,
            "updated_at" => formatted_datetime,
        }) {
            Ok(_) => {
                info!("Service with ID {} updated successfully", service.service_id);
                println!("Service with ID {} updated successfully", service.service_id);
            }
            Err(e) => {
                error!("Failed to update service with ID {}: {}", service.service_id, e);
                return Err(Box::new(e));
            }
        }
    }

    Ok(())
}
