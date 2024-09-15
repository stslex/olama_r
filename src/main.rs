use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use rocket::Rocket;
use routes::Routes;

mod config;
mod routes;

#[macro_use]
extern crate rocket;

#[rocket::launch]
async fn launch() -> Rocket<rocket::Build> {
    dotenv::dotenv().ok();

    let ollama = Ollama::default();

    // For custom values:
    // let ollama = Ollama::new("http://localhost".to_string(), 11434);
    let model = "llama3.1:latest".to_string();
    let prompt = "test initial prompt".to_string();
    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;
    println!("result :{:?}", res.unwrap().response);

    rocket::custom(config::from_env()).mount_routes()
}
