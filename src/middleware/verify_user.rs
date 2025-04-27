use axum::{
    extract::{FromRequestParts, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{
    response::server_response::ServerResponse,
    utils::{cookies::read_cookies, jwt::jwt_decode},
};
pub async fn verify_user(req: Request, next: Next) -> Response {
    // Extract parts from the request
    let (mut parts, body) = req.into_parts();

    // Get cookies from parts
    let cookies = tower_cookies::Cookies::from_request_parts(&mut parts, &())
        .await
        .unwrap_or_default();

    match read_cookies(cookies, "auth-token").await {
        Ok(val) => {
            match jwt_decode(val) {
                Ok(decoded) => {
                    let user_id = decoded.user_id;
                    parts.extensions.insert(user_id);

                    // Reconstruct the request with the added extensions
                    let req = Request::from_parts(parts, body);

                    // Continue the middleware chain
                    next.run(req).await
                }
                Err(err) => {
                    ServerResponse::BadRequest(format!("Invalid token: {}", err), None::<()>)
                        .into_response()
                }
            }
        }
        Err(_) => {
            ServerResponse::BadRequest("Unauthorized".to_string(), None::<()>).into_response()
        }
    }
}
