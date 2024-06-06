use crate::{database::Database, models, utils, config};
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[post("/register")]
async fn register(
    mut user: web::Json<models::User>,
    database: web::Data<Database>,
) -> impl Responder {
    // Validate the user data
    if let Err(_) = user.validate() {
        return HttpResponse::BadRequest().body("Invalid data");
    }

    // Check if the email already exists
    if let Ok(Some(_)) = database.get_user_by_email(user.email.clone()).await {
        return HttpResponse::BadRequest().body("Email already exists");
    }

    // Hash the password
    if let Ok(hash) = utils::hash_password(&user.password) {
        user.password = hash;

        // Create the user
        return match database.create_user(user.into_inner()).await {
            Ok(_) => HttpResponse::Ok().body("User created"),
            Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
        };
    } else {
        return HttpResponse::InternalServerError().body("Failed to hash password");
    }
}

#[post("/signin")]
async fn signin(
    credentials: web::Json<Credentials>,
    database: web::Data<Database>,
) -> impl Responder {
    // Get the user by email
    let user = match database.get_user_by_email(credentials.email.clone()).await {
        Ok(Some(user)) => user,
        _ => return HttpResponse::BadRequest().body("Invalid credentials"),
    };

    // Verify the password
    if let Ok(true) = utils::verify_password(&credentials.password, &user.password) {
        if let Some(id) = user.id {
            // Create a token
            if let Ok(token) = utils::create_token(&config::SECRET_KEY, id) {
                return HttpResponse::Ok().body(token);
            }
        }
    };

    HttpResponse::BadRequest().body("Invalid credentials")
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
