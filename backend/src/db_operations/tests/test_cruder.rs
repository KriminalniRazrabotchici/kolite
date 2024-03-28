#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use database::{auth::DatabaseConnectionData, handler::DatabaseHandler};
    use mongodb::bson::doc;
    use crate::{db_operations::{cruder::Cruder, savable::Savable}, users::model::User};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestModel {
        id: String,
    }

    impl Savable for TestModel {
        const COLLECTION_NAME: &'static str = "test";
        const VALIDATION_RULES: Option<mongodb::bson::Document> = None;
    }

    async fn get_db_handler() -> DatabaseHandler {
        let db_conn = DatabaseConnectionData::new("test".to_string(), "localhost".to_string(), 27017, None); 

        DatabaseHandler::new(db_conn).await.unwrap()
    }
    
    #[tokio::test]
    async fn test_creating_cruder() {
        let db_handler = get_db_handler().await; 

        let cruder = Cruder::from(db_handler);

        assert_eq!(cruder.get_db_handler().get_database().name(), "test");
    }

    #[tokio::test]
    async fn test_saving_user_model() {
        let db_handler = get_db_handler().await; 

        let cruder = Cruder::from(db_handler);

        let user = User::new("test_user", "test_password", "test_email");

        let result = cruder.save(&user).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_getting_user_model() {
        let db_handler = get_db_handler().await; 
        let cruder = Cruder::from(db_handler);

        let user = User::new("test1", "test_password", "test_email");
        let result = cruder.save(&user).await;

        assert_eq!(result.is_ok(), true);

        let query = Some(doc!{"name": "test1"});

        let user = cruder.get_one::<User>(query).await.unwrap();

        assert_eq!(user.is_some(), true);
    }

    #[tokio::test]
    async fn test_getting_many_user_models() {
        let db_handler = get_db_handler().await; 

        let cruder = Cruder::from(db_handler);

        let user1 = User::new("test_get", "test_password", "test_email");
        let user2 = User::new("test_get1", "test_password2", "test_email2");

        let users = vec![&user1, &user2];

        let result = cruder.save_many(users).await;

        assert_eq!(result.is_ok(), true);

        let query = Some(doc! {
            "name": {
                "$regex": "^test_get",
                "$options": "i", // case-insensitive matching
            }
        });

        let users = cruder.get_many::<User>(query, None).await.unwrap();

        assert_eq!(users.len(), 2);
    }

    #[tokio::test]
    async fn test_getting_user_by_id() {
        let db_handler = get_db_handler().await; 

        let cruder = Cruder::from(db_handler);

        let model = TestModel {
            id: "test".to_string()
        };

        let result = cruder.save(&model).await;

        assert_eq!(result.is_ok(), true);

        let user = cruder.get_by_id::<TestModel>(&model.id).await.unwrap();

        assert_eq!(user.is_some(), true);
    }

    #[tokio::test]
    async fn test_deleting_model_by_id() {
        let db_handler = get_db_handler().await; 
        let cruder = Cruder::from(db_handler);

        let model = TestModel {
            id: "test_deletion".to_string()
        };

        let result = cruder.save(&model).await;
        assert_eq!(result.is_ok(), true);

        let result = cruder.delete_one::<TestModel>(doc!{"id": &model.id}).await;
        assert_eq!(result.is_ok(), true);

        let model = cruder.get_by_id::<TestModel>(&model.id).await.unwrap();

        assert_eq!(model.is_none(), true);
    }

    #[tokio::test]
    async fn test_deleting_many_models() {
        let db_handler = get_db_handler().await; 
        let cruder = Cruder::from(db_handler);

        let model1 = TestModel {
            id: "test_deletion1".to_string()
        };

        let model2 = TestModel {
            id: "test_deletion2".to_string()
        };

        let models = vec![&model1, &model2];

        let result = cruder.save_many(models).await;
        assert_eq!(result.is_ok(), true);

        let query = doc!{"id": {"$regex": "^test_deletion"} };

        let result = cruder.delete_many::<TestModel>(query.clone()).await;
        assert_eq!(result.is_ok(), true);

        let models = cruder.get_many::<TestModel>(Some(query), None).await.unwrap();
        assert_eq!(models.len(), 0);
    }
}
