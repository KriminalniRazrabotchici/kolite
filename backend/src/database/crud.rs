use mongodb::{options::{InsertManyOptions, InsertOneOptions}, Database};
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

pub async fn save_multuple_models<T>(database: &Database, collection: &str, models: Vec<T>) -> Result<(), DatabaseError>
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
