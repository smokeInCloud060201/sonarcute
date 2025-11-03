use anyhow::{Result, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::ai::provider::AIProvider;
use crate::ai::{AIResponse, AIConfig};
use crate::config::AppConfig;

pub struct OllamaProvider {
    client: Client,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: Option<f32>,
    num_predict: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

impl OllamaProvider {
    pub fn new() -> Self {
        OllamaProvider {
            client: Client::new(),
            base_url: std::env::var("OLLAMA_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
        }
    }

    async fn call_api(&self, prompt: &str, model: &str) -> Result<String> {
        let url = format!("{}/api/generate", self.base_url);
        
        let request = OllamaRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            options: None,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Ollama")?;

        let ollama_response: OllamaResponse = response
            .json()
            .await
            .context("Failed to parse Ollama response")?;

        Ok(ollama_response.response)
    }
}

#[async_trait::async_trait]
impl AIProvider for OllamaProvider {
    async fn generate_review(
        &self,
        prompt: &str,
        config: &AIConfig,
    ) -> Result<AIResponse> {
        tracing::debug!("Generating review with Ollama model: {}", config.model);
        
        let response_text = self.call_api(prompt, &config.model).await?;
        
        // Parse JSON response from AI
        let ai_response: AIResponse = serde_json::from_str(&response_text)
            .context("Failed to parse AI response as JSON")?;
        
        Ok(ai_response)
    }

    fn validate_config(&self, config: &AIConfig) -> Result<()> {
        if config.model.is_empty() {
            anyhow::bail!("Ollama model name is required");
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "ollama"
    }
}

