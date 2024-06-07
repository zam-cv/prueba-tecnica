use crate::database::Database;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/rooms")]
async fn get_rooms(database: web::Data<Database>) -> impl Responder {
    match database.get_rooms().await {
        Ok(rooms) => HttpResponse::Ok().json(rooms),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn routes() -> actix_web::Scope {
    web::scope("").service(get_rooms)
}
