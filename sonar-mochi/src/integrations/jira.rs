use anyhow::{Result, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use base64::Engine;
use std::collections::HashMap;

pub struct JiraClient {
    client: Client,
    base_url: String,
    email: String,
    api_token: String,
}

#[derive(Debug, Deserialize)]
pub struct JiraIssue {
    pub key: String,
    pub fields: JiraFields,
}

#[derive(Debug, Deserialize)]
pub struct JiraFields {
    pub summary: String,
    pub description: Option<String>,
    pub project: JiraProject,
    #[serde(rename = "customfield_10020")] // Acceptance Criteria (may vary)
    pub acceptance_criteria: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JiraProject {
    pub key: String,
    pub name: String,
}

impl JiraClient {
    pub fn new(base_url: String, email: String, api_token: String) -> Self {
        JiraClient {
            client: Client::new(),
            base_url,
            email,
            api_token,
        }
    }

    fn get_auth_header(&self) -> String {
        let credentials = format!("{}:{}", self.email, self.api_token);
        let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);
        format!("Basic {}", encoded)
    }

    pub async fn get_issue(&self, issue_key: &str) -> Result<JiraIssue> {
        let url = format!("{}/rest/api/3/issue/{}", self.base_url, issue_key);

        let response = self
            .client
            .get(&url)
            .header("Authorization", self.get_auth_header())
            .header("Accept", "application/json")
            .send()
            .await
            .context("Failed to fetch issue from Jira")?;

        let issue: JiraIssue = response
            .json()
            .await
            .context("Failed to parse Jira issue response")?;

        Ok(issue)
    }

    pub async fn extract_acceptance_criteria(
        &self,
        issue_key: &str,
    ) -> Result<Vec<String>> {
        let issue = self.get_issue(issue_key).await?;
        
        // TODO: Parse acceptance criteria from issue fields
        // This depends on Jira project configuration
        Ok(vec![])
    }
}

