use rocket::figment::Figment;
use std::env;

pub fn from_env() -> Figment {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let address = env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());

    println!("Starting server from_env info");

    Figment::from(rocket::Config::default())
        .merge(("address", address))
        .merge(("port", port))
}

pub const DEFAULT_OLLAMA_MODEL: &str = "llama3.1:latest";
