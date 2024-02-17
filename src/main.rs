mod database;
mod services;
mod models;
mod handlers;
use axum::{
    routing::get,
    Router,
};
use handlers::services_handler;
use crate::handlers::services_handler::add_services_handler;

#[tokio::main]
async fn main() {
    // Initialize DB Connection
    if let Err(err) = database::db_connection() {
        println!("Database connection error: {}", err);
    }

    let app = Router::new()
        .route("/", get(index))
        .route("/services",
               get(services_handler::get_services_handler)
                   .post(add_services_handler));

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let local_addr = listener.local_addr().unwrap();
    println!("Running app on: {}", local_addr);

    axum::serve(listener, app).await.unwrap();
}

// App initializer
async fn index() -> &'static str {
    "Booking API - Built by Ourtek Systems"
}
