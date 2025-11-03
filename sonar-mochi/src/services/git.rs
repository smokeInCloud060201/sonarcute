use anyhow::{Result, Context};
use std::path::Path;
use git2::{Repository, DiffOptions};

pub struct GitService;

impl GitService {
    pub fn new() -> Self {
        GitService
    }

    /// Get diff between two commits
    pub fn get_diff(
        &self,
        repo_path: &str,
        base_commit: &str,
        head_commit: &str,
    ) -> Result<String> {
        let repo = Repository::open(repo_path)
            .context("Failed to open repository")?;

        let base_oid = repo.revparse_single(base_commit)?
            .id();
        let head_oid = repo.revparse_single(head_commit)?
            .id();

        let base_tree = repo.find_commit(base_oid)?
            .tree()?;
        let head_tree = repo.find_commit(head_oid)?
            .tree()?;

        let diff = repo.diff_tree_to_tree(
            Some(&base_tree),
            Some(&head_tree),
            Some(&mut DiffOptions::new()),
        )?;

        let mut output = Vec::new();
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            match line.origin() {
                '+' | '-' | ' ' => {
                    output.push(line.origin() as u8);
                    output.extend_from_slice(line.content());
                }
                _ => {}
            }
            true
        })?;

        Ok(String::from_utf8(output)?)
    }

    /// Get list of changed files
    pub fn get_changed_files(
        &self,
        repo_path: &str,
        base_commit: &str,
        head_commit: &str,
    ) -> Result<Vec<String>> {
        let repo = Repository::open(repo_path)
            .context("Failed to open repository")?;

        let base_oid = repo.revparse_single(base_commit)?
            .id();
        let head_oid = repo.revparse_single(head_commit)?
            .id();

        let base_tree = repo.find_commit(base_oid)?
            .tree()?;
        let head_tree = repo.find_commit(head_oid)?
            .tree()?;

        let diff = repo.diff_tree_to_tree(
            Some(&base_tree),
            Some(&head_tree),
            Some(&mut DiffOptions::new()),
        )?;

        let mut files = Vec::new();
        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    files.push(path.to_string_lossy().to_string());
                }
                true
            },
            None,
            None,
            None,
        )?;

        Ok(files)
    }

    /// Get file content at specific commit
    pub fn get_file_content(
        &self,
        repo_path: &str,
        file_path: &str,
        commit: Option<&str>,
    ) -> Result<String> {
        let repo = Repository::open(repo_path)
            .context("Failed to open repository")?;

        let tree = if let Some(commit_ref) = commit {
            let oid = repo.revparse_single(commit_ref)?
                .id();
            repo.find_commit(oid)?
                .tree()?
        } else {
            repo.head()?.peel_to_tree()?
        };

        let entry = tree.get_path(Path::new(file_path))?;
        let blob = repo.find_blob(entry.id())?;
        Ok(String::from_utf8(blob.content().to_vec())?)
    }

    /// Check if path is a valid git repository
    pub fn is_repository(&self, path: &str) -> bool {
        Repository::open(path).is_ok()
    }
}

