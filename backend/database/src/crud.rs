use std::pin::Pin;

use mongodb::{bson::doc, options::{FindOneOptions, InsertManyOptions, InsertOneOptions}, Database};
use super::error::DatabaseError;
use serde::{Deserialize, Serialize};


pub async fn save_model<T>(database: &Database, collection: &str, model: T) -> Result<(), DatabaseError> 
where T:
    Serialize + Deserialize<'static>
{
    let collection = database.collection(collection);
    let options = InsertOneOptions::default();

    collection.insert_one(model, options).await.map_err(|err| {
        DatabaseError::MongoError(err)
    })?;

    Ok(())
}

pub async fn save_multiple_models<T>(database: &Database, collection: &str, models: Vec<T>) -> Result<(), DatabaseError>
where T:
    Serialize + Deserialize<'static>
{
    let collection = database.collection(collection);
    let options = InsertManyOptions::default();

    collection.insert_many(models, options).await.map_err(|err| {
        DatabaseError::MongoError(err)
    })?;

    Ok(())
}


pub async fn get_model_by_id<T>(database: &Database, collection: &str, id: &str) -> Result<Pin<Box<T>>, DatabaseError>
where T:
    Deserialize<'static>
{
    let collection = database.collection(collection);

    let filter = doc! {"id": id};
    let options = FindOneOptions::default();

    let result: Option<Box<T>> = collection.find_one(filter, options).await.map_err(|err| {
        DatabaseError::MongoError(err)
    })?;

    match result {
        Some(model) => Ok(Pin::from(model)),
        None => Err(DatabaseError::custom("Model not found"))

    }

}


