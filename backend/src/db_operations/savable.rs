use database::{collections::CollectionHandler, handler::DatabaseHandler};
use mongodb::bson::Document;
use serde::{de::DeserializeOwned, Serialize};

use crate::errors::CrudError;

pub trait Savable: Serialize + DeserializeOwned + Send + Sync + Unpin
{ 
    const COLLECTION_NAME: &'static str;
    const VALIDATION_RULES: Option<Document>;

    async fn get_create_collection(db: &DatabaseHandler) -> Result<CollectionHandler<Self>, CrudError> {
        if let Ok(collection) = db.get_collection(&Self::COLLECTION_NAME).await {
            Ok(collection)
        }else {
            let collection_result = db.create_collection(&Self::COLLECTION_NAME, Self::VALIDATION_RULES).await;

            match collection_result {
                Ok(_) => {
                    db.get_collection(&Self::COLLECTION_NAME).await
                        .map_err(|e| CrudError::DatabaseError(e))
                }
                Err(e) => Err(CrudError::DatabaseError(e))
            }
        }
    }
}
