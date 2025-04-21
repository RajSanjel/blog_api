use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
}

pub enum ServerResponse<T>
where
    T: Serialize,
{
    BadRequest(String, Option<T>),
    SuccessMessage(String, Option<T>),
    ServerError(String),
}

impl<T> IntoResponse for ServerResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest(message, data) => {
                let res = Json(ApiResponse {
                    success: false,
                    message,
                    data,
                });
                (StatusCode::BAD_REQUEST, res).into_response()
            }
            Self::SuccessMessage(message, data) => {
                let res = Json(ApiResponse {
                    success: true,
                    message,
                    data,
                });
                (StatusCode::OK, res).into_response()
            }
            Self::ServerError(message) => {
                let res = Json(ApiResponse::<()> {
                    success: false,
                    message,
                    data: None,
                });
                (StatusCode::INTERNAL_SERVER_ERROR, res).into_response()
            }
        }
    }
}
