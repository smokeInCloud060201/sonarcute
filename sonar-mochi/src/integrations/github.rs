use anyhow::{Result, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct GitHubClient {
    client: Client,
    token: String,
    base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct PR {
    pub number: u64,
    pub title: String,
    pub body: String,
    pub state: String,
    pub labels: Vec<Label>,
    pub head: PRRef,
    pub base: PRRef,
}

#[derive(Debug, Deserialize)]
pub struct Label {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PRRef {
    pub r#ref: String,
    pub sha: String,
}

#[derive(Debug, Deserialize)]
pub struct PRFile {
    pub filename: String,
    pub status: String,
    pub patch: Option<String>,
}

impl GitHubClient {
    pub fn new(token: String, base_url: Option<String>) -> Self {
        GitHubClient {
            client: Client::new(),
            token,
            base_url: base_url.unwrap_or_else(|| "https://api.github.com".to_string()),
        }
    }

    pub async fn get_pr(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u64,
    ) -> Result<PR> {
        let url = format!(
            "{}/repos/{}/{}/pulls/{}",
            self.base_url, owner, repo, pr_number
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("token {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .context("Failed to fetch PR from GitHub")?;

        let pr: PR = response
            .json()
            .await
            .context("Failed to parse PR response")?;

        Ok(pr)
    }

    pub async fn get_pr_files(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u64,
    ) -> Result<Vec<PRFile>> {
        let url = format!(
            "{}/repos/{}/{}/pulls/{}/files",
            self.base_url, owner, repo, pr_number
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("token {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .context("Failed to fetch PR files from GitHub")?;

        let files: Vec<PRFile> = response
            .json()
            .await
            .context("Failed to parse PR files response")?;

        Ok(files)
    }

    pub async fn create_pr_comment(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u64,
        body: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/comments",
            self.base_url, owner, repo, pr_number
        );

        #[derive(Serialize)]
        struct CommentRequest {
            body: String,
        }

        self.client
            .post(&url)
            .header("Authorization", format!("token {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .json(&CommentRequest {
                body: body.to_string(),
            })
            .send()
            .await
            .context("Failed to create PR comment")?;

        Ok(())
    }
}

