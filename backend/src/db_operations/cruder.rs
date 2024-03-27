use database::handler::DatabaseHandler;
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

    pub async fn get_one() {
        todo!()
    }

    pub async fn get_many() {
        todo!()
    }

    pub async fn get_by_id() {
        todo!()
    }

    pub async fn delete_one() {
        todo!()
    }

    pub async fn delete_many() {
        todo!()
    }
}
