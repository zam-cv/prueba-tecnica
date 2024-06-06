use crate::{models, utils};
use actix_web::{post, web, HttpResponse, Responder};
use validator::Validate;

#[post("/register")]
async fn register(user: web::Json<models::user::User>) -> impl Responder {
    if let Err(_) = user.validate() {
        return HttpResponse::BadRequest().body("Invalid data");
    }

    if let Ok(hash) = utils::hash_password(&user.password) {
        println!("Hash: {}", hash);
    }

    HttpResponse::Ok().body("Register")
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
