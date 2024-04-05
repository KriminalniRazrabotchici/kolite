use actix_web::{web, HttpResponse, Responder};

use crate::{state::AppState, users::User};
use super::schemas::CreateUser;


pub fn users_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/register")
            .route(web::post().to(create_user))
    );
}


async fn create_user(user_in: web::Json<CreateUser>, app_state: web::Data<AppState>) -> impl Responder {
    let user: User = user_in.into_inner()
                            .into();

    let cruder = app_state.get_cruder();
    let result = cruder.save(&user).await;

    match result {
        Ok(_) => HttpResponse::Ok().body(format!("User {} created", user.get_name())),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e))
    }
}
