use rocket::serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct UnAuthRequest<'a> {
    #[serde(rename = "prompt")]
    pub prompt: &'a str,
    #[serde(rename = "model")]
    pub model: Option<&'a str>,
}
