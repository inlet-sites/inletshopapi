use actix_web::{HttpServer, web, App};
use mongodb::{Client, Database};
use crate::app_error::AppError;

mod routes;
mod models;
mod controllers;
mod app_error;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    let uri = if app_env == "production" {
        std::env::var("MONGO_URI").expect("MONGO_URI must be set in production")
    } else {
        "mongodb://127.0.0.1:27017".to_string()
    };
    let db = connect_db(&uri, "inletshop").await;

    //Spin up the server
    HttpServer::new (move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(
                web::JsonConfig::default().error_handler(|err, _req| {
                    AppError::JsonDeserializationError(err.to_string()).into()
                })
            )
            .configure(routes::other::config)
            .configure(routes::vendor::config)
    })
        .bind(("0.0.0.0", 8001))?
        .run()
        .await
}

async fn connect_db(uri: &str, db_name: &str) -> Database {
    let client = Client::with_uri_str(uri).await.expect("Failed to connect to database");
    client.database(db_name)
}
