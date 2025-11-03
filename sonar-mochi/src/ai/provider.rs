use anyhow::Result;
use async_trait::async_trait;
use crate::ai::{AIResponse, AIConfig};

#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Generate a code review
    async fn generate_review(
        &self,
        prompt: &str,
        config: &AIConfig,
    ) -> Result<AIResponse>;

    /// Validate provider configuration
    fn validate_config(&self, config: &AIConfig) -> Result<()>;

    /// Get provider name
    fn name(&self) -> &str;
}

pub struct AIProviderFactory;

impl AIProviderFactory {
    pub fn create(provider_type: &str) -> Result<Box<dyn AIProvider>> {
        match provider_type {
            "ollama" => Ok(Box::new(crate::ai::ollama::OllamaProvider::new())),
            // TODO: Add other providers
            _ => anyhow::bail!("Unknown AI provider: {}", provider_type),
        }
    }
}

