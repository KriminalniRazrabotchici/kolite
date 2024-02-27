use mongodb::Database;

use crate::database::{error::DatabaseError, crud};
use super::model::User;


pub async fn create_user(database: Database, user: User) -> Result<(), DatabaseError> {
    crud::save_model(&database, "users", user).await
}
