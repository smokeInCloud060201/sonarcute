use crate::services::context::ReviewContext;
use serde_json::json;

pub struct PromptBuilder {
    context: Option<String>,
    code_changes: Vec<CodeChange>,
    guidelines: Vec<String>,
    language: String,
    framework: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CodeChange {
    pub file_path: String,
    pub diff: String,
    pub language: String,
}

impl PromptBuilder {
    pub fn new(language: String) -> Self {
        PromptBuilder {
            context: None,
            code_changes: vec![],
            guidelines: vec![],
            framework: None,
            language,
        }
    }

    pub fn with_context(mut self, context: ReviewContext) -> Self {
        self.context = Some(self.format_context(&context));
        if let Some(ref github) = context.github_context {
            // Add GitHub context to prompt
        }
        if let Some(ref jira) = context.jira_context {
            // Add Jira context to prompt
        }
        self
    }

    pub fn with_code_changes(mut self, changes: Vec<CodeChange>) -> Self {
        self.code_changes = changes;
        self
    }

    pub fn with_guidelines(mut self, guidelines: Vec<String>) -> Self {
        self.guidelines = guidelines;
        self
    }

    pub fn with_framework(mut self, framework: String) -> Self {
        self.framework = Some(framework);
        self
    }

    pub fn build(self) -> String {
        let mut prompt = String::new();

        // Context section
        prompt.push_str(&format!(
            "You are an expert code reviewer specializing in {}",
            self.language
        ));

        if let Some(ref framework) = self.framework {
            prompt.push_str(&format!(" and {}", framework));
        }

        prompt.push_str(".\n\n");

        // Project context
        if let Some(ref context) = self.context {
            prompt.push_str("PROJECT CONTEXT:\n");
            prompt.push_str(context);
            prompt.push_str("\n\n");
        }

        // Code changes section
        prompt.push_str("CODE CHANGES:\n");
        for change in &self.code_changes {
            prompt.push_str(&format!("File: {}\n", change.file_path));
            prompt.push_str(&format!("```{}\n", change.language));
            prompt.push_str(&change.diff);
            prompt.push_str("\n```\n\n");
        }

        // Guidelines section
        if !self.guidelines.is_empty() {
            prompt.push_str("REVIEW GUIDELINES:\n");
            for guideline in &self.guidelines {
                prompt.push_str(&format!("- {}\n", guideline));
            }
            prompt.push_str("\n");
        }

        // Output format
        prompt.push_str(
            r#"
Provide structured feedback in JSON format:
{
  "comments": [
    {
      "file": "path/to/file",
      "line": 42,
      "type": "suggestion",
      "severity": "medium",
      "message": "...",
      "suggestion": "..."
    }
  ],
  "summary": {
    "overall_assessment": "...",
    "key_concerns": [...],
    "recommendations": [...]
  }
}
"#,
        );

        prompt
    }

    fn format_context(&self, context: &ReviewContext) -> String {
        let mut formatted = String::new();

        formatted.push_str(&format!(
            "- Language: {}\n",
            context.project_info.language
        ));

        if let Some(ref framework) = context.project_info.framework {
            formatted.push_str(&format!("- Framework: {}\n", framework));
        }

        if !context.design_patterns.is_empty() {
            formatted.push_str(&format!(
                "- Design Patterns Used: {}\n",
                context.design_patterns.join(", ")
            ));
        }

        if !context.related_files.is_empty() {
            formatted.push_str(&format!(
                "- Related Files: {}\n",
                context.related_files.join(", ")
            ));
        }

        formatted
    }
}

