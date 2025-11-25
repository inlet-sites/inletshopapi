use serde::{Serialize, Deserialize, de::DeserializeOwned};
use mongodb::{
    Database,
    bson::{DateTime, Document, doc, oid::ObjectId}
};
use futures::stream::TryStreamExt;
use crate::app_error::AppError;

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub _id: ObjectId,
    pub vendor: ObjectId,
    pub name: String,
    pub tags: Vec<String>,
    pub images: Vec<String>,
    pub thumbnail: Option<String>,
    pub active: bool,
    pub archived: bool,
    pub created_at: DateTime,
    pub prices: Vec<Price>,
}

#[derive(Serialize, Deserialize)]
pub struct Price {
    pub _id: ObjectId,
    pub descriptor: String,
    pub price: i32,
    pub quantity: i32,
    pub shipping: i32,
    pub images: Vec<String>,
    pub purchase_option: PurchaseOption,
    pub archived: bool
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PurchaseOption {
    Ship,
    Buy,
    List
}

impl PurchaseOption {
    pub fn to_string(&self) -> String {
        match self {
            PurchaseOption::Ship => "ship".to_string(),
            PurchaseOption::Buy => "buy".to_string(),
            PurchaseOption::List => "list".to_string()
        }
    }
}

impl From<PurchaseOption> for String {
    fn from(p: PurchaseOption) -> Self {
        p.to_string()
    }
}

impl Product {
    pub async fn insert(&self, db: &Database) -> Result<(), AppError> {
        match db.collection::<Product>("products").insert_one(self).await {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::Database(e.into()))
        }
    }

    pub async fn find_by_id<P>(
        db: &Database,
        id: ObjectId,
        vendor: Option<ObjectId>,
        proj: Document
    ) -> Result<P, AppError> 
        where
            P: DeserializeOwned + Send + Sync + Unpin
    {
        let find_doc = match vendor {
            Some(v) => doc!{"_id": id, "vendor": v},
            None => doc!{"_id": id}
        };

        match db.collection::<P>("products")
            .find_one(find_doc)
            .projection(proj)
            .await {
                Ok(Some(p)) => Ok(p),
                Ok(None) => Err(AppError::not_found("Product with this ID does not exist")),
                Err(e) => Err(AppError::Database(e.into()))
            }
    }

    pub async fn find_by_vendor<P>(
        db: &Database,
        vendor_id: ObjectId,
        proj: Document,
        page: u64,
        results: u64
    ) -> Result<Vec<P>, AppError> 
    where
        P: DeserializeOwned + Send + Sync + Unpin
    {
        let cursor = db.collection::<P>("products")
            .find(doc!{"vendor": vendor_id})
            .skip(page * results as u64)
            .limit(results as i64)
            .projection(proj)
            .await?;

        let products: Vec<P> = cursor.try_collect().await?;
        Ok(products)
    }

    pub async fn update(
        db: &Database,
        product_id: ObjectId,
        vendor_id: Option<ObjectId>,
        updates: Document
    ) -> Result<(), AppError> {
        let filter = match vendor_id {
            Some(v) => doc!{"_id": product_id, "vendor": v},
            None => doc!{"_id": product_id}
        };

        match db.collection::<Product>("products").update_one(filter, updates).await {
            Ok(ur) if ur.matched_count == 1 => Ok(()),
            Ok(_) => Err(AppError::forbidden("You do not have permissions for this product")),
            Err(e) => Err(AppError::Database(e.into()))
        }
    }

    pub async fn delete(db: &Database, id: ObjectId, vendor: ObjectId) -> Result<(), AppError> {
        match db.collection::<Product>("products").find_one_and_delete(doc!{"_id": id, "vendor": vendor}).await {
            Ok(Some(_)) => Ok(()),
            Ok(None) => Err(AppError::forbidden("You do not have authorization for this product")),
            Err(e) => Err(AppError::Database(e.into()))
        }
    }

    pub async fn verify_ownership(db: &Database, product_id: ObjectId, vendor_id: ObjectId) -> Result<(), AppError> {
        match db.collection::<Document>("products")
            .find_one(doc!{"_id": product_id, "vendor": vendor_id})
            .projection(doc!{"_id": 1})
            .await {
                Ok(Some(_)) => Ok(()),
                Ok(None) => Err(AppError::forbidden("You do not have authorization for this product")),
                Err(e) => Err(AppError::Database(e.into()))
            }
    }
}

impl Price {
    pub fn new(
        descriptor: String,
        price: i32,
        quantity: i32,
        shipping: i32,
        purchase_option: PurchaseOption
    ) -> Price {
        Price {
            _id: ObjectId::new(),
            descriptor: descriptor,
            price: price,
            quantity: quantity,
            shipping: shipping,
            images: Vec::new(),
            purchase_option: purchase_option,
            archived: false
        }
    }

    pub async fn delete(
        db: &Database,
        product_id: ObjectId,
        price_id: ObjectId,
        vendor_id: Option<ObjectId>
    ) -> Result<(), AppError> {
        let find_doc = match vendor_id {
            Some(_) => doc!{"_id": product_id, "vendor": vendor_id},
            None => doc!("_id": product_id)
        };
        let update_doc = doc!{"$pull": {"prices": {"_id": price_id}}};

        match db.collection::<Product>("products")
            .update_one(find_doc, update_doc)
            .await {
                Ok(ur) if ur.matched_count == 1  => Ok(()),
                Ok(_) => Err(AppError::forbidden("Invalid permisssions for this product")),
                Err(e) => Err(AppError::Database(e.into()))
            }
    }
}
