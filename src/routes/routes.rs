use rocket::{serde::json::Json, Build, Rocket};

use crate::routes::request::UnAuthRequest;

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
) -> ApiResponse<'static, Json<UnAuthResponse<'a>>> {
    todo!("request to olama")
}
