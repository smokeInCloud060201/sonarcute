use std::path::Path;

/// Validate repository path to prevent path traversal attacks
pub fn validate_repo_path(path: &str) -> Result<(), ValidationError> {
    let path = Path::new(path);
    
    // Check for path traversal attempts
    if path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
        return Err(ValidationError::InvalidPath);
    }

    // Check if path is absolute (could be a requirement, adjust as needed)
    if !path.is_absolute() {
        return Err(ValidationError::PathMustBeAbsolute);
    }

    Ok(())
}

/// Validate GitHub owner/repo format
pub fn validate_github_repo(owner: &str, repo: &str) -> Result<(), ValidationError> {
    // Basic validation - owner and repo should be alphanumeric with hyphens and underscores
    if !owner.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(ValidationError::InvalidGitHubOwner);
    }

    if !repo.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.') {
        return Err(ValidationError::InvalidGitHubRepo);
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid repository path")]
    InvalidPath,
    #[error("Path must be absolute")]
    PathMustBeAbsolute,
    #[error("Invalid GitHub owner format")]
    InvalidGitHubOwner,
    #[error("Invalid GitHub repo format")]
    InvalidGitHubRepo,
}

