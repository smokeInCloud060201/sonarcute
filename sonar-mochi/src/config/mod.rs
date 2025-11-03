use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub ai: AIConfig,
    pub integrations: IntegrationsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AIConfig {
    pub default_provider: String,
    pub ollama: OllamaConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OllamaConfig {
    pub base_url: String,
    pub default_model: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IntegrationsConfig {
    pub github: Option<GitHubConfig>,
    pub jira: Option<JiraConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubConfig {
    pub token: Option<String>,
    pub api_base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JiraConfig {
    pub url: Option<String>,
    pub email: Option<String>,
    pub api_token: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(AppConfig {
            server: ServerConfig {
                host: env::var("SERVER_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .map_err(|_| ConfigError::InvalidPort)?,
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .map_err(|_| ConfigError::MissingDatabaseUrl)?,
            },
            ai: AIConfig {
                default_provider: env::var("AI_DEFAULT_PROVIDER")
                    .unwrap_or_else(|_| "ollama".to_string()),
                ollama: OllamaConfig {
                    base_url: env::var("OLLAMA_BASE_URL")
                        .unwrap_or_else(|_| "http://localhost:11434".to_string()),
                    default_model: env::var("OLLAMA_DEFAULT_MODEL")
                        .unwrap_or_else(|_| "llama2".to_string()),
                },
            },
            integrations: IntegrationsConfig {
                github: Some(GitHubConfig {
                    token: env::var("GITHUB_TOKEN").ok(),
                    api_base_url: env::var("GITHUB_API_BASE_URL")
                        .unwrap_or_else(|_| "https://api.github.com".to_string()),
                }),
                jira: Some(JiraConfig {
                    url: env::var("JIRA_URL").ok(),
                    email: env::var("JIRA_EMAIL").ok(),
                    api_token: env::var("JIRA_API_TOKEN").ok(),
                }),
            },
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing DATABASE_URL environment variable")]
    MissingDatabaseUrl,
    #[error("Invalid port number")]
    InvalidPort,
}

