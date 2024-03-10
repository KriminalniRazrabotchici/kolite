use mongodb::Database;

use database::{error::DatabaseError, crud};
use super::model::User;



pub async fn create_user(database: Database, user: User) -> Result<(), DatabaseError> {
    todo!()
}
