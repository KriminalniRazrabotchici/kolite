use crate::db_operations::savable::Savable;

use super::model::User;


impl Savable for User {
    const COLLECTION_NAME: &'static str = "users";

    const VALIDATION_RULES: Option<mongodb::bson::Document> = None;
}
