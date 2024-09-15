use rocket::serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct UnAuthRequest<'a> {
    #[serde(rename = "msg")]
    pub msg: &'a str,
}
