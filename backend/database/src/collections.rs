use mongodb::{bson::{doc, Document}, options::{FindOneOptions, FindOptions, InsertManyOptions, InsertOneOptions}, Collection};
use serde::{de::DeserializeOwned,  Serialize};

use crate::errors::DatabaseError;

pub struct CollectionHandler<T> 
{
    collection: Collection<T>,
}

impl<T> From<Collection<T>> for CollectionHandler<T> 
where T: Serialize + DeserializeOwned
{
    fn from(collection: Collection<T>) -> Self {
        CollectionHandler {
            collection,
        }
    } 
}



impl <T> CollectionHandler<T>
where T: Serialize + DeserializeOwned
{
    pub async fn save_one(&self, model: &T) -> Result<(), DatabaseError> {
        let options = InsertOneOptions::default();

        let operation = self.collection.insert_one(model, options).await;

        match operation {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseError::CouldNotSaveError(format!("The model, couldn't be saved: {}", e.to_string()))),
        }
    }

    pub async fn save_many<I>(&self, models: Box<I>) -> Result<(), DatabaseError> 
    where I: IntoIterator<Item = T>
    {
        let options = InsertManyOptions::default();

        let operation = self.collection.insert_many(*models, options).await;

        match operation {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseError::CouldNotSaveError(format!("The models, couldn't be saved: {}", e.to_string()))),
        }
    }

    pub async fn get_many(&self, query: Option<Document>, limit: Option<i64>) -> Result<Vec<T>, DatabaseError> 
    {
        let options = FindOptions::builder()
            .limit(limit)
            .build();

        let mut cursor = self.collection.find(query, options).await
            .map_err(|e| DatabaseError::CursorError(e.to_string()))?;


        let mut output = Vec::new();
        while cursor.advance().await.map_err(|e| DatabaseError::CursorError(e.to_string()))? {
            let model = cursor.deserialize_current().map_err(|e| DatabaseError::CursorError(e.to_string()))?;
            output.push(model);
        }

        Ok(output)
    }
 

}

impl <T> CollectionHandler<T>
where T: DeserializeOwned + Unpin + Send + Sync
{
    pub async fn get_one(&self, query: Option<Document>) -> Result<Option<T>, DatabaseError> 
    {
        let options = FindOneOptions::default();

        let operation = self.collection.find_one(query, options).await;

        match operation {
            Ok(result) => Ok(result),
            Err(e) => Err(DatabaseError::CouldNotSaveError(format!("The model, couldn't be retrieved: {}", e.to_string()))),
        }
    }

    pub async fn get_one_with_id(&self, id: &str) -> Result<Option<T>, DatabaseError> 
    {
        let query = doc! {
            "id": id
        };

        self.get_one(Some(query)).await
    }
}

