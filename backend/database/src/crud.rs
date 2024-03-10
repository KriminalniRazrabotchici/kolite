use mongodb::{
    bson::{doc, Document}, options::{FindOptions, InsertManyOptions, InsertOneOptions}, Collection
};
use super::error::DatabaseError;
use serde::{de::DeserializeOwned, Serialize};


pub async fn save_model<T>(collection: &Collection<T>, model: &T) -> Result<(), DatabaseError> 
where 
    T: Serialize,
{
    let options = InsertOneOptions::default();

    collection.insert_one(model, options).await.map_err(|err| {
        DatabaseError::MongoError(err)
    })?;

    Ok(())
}

pub async fn save_multiple_models<'a, I, T>(collection: &'a Collection<T>, models: I) -> Result<(), DatabaseError>
where 
    T: Serialize,
    I : IntoIterator<Item = &'a T>
{
    let options = InsertManyOptions::default();

    collection.insert_many(models, options).await.map_err(|err| {
        DatabaseError::MongoError(err)
    })?;

    Ok(())
}


pub async fn get_single_model<T>(collection: &Collection<T>, query: Document) -> Result<Option<T>, DatabaseError> 
where T:
    DeserializeOwned
{
    let options = FindOptions::builder()
                    .limit(1)
                    .build();

    let mut cursor = collection.find(query, options)
                    .await
                    .map_err(|err| DatabaseError::MongoError(err))?;

    let has_match_occured = if let Ok(result) = cursor.advance().await {
        result
    }else {
        false
    };

    if has_match_occured {
        let model = cursor.deserialize_current().map_err(|err| {DatabaseError::MongoError(err)})?;
        Ok(Some(model))
    }else {
        Ok(None)
    }
}

pub async fn get_model_by_id<T>(collection: &Collection<T>, id: &str) -> Result<Option<T>, DatabaseError>
where T:
    DeserializeOwned
{
    let filter = doc! {"id": id};

    get_single_model(collection, filter).await
}


type ManyModelsResult<T> = Option<Vec<T>>;

pub enum LimitResult {
    Limited(i64),
    Unlimited
}

pub async fn get_many_models<T>(collection: &Collection<T>, query: Option<Document>, limit_result: LimitResult) -> Result<ManyModelsResult<T>, DatabaseError>
where T:
    DeserializeOwned 
{ 
    let options_builder = FindOptions::builder();

    let options = if let LimitResult::Limited(limit) = limit_result {
        options_builder
            .limit(limit)
            .build()
    }else {
        options_builder
            .build()
    };

    

    let mut output: Vec<T> = Vec::new();
    
    let mut cursor = collection.find(query, options)
        .await
        .map_err(|err| DatabaseError::MongoError(err))?;

    while cursor.advance().await.map_err(|err| DatabaseError::MongoError(err))? {
        let current_model = cursor.deserialize_current()
            .map_err(|err| DatabaseError::MongoError(err))?;
        output.push(current_model);
    }

    if output.len() == 0 {
        Ok(None)
    } else {
        Ok(Some(output))
    }
}
