use mongodb::{
    bson::{self, doc, Document},
    options::{FindOneOptions, FindOptions, InsertManyOptions, InsertOneOptions}, 
    Database
};
use super::error::DatabaseError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};


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


pub async fn get_model_by_id<T>(database: &Database, collection: &str, id: &str) -> Result<Option<T>, DatabaseError>
where T:
    DeserializeOwned
{
    let collection = database.collection(collection);

    let filter = doc! {"id": id};
    let options = FindOneOptions::default();

    let result = collection.find_one(filter, options)
        .await
        .map_err(|err| DatabaseError::MongoError(err));

    match result{
        Ok(Some(doc)) => {
            let model: T = bson::from_bson(bson::Bson::Document(doc)).unwrap();
            Ok(Some(model))
        },
        Ok(None) => Ok(None),
        Err(err) => Err(err)
    }
    
}


type ManyModelsResult<T> = Option<Vec<T>>;

pub async fn get_many_models<T>(database: &Database, collection: &str, query: &Document, limit: i64) -> Result<ManyModelsResult<T>, DatabaseError>
where T:
    DeserializeOwned
{ 
    let collection = database.collection(collection);

    let options = FindOptions::builder()
                    .limit(limit)
                    .build();

    let output: Vec<T> = Vec::new();
        
    let mut cursor = collection.find(*query, options).await
        .map_err(|err| DatabaseError::MongoError(err))?;

    while let Some(result) = cursor.try_next().await

}


