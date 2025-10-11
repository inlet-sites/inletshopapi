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

#[get("/vendor")]
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
