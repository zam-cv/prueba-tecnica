use crate::database::Database;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/rooms")]
async fn get_rooms(database: web::Data<Database>) -> impl Responder {
    match database.get_rooms().await {
        Ok(rooms) => HttpResponse::Ok().json(rooms),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/rooms/{id}/best_solving_time")]
async fn get_best_solving_time(
    database: web::Data<Database>,
    id: web::Path<i32>,
) -> impl Responder {
    match database.get_best_solving_time(id.into_inner()).await {
        Ok(time) => HttpResponse::Ok().json(time),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn routes() -> actix_web::Scope {
    web::scope("")
        .service(get_rooms)
        .service(get_best_solving_time)
}
