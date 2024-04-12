use std::env;

use crate::users::hasher::Hasher;
use actix_web::{web, HttpResponse, Responder};
use jsonwebtoken::Algorithm;
use mongodb::bson::doc;
use crate::auth::Authenticator; 

use crate::{state::AppState, users::User};
use super::schemas::{CreateUser, ErrorResponse, LoginUserData, Refresh, SuccessfulLoginResponse};


pub fn users_scope(cfg: &mut web::ServiceConfig) {
    // Register a new user
    cfg.service(
        web::resource("/register")
            .route(web::post().to(create_user))
    );

    // Login a user
    cfg.service(
        web::resource("/login")
            .route(web::post().to(login_user))
    );

    // Refresh a token
    cfg.service(
        web::resource("/refresh")
            .route(web::post().to(refresh_token))
    );
}


async fn create_user(user_in: web::Json<CreateUser>, app_state: web::Data<AppState>) -> impl Responder {
    let user: User = user_in.into_inner()
                            .into();

    let cruder = app_state.get_cruder();
    
    match cruder.get_one::<User>(Some(doc! {"email": user.get_email()})).await {
        Ok(Some(_)) => HttpResponse::BadRequest().json(ErrorResponse::new("A user with this email already exists")),
        Err(e) => HttpResponse::InternalServerError().body(format!("Internal server error: {}", e.to_string())),
        Ok(None) => {    
            let result = cruder.save(&user).await;

            match result {
                Ok(_) => HttpResponse::Ok().body(format!("User {} created", user.get_name())),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e))
            }
        }
    }

}

async fn login_user(user_in: web::Json<LoginUserData>, app_state: web::Data<AppState>) -> impl Responder {
    let secret = env::var("SECRET").unwrap();
    let authenticator = Authenticator::new(Algorithm::HS256, secret);


    let cruder = app_state.get_cruder();

    let user_login_data = user_in.into_inner();
    let query = doc! {"email" : user_login_data.email};
    let user_result = cruder.get_one::<User>(Some(query)).await;

    let user: User = match user_result {
        Ok(Some(result))=> {
            let hasher = Hasher::new();

            if hasher.check_password(&user_login_data.password, *result.get_password()) {
                result
            } else {
                return HttpResponse::BadRequest().json(ErrorResponse::new("Incorrect password"));
            }
        },
        Ok(None) => return HttpResponse::BadRequest().json(ErrorResponse::new("There is no such user in the database!")),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Interal server error: {}", e.to_string()))
    };

    let authentication_details = match authenticator.issue_token(&user) {
        Ok(auth) => auth,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Internal server error: {}", e.to_string()))
    };

    HttpResponse::Ok().json(SuccessfulLoginResponse::new("Successful login, brother!", authentication_details))
}


async fn refresh_token(ref_token: web::Json<Refresh>, app_state: web::Data<AppState>) -> impl Responder {
    let secret = env::var("SECRET").unwrap();
    let authenticator = Authenticator::new(Algorithm::HS256, secret);

    let cruder = app_state.get_cruder();

    let refresh_token = ref_token.into_inner().refresh_token;
    let auth_details = match authenticator.refresh(refresh_token, cruder).await {
        Ok(auth) => auth,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Internal server error: {}", e.to_string()))
    };

    HttpResponse::Ok().json(SuccessfulLoginResponse::new("Token refreshed", auth_details))
}
