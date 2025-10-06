use actix_web::{HttpServer, web};
use mongodb::{Client, Database};

#[actix_web::main]
async fn main() {
    let uri = if node_env == "production" {
        std::env::var("MONGO_URI").expect("MONGO_URI must be set in production")
    } else {
        "mongodb://127.0.0.1:27017".to_string()
    };
    let db = connect_db(&uri, "suma").await;

    HttpServer::new (move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(routes::user::config)
    })
        .bind(("0.0.0.0", 8001))?
        .run()
        .await
}

pub async fn connect_db(uri: &str, db_name: &str) -> Database {
    let client = Client::with_uri_str(uri).await.expect("Failed to connect to database");
    client.database(db_name)
}
