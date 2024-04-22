mod users;
mod db_operations;
mod errors;
mod auth;
mod state;
mod cars;

use actix_web::{web, App, HttpRequest, HttpServer};
use actix_web::middleware::Logger;
use cars::api::routes::cars_scope;
use state::AppState;
use users::api::routes::users_scope;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let app_state = web::Data::new(AppState::init().await);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(
                web::resource("/")
                    .route(web::get().to(say_hello))
            )
            .service(web::scope("/user").configure(users_scope))
            .service(web::scope("/cars").configure(cars_scope))
            
    })
    .workers(4)
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

async fn say_hello(_req: HttpRequest) -> String {
    "You got all my love <3".to_string()
}
