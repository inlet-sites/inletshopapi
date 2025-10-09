use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use mongodb::{Client, bson::doc};

/*async fn sead_test_data() {
    let client = Client::with_uri_str("mongodb://127.0.0.1:27017")
        .await
        .unwrap();
    let db = client.database("test");
    db.collection::<mongodb::bson::Document>("vendors")
        .insert_one(doc!{"name": "Test Vendor"}, None)
        .await
        .unwrap();
}*/

[#tokio::test]
async fn test_doc_route() {

}
