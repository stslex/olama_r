use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use rocket::{serde::json::Json, Build, Rocket};

use crate::{
    config::DEFAULT_OLLAMA_MODEL,
    routes::{
        request::UnAuthRequest,
        response::{ErrorResponse, BAD_REQUEST, PROMPT_IS_EMPTY},
    },
};

use super::{
    response::{ApiResponse, UnAuthResponse},
    Routes,
};

impl Routes for Rocket<Build> {
    fn mount_routes(self) -> Self {
        self.mount("/", routes![request_un_auth])
    }
}

#[get("/unauth", format = "json", data = "<request>")]
async fn request_un_auth<'a>(
    request: Option<Json<UnAuthRequest<'a>>>,
) -> ApiResponse<'static, Json<UnAuthResponse>> {
    match request {
        Some(request) => {
            let ollama = Ollama::default();
            let model = request.model.unwrap_or(DEFAULT_OLLAMA_MODEL);
            let prompt = request.prompt;
            if prompt.is_empty() {
                return ApiResponse::Err(PROMPT_IS_EMPTY);
            };

            match ollama
                .generate(GenerationRequest::new(model.to_owned(), prompt.to_owned()))
                .await
            {
                Result::Ok(res) => ApiResponse::Ok(Json(UnAuthResponse {
                    response: res.response,
                })),
                Result::Err(_) => ApiResponse::Err(BAD_REQUEST),
            }
        }
        None => ApiResponse::Err(BAD_REQUEST),
    }
}
