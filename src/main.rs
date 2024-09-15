use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();

    // For custom values:
    // let ollama = Ollama::new("http://localhost".to_string(), 11434);
    let model = "llama3.1:latest".to_string();
    let prompt = "test initial prompt".to_string();
    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;
    println!("result :{:?}", res.unwrap().response);
}
