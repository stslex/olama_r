use rocket::response::{self, Responder};
use rocket::{http::ContentType, http::Status, serde::json::Json};
use rocket::{Request, Response};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize, Debug, Clone)]
pub struct UnAuthResponse<'a> {
    #[serde(rename = "msg")]
    pub msg: &'a str,
}

pub enum ApiResponse<'a, T> {
    Ok(T),
    Err(&'a ErrorResponse<'a>),
}

impl<'r, 'o: 'r, T> Responder<'r, 'o> for ApiResponse<'r, T>
where
    T: Responder<'r, 'o>,
{
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
        match self {
            ApiResponse::Ok(t) => t.respond_to(request),
            ApiResponse::Err(e) => e.respond_to(request),
        }
    }
}

impl<'a, T> Into<ApiResponse<'a, T>> for Result<T, &'a ErrorResponse<'a>> {
    fn into(self) -> ApiResponse<'a, T> {
        match self {
            Ok(t) => ApiResponse::Ok(t),
            Err(e) => ApiResponse::Err(e),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ErrorResponse<'a> {
    pub status: Status,
    pub message: &'a str,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ErrorResponse<'r> {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
        if let Ok(response) = Json(json!({"error": self.message})).respond_to(request) {
            Response::build_from(response)
                .status(self.status)
                .header(ContentType::JSON)
                .ok()
        } else {
            Response::build()
                .status(Status::InternalServerError)
                .header(ContentType::JSON)
                .ok()
        }
    }
}
