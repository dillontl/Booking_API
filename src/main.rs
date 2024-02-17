mod database;
mod services;
use axum::{
    routing::get,
    Router,
    // Json,
};
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    // Initialize DB Connection
    if let Err(err) = database::db_connection() {
        println!("Database connection error: {}", err);
    }

    let app = Router::new()
        .route("/", get(index))
        .route("/services", get(get_all_services));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

// App initializer
async fn index() -> &'static str {
    "Booking API - Built by OurTek Systems"
}
