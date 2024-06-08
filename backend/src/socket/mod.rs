use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web_actors::ws;

pub mod server;
pub mod session;

pub async fn server_index(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<server::ServerHandle>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let room_id = path.into_inner();

    // The id was obtained from the token when authenticating
    if let Some(user_id) = req.extensions().get::<i32>() {
        return ws::start(
            session::Session {
                user_id: *user_id,
                srv: srv.get_ref().clone(),
                room_id,
            },
            &req,
            stream,
        );
    }

    Ok(HttpResponse::Unauthorized().finish())
}
