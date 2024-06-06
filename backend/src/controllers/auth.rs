use actix_web::{post, web, Responder, HttpResponse};

#[post("/register")]
async fn register() -> impl Responder {
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
