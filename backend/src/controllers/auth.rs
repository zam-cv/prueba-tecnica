use crate::{database::Database, models, utils};
use actix_web::{post, web, HttpResponse, Responder};
use validator::Validate;

#[post("/register")]
async fn register(
    mut user: web::Json<models::User>,
    database: web::Data<Database>,
) -> impl Responder {
    if let Err(_) = user.validate() {
        return HttpResponse::BadRequest().body("Invalid data");
    }

    if let Ok(Some(_)) = database.get_user_by_email(user.email.clone()).await {
        return HttpResponse::BadRequest().body("Email already exists");
    }

    if let Ok(hash) = utils::hash_password(&user.password) {
        user.password = hash;

        return match database.create_user(user.into_inner()).await {
            Ok(_) => HttpResponse::Ok().body("User created"),
            Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
        };
    } else {
        return HttpResponse::InternalServerError().body("Failed to hash password");
    }
}

#[post("/signin")]
async fn signin() -> impl Responder {
    HttpResponse::Ok().body("Signin")
}

#[post("/verify")]
async fn verify() -> impl Responder {
    HttpResponse::Ok().body("Verify")
}

pub fn routes() -> actix_web::Scope {
    web::scope("/auth")
        .service(register)
        .service(signin)
        .service(verify)
}
