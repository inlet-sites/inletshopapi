use actix_web::{HttpResponse, HttpRequest, web, put};
use mongodb::{Database, bson::{Bson, Document}};
use serde::Deserialize;
use crate::{
    models::vendor::Vendor,
    app_error::AppError,
    auth::vendor_auth
};

#[derive(Deserialize)]
struct Body {
    stripe_activated: Option<bool>,
    new_order_send_email: Option<bool>,
    public_data: Option<PublicData>
}

#[derive(Deserialize)]
struct PublicData {
    phone: Option<String>,
    email: Option<String>,
    address: Option<Address>,
    slogan: Option<String>,
    description: Option<String>,
    hours: Option<Hours>,
    links: Option<Vec<Link>>,
    website: Option<String>
}

#[derive(Deserialize)]
struct Address {
    text: Option<String>,
    link: Option<String>
}

#[derive(Deserialize)]
struct Hours {
    sunday: Option<Vec<String>>,
    monday: Option<Vec<String>>,
    tuesday: Option<Vec<String>>,
    wednesday: Option<Vec<String>>,
    thursday: Option<Vec<String>>,
    friday: Option<Vec<String>>,
    saturday: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Link {
    text: String,
    url: String
}

impl Into<Bson> for Link {
    fn into(self) -> Bson {
        let mut doc = Document::new();
        doc.insert("text", self.text);
        doc.insert("url", self.url);
        Bson::Document(doc)
    }
}

#[put("/vendor")]
pub async fn route(
    db: web::Data<Database>,
    body: web::Json<Body>,
    req: HttpRequest
) -> Result<HttpResponse, AppError> {
    let mut vendor = vendor_auth(&db, &req).await?;

    let updates = create_update_doc(body.into_inner());

    vendor = vendor.update(&db, updates).await?;
    vendor = Vendor::find_by_id(&db, vendor._id).await?;
    Ok(HttpResponse::Ok().json(vendor.response()))
}

fn create_update_doc(body: Body) -> Document {
    let mut doc = Document::new();

    if let Some(stripe_activated) = body.stripe_activated {
        let mut stripe_doc = Document::new();
        stripe_doc.insert("activated", stripe_activated);
        doc.insert("stripe", stripe_doc);
    }

    if let Some(new_order_send_email) = body.new_order_send_email {
        doc.insert("new_order_send_email", new_order_send_email);
    }

    if let Some(public_data) = body.public_data {
        let mut public_data_doc = Document::new();

        if let Some(phone) = public_data.phone {
            public_data_doc.insert("phone", phone);
        }

        if let Some(email) = public_data.email {
            public_data_doc.insert("email", email);
        }

        if let Some(address) = public_data.address {
            let mut address_doc = Document::new();

            if let Some(text) = address.text {
                address_doc.insert("text", text);
            }

            if let Some(link) = address.link {
                address_doc.insert("link", link);
            }

            public_data_doc.insert("address", address_doc);
        }

        if let Some(slogan) = public_data.slogan {
            public_data_doc.insert("slogan", slogan);
        }

        if let Some(description) = public_data.description {
            public_data_doc.insert("description", description);
        }

        if let Some(hours) = public_data.hours {
            public_data_doc.insert("hours", create_hours_doc(hours));
        }

        if let Some(links) = public_data.links {
            public_data_doc.insert("links", links);
        }

        if let Some(website) = public_data.website {
            public_data_doc.insert("website", website);
        }

        doc.insert("public_data", public_data_doc);
    }

    doc
}

fn create_hours_doc(hours: Hours) -> Document {
    let mut hours_doc = Document::new();

    if let Some(sunday) = hours.sunday {
        hours_doc.insert("sunday", sunday);
    }

    if let Some(monday) = hours.monday {
        hours_doc.insert("monday", monday);
    }

    if let Some(tuesday) = hours.tuesday {
        hours_doc.insert("tuesday", tuesday);
    }

    if let Some(wednesday) = hours.wednesday {
        hours_doc.insert("wednesday", wednesday);
    }

    if let Some(thursday) = hours.thursday {
        hours_doc.insert("thursday", thursday);
    }

    if let Some(friday) = hours.friday {
        hours_doc.insert("friday", friday);
    }

    if let Some(saturday) = hours.saturday {
        hours_doc.insert("saturday", saturday);
    }

    hours_doc
}

#[cfg(test)]
mod tests {
    use super::*;

    //create_update_doc
    #[test]
    fn creates_proper_data() {
        let body = Body {
            stripe_activated: Some(true),
            new_order_send_email: None,
            public_data: Some(PublicData {
                phone: None,
                email: Some(String::from("test@inletsites.dev")),
                address: None,
                slogan: Some(String::from("A new slogan")),
                description: None,
                hours: Some(Hours {
                    sunday: None,
                    monday: Some(vec![String::from("09:00"), String::from("17:00")]),
                    tuesday: None,
                    wednesday: None,
                    thursday: None,
                    friday: None,
                    saturday: None
                }),
                links: None,
                website: Some(String::from("https://inletsites.dev"))
            })
        };

        let doc = create_update_doc(body);

        let stripe_doc = doc.get_document("stripe").unwrap();
        assert_eq!(stripe_doc.get_bool("activated").unwrap(), true);

        let public_data_doc = doc.get_document("public_data").unwrap();
        assert_eq!(public_data_doc.get_str("email").unwrap(), "test@inletsites.dev");
        assert_eq!(public_data_doc.get_str("slogan").unwrap(), "A new slogan");
        assert_eq!(public_data_doc.get_str("website").unwrap(), "https://inletsites.dev");

        let hours_doc = public_data_doc.get_document("hours").unwrap();
        let monday = hours_doc.get_array("monday").unwrap();
        assert_eq!(monday[0], "09:00".into());
        assert_eq!(monday[1], "17:00".into());
    }
}
