pub mod provider;
pub mod ollama;
pub mod prompt;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub comments: Vec<AIComment>,
    pub summary: AISummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIComment {
    pub file: String,
    pub line: Option<i32>,
    pub r#type: String,
    pub severity: String,
    pub message: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISummary {
    pub overall_assessment: String,
    pub key_concerns: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AIConfig {
    pub provider: String,
    pub model: String,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

