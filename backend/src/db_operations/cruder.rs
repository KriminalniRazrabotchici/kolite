use database::handler::DatabaseHandler;
use mongodb::bson::Document;
use crate::errors::CrudError;
use super::savable::Savable;


pub struct Cruder(DatabaseHandler);

impl From<DatabaseHandler> for Cruder {
    fn from(value: DatabaseHandler) -> Self {
        Cruder(value)
    }
}

impl Cruder {
    pub fn new(db: DatabaseHandler) -> Self {
        Cruder::from(db)
    }

    pub fn get_db_handler(&self) -> &DatabaseHandler {
        &self.0
    }

    pub async fn save<T: Savable> (&self, model: &T) -> Result<(), CrudError>{
        let collection = T::get_create_collection(&self.0).await?;
        
        if let Err(e) = collection.save_one(model).await {
            Err(CrudError::DatabaseError(e))
        }else {
            Ok(()) 
        }

    }

    pub async fn save_many<'a, T, I>(&self, models: I) -> Result<(), CrudError> 
    where 
        T: 'a +  Savable,
        I: 'a +  IntoIterator<Item = &'a T>,
    {

        let collection = T::get_create_collection(&self.0).await?;

        if let Err(e) = collection.save_many(models).await {
            Err(CrudError::DatabaseError(e))
        } else {
            Ok(())
        }
    }

    pub async fn get_one<T: Savable>(&self, query: Option<Document>) -> Result<Option<T>, CrudError> {
        let collection = T::get_create_collection(&self.0).await?;

        match collection.get_one(query).await {
            Ok(model) => Ok(model),
            Err(e) => Err(CrudError::DatabaseError(e))
        }
    }

    pub async fn get_many<T: Savable>(&self, query: Option<Document>, limit: Option<i64>) -> Result<Vec<T>, CrudError> {
        let collection = T::get_create_collection(&self.0).await?;

        match collection.get_many(query, limit).await {
            Ok(models) => Ok(models),
            Err(e) => Err(CrudError::DatabaseError(e))
        }
    }

    pub async fn get_by_id<T: Savable>(&self, id: &str) -> Result<Option<T>, CrudError> {
        let collection = T::get_create_collection(&self.0).await?;

        match collection.get_one_with_id(id).await {
            Ok(model) => Ok(model),
            Err(e) => Err(CrudError::DatabaseError(e))
        }
        
    }

    pub async fn delete_one<T: Savable>(&self, query: Document) -> Result<(), CrudError> {
        let collection = T::get_create_collection(&self.0).await?;

        if let Err(e) = collection.delete_one(query).await {
            Err(CrudError::DatabaseError(e))
        } else {
            Ok(())
        }

    }

    pub async fn delete_many<T: Savable>(&self, query: Document) -> Result<(), CrudError> {
        let collection = T::get_create_collection(&self.0).await?;

        if let Err(e) = collection.delete_many(query).await {
            Err(CrudError::DatabaseError(e))
        } else {
            Ok(())
        }
    }
}
