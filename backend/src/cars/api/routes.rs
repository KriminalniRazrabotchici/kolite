use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::{doc, Document}}; 

use crate::state::AppState;

pub fn cars_scope(cfg: &mut web::ServiceConfig) {

    // get cars specifications 
    cfg.service(
        web::resource("/{spec}")
            .route(web::get().to(get_car_specifications))
    );

    cfg.service(
        web::resource("/{brand}/models")
            .route(web::get().to(get_car_models))
    );
}

/// Get a car specification
/// A car specification is defined as one of the following:
///     - coupes
///     - fuel
///     - wheel
///     - extras
///     - doors
///     - colors
///     - transmission
///     - brands
/// These are all endpoints that can be hit.
///
async fn get_car_specifications(spec: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    let spec = spec.into_inner();

    let aggregation_pipeline = [
        doc! {
            "$match": doc! {
                "name": &spec
            }
        },
        doc! {
            "$project": doc! {
                "_id": 0,
                &spec: "$values"
            }
        }
    ];

    execute_aggregation(app_state, "specs",  aggregation_pipeline).await
}


async fn get_car_models(brand: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder { 
    let brand = brand.into_inner();

    let aggregation_pipeline = [
        doc! {
            "$match": doc! {
                "brand": &brand
            }
       },
        doc! {
            "$project": doc! {
                "_id": 0,
                "models": 1
            }
        }
    ];

    execute_aggregation(app_state, "brands", aggregation_pipeline).await

}

async fn execute_aggregation<I>(app_state: web::Data<AppState>, collection_name: &str, aggregation_pipeline: I) -> impl Responder 
where
    I: IntoIterator<Item = Document>
{
    let collection_result = app_state.get_cruder()
                    .get_db_handler()
                    .get_collection::<Document>(collection_name).await;    

    match collection_result {
        Ok(handler) => {
            let result = handler.aggregate(aggregation_pipeline).await;

            let pipeline_result = if let Ok(documents) = result {
                documents.iter().next().cloned()
            } else {
                return HttpResponse::BadRequest().body("Ti mojesh li da pishesh?")
            };

            if let Some(data) = pipeline_result {
                HttpResponse::Ok().json(data)
            } else { 
                HttpResponse::InternalServerError().body("Maikata si eba")
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Maikata si eba ama na drugo mesto")
    }
}
