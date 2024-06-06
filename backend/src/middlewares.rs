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
    if let Some(Some(token)) = req
        .headers()
        // Get the Authorization header
        .get("Authorization")
        .map(|t| t.to_str().ok().map(|s| s.to_string()))
    {
        if let Ok(claims) = utils::decode_token(&config::SECRET_KEY, &token) {
            // Check if the token has expired
            if claims.exp < chrono::Utc::now().timestamp() as usize {
                let response = HttpResponse::Unauthorized().finish();
                return Ok(req.into_response(response).map_into_right_body());
            }

            req.extensions_mut().insert(claims.id);
            return Ok(next.call(req).await?.map_into_left_body());
        }
    }

    log::error!("Unauthorized");
    let response = HttpResponse::Unauthorized().finish();
    Ok(req.into_response(response).map_into_right_body())
}
