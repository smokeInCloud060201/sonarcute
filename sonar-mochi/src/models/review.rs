use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: Option<i64>,
    pub review_type: ReviewType,
    pub status: ReviewStatus,
    pub repo_path: Option<String>,
    pub github_owner: Option<String>,
    pub github_repo: Option<String>,
    pub pr_number: Option<u64>,
    pub base_commit: Option<String>,
    pub head_commit: Option<String>,
    pub ai_model: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewType {
    Local,
    Pr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

// Request types for handlers
pub struct LocalReviewRequest {
    pub repo_path: String,
    pub base_commit: Option<String>,
    pub head_commit: Option<String>,
}

pub struct PRReviewRequest {
    pub owner: String,
    pub repo: String,
    pub pr_number: u64,
}

