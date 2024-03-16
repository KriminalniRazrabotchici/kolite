use mongodb::{Collection};
use serde::{de::DeserializeOwned,  Serialize};

pub struct CollectionHandler<T> 
where T: Serialize + DeserializeOwned 
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

