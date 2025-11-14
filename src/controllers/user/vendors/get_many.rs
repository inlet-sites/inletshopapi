use actix_web::{HttpResponse, web, get};
use mongodb::{
    Database,
    bson::{
        Document,
        doc
    }
};
use serde::{Serialize, Deserialize};
use crate::{
    models::vendor::Vendor,
    app_error::AppError
};
#[cfg(test)]
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize)]
struct ResponseVendor {
    id: String,
    store: String,
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_data: Option<ResponsePublicData>
}

#[derive(Serialize, Deserialize)]
struct ResponsePublicData {
    #[serde(skip_serializing_if = "Option::is_none")]
    slogan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>
}

#[get("/user/vendors")]
pub async fn route(
    db: web::Data<Database>
) -> Result<HttpResponse, AppError> {
    //Gather data
    let projection_doc = create_projection_document();
    let vendors = Vendor::get_all(&db, projection_doc).await?;

    let response = create_response(vendors);
    
    //Respond
    Ok(HttpResponse::Ok().json(response))
}

fn create_projection_document() -> Document {
    doc!{
        "_id": 1,
        "store": 1,
        "url": 1,
        "public_data.slogan": 1,
        "public_data.image": 1
    }
}

fn create_response(vendors: Vec<Document>) -> Vec<ResponseVendor> {
    vendors.into_iter().map(|v| {
        let p = match v.get_document("public_data") {
            Ok(p) if p.is_empty() => None,
            Ok(p) => Some(p),
            Err(_) => None
        };

        ResponseVendor {
            id: v.get_object_id("_id").unwrap().to_hex(),
            store: v.get_str("store").unwrap_or("").to_string(),
            url: v.get_str("url").ok().unwrap_or("").to_string(),
            public_data: match p {
                Some(d) => {
                    Some(ResponsePublicData {
                        slogan: d.get_str("slogan").ok().map(String::from),
                        image: d.get_str("image").ok().map(String::from)
                    })
                },
                None => None
            }
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    //create_projection_document
    #[test]
    fn correct_projection_data() {
        let result = create_projection_document();

        assert!(result.contains_key("_id"));
        assert!(result.contains_key("store"));
        assert!(result.contains_key("url"));
        assert!(result.contains_key("public_data.slogan"));
        assert!(result.contains_key("public_data.image"));
        assert!(result.len() == 5);
    }

    //create_response
    fn create_test_documents(public_data: bool) -> Vec<Document> {
        let mut docs = Vec::new();

        for _i in 0..10 {
            let mut new_doc = doc!{
                "_id": ObjectId::new(),
                "store": "Example Store",
                "url": "example-store",
            };

            if public_data {
                new_doc.insert("public_data", doc!{
                    "slogan": "Some slogan here",
                    "image": "/image/12345"
                });
            }

            docs.push(new_doc);
        }

        docs
    }

    #[test]
    fn maps_fields_correctly() {
        let docs = create_test_documents(false);
        let docs_len = docs.len();
        let docs_value = docs[1].get_object_id("_id").unwrap().to_hex();
        let result = create_response(docs);

        assert_eq!(result.len(), docs_len);
        assert_eq!(result[0].store, "Example Store");
        assert_eq!(result[1].id, docs_value);
    }

    #[test]
    fn creates_public_data() {
        let docs = create_test_documents(true);
        let value_one = docs[1]
            .get_document("public_data").unwrap()
            .get_str("slogan").unwrap()
            .to_string();
        let value_two = docs[3]
            .get_document("public_data").unwrap()
            .get_str("image").unwrap()
            .to_string();
        let result = create_response(docs);

        assert_eq!(
            result[1].public_data.as_ref().unwrap().slogan.as_ref().unwrap(),
            &value_one
        );
        assert_eq!(
            result[3].public_data.as_ref().unwrap().image.as_ref().unwrap(),
            &value_two
        );
    }
}
