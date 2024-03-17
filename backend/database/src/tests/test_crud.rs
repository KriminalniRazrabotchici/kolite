#[cfg(test)]
mod tests{
    use mongodb::bson::doc;
    use serde::{Deserialize, Serialize};
    use crate::tests::utils::get_database_handler;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SampleModel {
        id: String,
        name: String,
        age: u8,
    }


    impl SampleModel {
        pub fn new(id: String, name: String, age: u8) -> SampleModel {
            SampleModel {
                id,
                name,
                age,
            }
        }

        pub fn generate_many(amount: u8) -> Vec<SampleModel> {
            let mut models = Vec::new();

            for i in 0..amount {
                let model = SampleModel::new(i.to_string(), "John".to_string(), i);
                models.push(model);
            }

            models
        }
    }
    pub fn assert_models_are_equal(models: Vec<SampleModel>, get_result: Vec<SampleModel>) {
        for i in 0..models.len() {
            assert_eq!(models[i].id, get_result[i].id);
            assert_eq!(models[i].name, get_result[i].name);
            assert_eq!(models[i].age, get_result[i].age);
        }
    }

    #[tokio::test]
    async fn test_inserting_model_to_database() {
        let handler = get_database_handler().await;
        handler.create_collection("test_inserting_model_to_database", None).await.unwrap();

        let collection_handler = handler.get_collection::<SampleModel>("test_inserting_model_to_database").await.unwrap();

        let model = SampleModel::new("1".to_string(), "John".to_string(), 20);


        let insert_result = collection_handler.save_one(&model).await;

        match insert_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_inserting_many_models_to_database() {
        let handler = get_database_handler().await;
        handler.create_collection("test_inserting_many_models_to_database", None).await.unwrap();

        let collection_handler = handler.get_collection::<SampleModel>("test_inserting_many_models_to_database").await.unwrap();

        let models = SampleModel::generate_many(10);
        let models = Box::new(models);

        let insert_result = collection_handler.save_many(models.clone()).await;

        match insert_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }


    #[tokio::test]
    async fn test_getting_many_models_from_database() {
        let handler = get_database_handler().await;
        handler.create_collection("test_getting_many_models_from_database", None).await.unwrap();

        let collection_handler = handler.get_collection::<SampleModel>("test_getting_many_models_from_database").await.unwrap();

        let models = SampleModel::generate_many(10);
        let models = Box::new(models);

        let _ = collection_handler.save_many(models.clone()).await;

        let returned_models = collection_handler.get_many(None, None).await.unwrap();

        assert_models_are_equal(*models, returned_models);
    }


    #[tokio::test]
    async fn test_getting_many_models_with_query_and_limit() {
        let handler = get_database_handler().await;
        handler.create_collection("test_getting_many_models_with_query", None).await.unwrap();

        let collection_handler = handler.get_collection::<SampleModel>("test_getting_many_models_with_query").await.unwrap();

        let models = SampleModel::generate_many(10);
        let models = Box::new(models);

        let _ = collection_handler.save_many(models.clone()).await;

        let query = doc! {
            "age": {
                "$gt": 5,
            }
        };

        let limit = 5;

        let returned_models = collection_handler.get_many(Some(query), Some(limit)).await.unwrap();

        let expected_models: Vec<SampleModel> = models.into_iter().filter(|model| model.age > 5).collect();
        
        assert_models_are_equal(expected_models, returned_models);
    }

    #[tokio::test]
    async fn test_getting_one_model() {
        let model = SampleModel::new("1".to_string(), "John".to_string(), 20);

        let handler = get_database_handler().await;
        handler.create_collection("test_getting_one_model", None).await.unwrap();

        let collection_handler = handler.get_collection::<SampleModel>("test_getting_one_model").await.unwrap();
        let _ = collection_handler.save_one(&model).await;

        let query = doc! {
            "name": "John",
        };

        let result = collection_handler.get_one(Some(query)).await.unwrap();

        if let Some(model) = result {
            assert_eq!(model.id, "1");
            assert_eq!(model.name, "John");
            assert_eq!(model.age, 20);
        } else {
            assert!(false);
        }
    }

    #[tokio::test]
    async fn test_getting_one_model_with_id() {
        let handler = get_database_handler().await;
        handler.create_collection("test_getting_one_model_with_id", None).await.unwrap();

        let collection_handler = handler.get_collection::<SampleModel>("test_getting_one_model_with_id").await.unwrap();

        let model = SampleModel::new("1".to_string(), "John".to_string(), 20);

        collection_handler.save_one(&model).await.unwrap();

        let returned_model = collection_handler.get_one_with_id("1").await.unwrap();

        if let Some(model) = returned_model {
            assert_eq!(model.id, "1");
            assert_eq!(model.name, "John");
            assert_eq!(model.age, 20);
        } else {
            assert!(false);
        }
    }
}
