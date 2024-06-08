use crate::{config, utils};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage, HttpResponse,
};
use actix_web_lab::middleware::Next;

pub async fn auth(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let token = if let Some(header) = req
        .headers()
        // Get the Authorization header
        .get("Authorization")
    {
        header.to_str().map(|s| s.to_string()).ok()
    } else {
        // support for websockets
        req.headers()
            .get(actix_web::http::header::SEC_WEBSOCKET_PROTOCOL)
            .and_then(|header| {
                header
                    .to_str()
                    .map(|s| {
                        let parts: Vec<&str> = s.split(", ").collect();

                        if parts.len() == 2 {
                            let token = parts[1];
                            return Some(token.to_string());
                        }

                        None
                    })
                    .ok()
                    .flatten()
            })
    };

    if let Some(token) = token {
        if let Ok(claims) = utils::decode_token(&config::SECRET_KEY, &token) {
            // Check if the token has expired
            if claims.exp < chrono::Utc::now().timestamp() as usize {
                let response = HttpResponse::Unauthorized().finish();
                return Ok(req.into_response(response).map_into_right_body());
            }

            // Insert the user id into the request extensions
            req.extensions_mut().insert(claims.id);
            return Ok(next.call(req).await?.map_into_left_body());
        }
    }

    log::error!("Unauthorized");
    let response = HttpResponse::Unauthorized().finish();
    Ok(req.into_response(response).map_into_right_body())
}
