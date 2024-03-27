struct Cruder(DatabaseHandler);

use database::handler::DatabaseHandler;
use serde::{de::DeserializeOwned, Serialize};

use crate::errors::CrudError;

use super::savable::Savable;

impl From<DatabaseHandler> for Cruder {
    fn from(value: DatabaseHandler) -> Self {
        Cruder(value)
    }
}

impl Cruder {
    fn new(db: DatabaseHandler) -> Self {
        Cruder::from(db)
    }

    async fn save<T: Savable> (&self, model: &T) -> Result<(), CrudError>{
        let collection = model.get_create_collection(&self.0).await?;
        
        if let Err(e) = collection.save_one(model).await {
            Err(CrudError::DatabaseError(e))
        }else {
            Ok(()) 
        }

    }

    async fn save_many<'a, T, I>(&self, models: I) -> Result<(), CrudError> 
    where 
        T: 'a +  Savable,
        I: 'a +  IntoIterator<Item = &'a T>,
    {

        // Creating a peekable iterator
        let mut iter = models.into_iter().peekable();

        // Peeking the first value, without consuming it
        let first_model = match iter.peek() {
            Some(model) => *model,
            None => return Err(CrudError::CustomError("The Iterator is empty, fam!".to_string())),
        };

        let collection = first_model.get_create_collection(&self.0).await?;

        if let Err(e) = collection.save_many(iter).await {
            Err(CrudError::DatabaseError(e))
        } else {
            Ok(())
        }
    }

    async fn get_one() {
        todo!()
    }

    async fn get_many() {
        todo!()
    }

    async fn get_by_id() {
        todo!()
    }

    async fn delete_one() {
        todo!()
    }

    async fn delete_many() {
        todo!()
    }
}
