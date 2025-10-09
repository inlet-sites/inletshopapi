use mongodb::bson::{
    DateTime,
    oid::ObjectId
};
use uuid::Uuid;
use crate::{
    models::vendor::{
        Vendor,
        PublicData
    }
};

pub fn create_vendor(has_pass: bool, token: Option<String>) -> Vendor {
    Vendor {
        _id: ObjectId::new(),
        email: String::from("john.doe@inletsites.dev"),
        owner: String::from("John Doe"),
        store: String::from("Inlet Sites"),
        url: String::from("inlet-sites"),
        pass_hash: if has_pass {Some("hash".to_string())}else{None},
        token: match token {
            Some(t) => t,
            None => Uuid::new_v4().to_string()
        },
        public_data: PublicData{
            phone: None,
            email: None,
            address: None,
            slogan: None,
            description: None,
            image: None,
            hours: None,
            links: None,
            website: None
        },
        html: None,
        active: true,
        new_order_send_email: false,
        stripe: None,
        created_at: DateTime::now()
    }
}
