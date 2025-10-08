use actix_web::{HttpServer, web, App};
use mongodb::{Client, Database};

mod routes;
mod models;
mod controllers;
mod dto;
mod app_error;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let node_env = std::env::var("NODE_ENV").unwrap_or_else(|_| "development".to_string());
    let uri = if node_env == "production" {
        std::env::var("MONGO_URI").expect("MONGO_URI must be set in production")
    } else {
        "mongodb://127.0.0.1:27017".to_string()
    };
    let db = connect_db(&uri, "suma").await;

    HttpServer::new (move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(routes::vendor::config)
    })
        .bind(("0.0.0.0", 8001))?
        .run()
        .await
}

pub async fn connect_db(uri: &str, db_name: &str) -> Database {
    let client = Client::with_uri_str(uri).await.expect("Failed to connect to database");
    client.database(db_name)
}
