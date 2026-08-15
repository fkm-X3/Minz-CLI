use async_openai::{config::OpenAIConfig, Client};
use std::{env, process};

pub struct Config {
    pub client: Client<OpenAIConfig>,
}

impl Config {
    /// Loads environment variables and initializes the API client.
    pub fn new() -> Self {
        let base_url = env::var("OPENROUTER_BASE_URL")
            .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

        let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
            eprintln!("Error: OPENROUTER_API_KEY environment variable is not set.");
            process::exit(1);
        });

        let config = OpenAIConfig::new()
            .with_api_base(base_url)
            .with_api_key(api_key);

        let client = Client::with_config(config);

        Self { client }
    }
}