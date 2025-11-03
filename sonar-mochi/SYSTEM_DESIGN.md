# Sonar-Mochi - AI-Powered PR Review System

Comprehensive system architecture and design documentation for Sonar-Mochi, an AI-powered code review system for Git Pull Requests.

## Table of Contents

- [Overview](#overview)
- [Architecture Overview](#architecture-overview)
- [System Components](#system-components)
- [Core Features](#core-features)
- [Data Flow](#data-flow)
- [Database Design](#database-design)
- [API Design](#api-design)
- [AI Integration](#ai-integration)
- [Integration Architecture](#integration-architecture)
- [Design Patterns](#design-patterns)
- [Security Architecture](#security-architecture)
- [Error Handling](#error-handling)
- [Performance Considerations](#performance-considerations)
- [Scalability](#scalability)
- [Deployment Architecture](#deployment-architecture)

## Overview

### Project Purpose

Sonar-Mochi is an AI-powered code review system designed to help developers and reviewers assess code changes in Git Pull Requests. It supports both local pre-push reviews and remote PR reviews, leveraging AI models to provide intelligent code analysis and suggestions.

### Key Capabilities

1. **Dual Review Modes**:
   - **Developer Mode**: Review local changes before pushing to remote
   - **Reviewer Mode**: Review PRs on remote origin server

2. **AI-Powered Analysis**:
   - Multiple AI model support (Ollama default, extensible)
   - Context-aware code review
   - Design pattern detection and suggestions
   - Code quality assessment

3. **Context Integration**:
   - GitHub integration for PR metadata
   - Jira integration for issue tracking and project context
   - Repository analysis for historical context

4. **Multi-Language Support**:
   - Java (Spring Boot)
   - JavaScript/TypeScript (React)
   - Rust (Actix-web)

## Architecture Overview

### High-Level Architecture

```
┌──────────────────────────────────────────────────────────────────────┐
│                        Client Interface                               │
│  ┌──────────────────────┐  ┌──────────────────────┐                │
│  │  Developer Mode      │  │  Reviewer Mode        │                │
│  │  (Local Git Repo)    │  │  (Remote PR)          │                │
│  └──────────────────────┘  └──────────────────────┘                │
└──────────────────────┬──────────────────────────┬────────────────────┘
                       │                          │
                       │ CLI / API                │ API
                       │                          │
┌──────────────────────▼──────────────────────────▼────────────────────┐
│                    Sonar-Mochi API Server                            │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │              HTTP Layer (Actix-web)                            │ │
│  │  ┌─────────────┐  ┌──────────────┐  ┌─────────────┐          │ │
│  │  │   Routes    │  │  Middleware  │  │   Handlers  │          │ │
│  │  └─────────────┘  └──────────────┘  └─────────────┘          │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                          │                                           │
│  ┌───────────────────────┼─────────────────────────────────────────┐ │
│  │                       │                                         │ │
│  │  ┌────────────────────▼────────┐  ┌──────────────────────────┐ │ │
│  │  │   Service Layer             │  │   AI Service Layer        │ │ │
│  │  │  - Review Service           │  │  - Model Manager          │ │ │
│  │  │  - Git Service              │  │  - Prompt Builder        │ │ │
│  │  │  - Context Service          │  │  - Response Parser        │ │ │
│  │  └─────────────────────────────┘  └──────────────────────────┘ │ │
│  │                                                                  │ │
│  │  ┌────────────────────────────────────────────────────────────┐ │ │
│  │  │   Integration Layer                                        │ │ │
│  │  │  - GitHub Client                                           │ │ │
│  │  │  - Jira Client                                             │ │ │
│  │  │  - Git Operations                                          │ │ │
│  │  └────────────────────────────────────────────────────────────┘ │ │
│  └──────────────────────────────────────────────────────────────────┘ │
└──────────────┬────────────────┬────────────────┬─────────────────────┘
               │                │                │
      ┌────────▼────────┐ ┌─────▼─────┐ ┌───────▼────────┐
      │   PostgreSQL    │ │ Ollama /  │ │  External APIs  │
      │   Database      │ │ AI Models │ │  GitHub / Jira │
      └─────────────────┘ └───────────┘ └────────────────┘
```

## System Components

### 1. HTTP Server Layer

**Technology**: Actix-web 4.x

**Responsibilities**:
- HTTP request/response handling
- Route registration and routing
- Middleware (CORS, authentication, logging, rate limiting)
- Request validation and deserialization
- Response serialization

**Key Modules**:
- `src/web/server.rs`: Server initialization and route configuration
- `src/web/middleware.rs`: Custom middleware implementations
- `src/web/handlers/`: Request handlers for different resources

### 2. Service Layer

**Responsibilities**:
- Business logic orchestration
- Transaction management
- Data transformation
- Error handling and validation

**Key Services**:

#### Review Service
- Orchestrates the review process
- Manages review lifecycle
- Coordinates between Git, AI, and Context services

#### Git Service
- Local repository operations
- Diff extraction
- Commit analysis
- Branch management

#### Context Service
- Aggregates context from multiple sources
- Caches context data
- Manages context freshness

### 3. AI Service Layer

**Responsibilities**:
- AI model management and abstraction
- Prompt construction and optimization
- Response parsing and validation
- Model switching and fallback

**Key Components**:

#### Model Manager
- Manages multiple AI provider backends
- Handles model selection and switching
- Implements provider abstraction

#### Prompt Builder
- Constructs context-aware prompts
- Formats code changes for AI
- Includes design pattern context

#### Response Parser
- Parses AI responses
- Extracts structured feedback
- Validates response quality

### 4. Integration Layer

**Responsibilities**:
- External API communication
- Authentication and authorization
- Rate limiting and retry logic
- Data caching

**Key Integrations**:

#### GitHub Client
- PR metadata retrieval
- File content fetching
- Commit history analysis
- Review comment creation

#### Jira Client
- Issue details retrieval
- Project context extraction
- Issue linking and updates

#### Git Operations
- Local repository access
- Diff generation
- File system operations

### 5. Database Layer

**Technology**: SeaORM with PostgreSQL

**Responsibilities**:
- Data persistence
- Entity management
- Query building and optimization
- Migration management

**Key Entities**:
- Reviews
- Review Comments
- AI Model Configurations
- Integration Credentials
- Review Templates

### 6. Configuration Layer

**Responsibilities**:
- Environment variable management
- Logger initialization
- Application configuration
- Feature flags

## Core Features

### 1. Developer Mode - Local Review

**Workflow**:
```
1. Developer initiates local review
   └─> Git Service extracts uncommitted changes or specific commit range
       │
2. Context Service gathers project context
   ├─> Repository structure analysis
   ├─> Related files identification
   └─> Historical context from Git
       │
3. AI Service processes changes
   ├─> Code diff analysis
   ├─> Design pattern detection
   ├─> Best practices validation
   └─> Security and performance checks
       │
4. Review results presented to developer
   └─> Actionable feedback and suggestions
```

**Use Cases**:
- Pre-commit code review
- Pre-push validation
- Feature branch review
- Code quality check before creating PR

### 2. Reviewer Mode - Remote PR Review

**Workflow**:
```
1. Reviewer requests PR review
   └─> GitHub Client fetches PR metadata
       │
2. Context Service aggregates comprehensive context
   ├─> PR description and comments
   ├─> Jira issues linked to PR
   ├─> Related PRs and discussions
   ├─> Code review history
   └─> Project documentation
       │
3. AI Service performs deep analysis
   ├─> PR-level context understanding
   ├─> Change impact analysis
   ├─> Design pattern compliance
   ├─> Consistency with codebase
   └─> Best practices review
       │
4. Review results formatted for PR
   └─> Comments ready for GitHub PR
```

**Use Cases**:
- Automated PR review
- Consistency checking
- Design pattern validation
- Comprehensive code quality assessment

### 3. AI Model Management

**Supported Providers**:
- Ollama (default, local)
- OpenAI API
- Anthropic Claude API
- Azure OpenAI
- Custom providers (extensible)

**Model Selection**:
- Default: Ollama
- User-configurable per review
- Automatic fallback on failures
- Model-specific prompt optimization

### 4. Context Gathering

**Sources**:
1. **GitHub**:
   - PR metadata (title, description, labels)
   - Comments and discussions
   - Review history
   - Related PRs
   - File history and blame

2. **Jira**:
   - Issue details and requirements
   - Acceptance criteria
   - Related issues
   - Project context and standards
   - Team knowledge base

3. **Repository**:
   - Project structure and organization
   - Related files and dependencies
   - Design patterns in use
   - Coding standards and conventions
   - Documentation

4. **Historical Context**:
   - Similar changes in the past
   - Common patterns and solutions
   - Previous review feedback

## Data Flow

### Local Review Flow

```
1. Client Request
   └─> POST /api/reviews/local
       Body: { repo_path, commit_range, options }
       │
2. Request Handler (create_local_review)
   └─> Validate request
       └─> Extract parameters
           │
3. Git Service
   └─> Access local repository
       └─> Extract diff for commit range
           └─> Parse changed files
               │
4. Context Service
   ├─> Analyze repository structure
   ├─> Identify related files
   ├─> Extract design patterns
   └─> Gather historical context
       │
5. Prompt Builder
   └─> Construct AI prompt
       ├─> Include code changes
       ├─> Add project context
       ├─> Include design patterns
       └─> Add review guidelines
           │
6. AI Service
   └─> Send prompt to selected model
       └─> Parse AI response
           └─> Extract structured feedback
               │
7. Review Service
   └─> Save review to database
       └─> Format response
           │
8. HTTP Response
   └─> Return ReviewResponse with feedback
```

### Remote PR Review Flow

```
1. Client Request
   └─> POST /api/reviews/pr
       Body: { owner, repo, pr_number, options }
       │
2. Request Handler (create_pr_review)
   └─> Validate request
       └─> Extract parameters
           │
3. GitHub Client
   ├─> Fetch PR metadata
   ├─> Fetch PR files and diffs
   ├─> Fetch PR comments
   └─> Fetch related PRs
       │
4. Jira Client (if PR linked to issues)
   ├─> Fetch issue details
   ├─> Fetch acceptance criteria
   └─> Fetch project standards
       │
5. Context Service
   └─> Aggregate all context
       ├─> PR context from GitHub
       ├─> Requirements from Jira
       ├─> Repository structure
       └─> Historical patterns
           │
6. Prompt Builder
   └─> Construct comprehensive prompt
       ├─> PR-level context
       ├─> All code changes
       ├─> Requirements and criteria
       └─> Review standards
           │
7. AI Service
   └─> Send prompt to AI model
       └─> Parse comprehensive response
           │
8. Review Service
   ├─> Save review to database
   ├─> Format GitHub comments
   └─> Optionally post to GitHub PR
       │
9. HTTP Response
   └─> Return ReviewResponse with results
```

## Database Design

### Entity Relationship Diagram

```
┌─────────────────────┐
│      reviews        │
├─────────────────────┤
│ id (PK)             │
│ review_type         │  (LOCAL | PR)
│ status              │  (PENDING | IN_PROGRESS | COMPLETED | FAILED)
│ repo_path           │  (for local reviews)
│ github_owner        │  (for PR reviews)
│ github_repo          │
│ pr_number           │
│ base_commit         │
│ head_commit         │
│ ai_model            │
│ created_at          │
│ completed_at        │
│ user_id             │
└─────────────────────┘
         │
         │ 1:N
         │
┌────────▼────────────────┐
│    review_comments      │
├─────────────────────────┤
│ id (PK)                 │
│ review_id (FK)          │
│ file_path               │
│ line_number             │
│ comment_type            │  (SUGGESTION | QUESTION | BUG | PRAISE)
│ severity                │  (LOW | MEDIUM | HIGH | CRITICAL)
│ message                 │
│ suggested_code          │
│ metadata                │  (JSON)
│ created_at              │
└─────────────────────────┘

┌─────────────────────┐
│  ai_model_configs    │
├─────────────────────┤
│ id (PK)              │
│ name                 │
│ provider             │  (OLLAMA | OPENAI | ANTHROPIC | ...)
│ model_name           │
│ api_endpoint         │
│ api_key_encrypted    │
│ is_default           │
│ is_active            │
│ config               │  (JSON)
│ created_at           │
│ updated_at           │
└─────────────────────┘

┌─────────────────────┐
│ integration_configs │
├─────────────────────┤
│ id (PK)             │
│ type                │  (GITHUB | JIRA)
│ name                │
│ api_endpoint        │
│ credentials_encrypted│
│ user_id             │
│ is_active           │
│ config              │  (JSON)
│ created_at          │
│ updated_at          │
└─────────────────────┘

┌─────────────────────┐
│  review_templates   │
├─────────────────────┤
│ id (PK)             │
│ name                │
│ description         │
│ language            │  (JAVA | JAVASCRIPT | TYPESCRIPT | RUST)
│ framework           │  (SPRING_BOOT | REACT | ACTIX_WEB)
│ prompt_template     │
│ guidelines          │  (JSON)
│ is_default          │
│ created_at          │
│ updated_at          │
└─────────────────────┘

┌─────────────────────┐
│    review_history   │
├─────────────────────┤
│ id (PK)             │
│ review_id (FK)      │
│ action              │  (CREATED | STARTED | COMPLETED | FAILED)
│ details             │  (JSON)
│ timestamp           │
└─────────────────────┘
```

### Table Details

#### reviews Table

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | SERIAL | PRIMARY KEY | Auto-increment ID |
| review_type | VARCHAR(20) | NOT NULL | LOCAL or PR |
| status | VARCHAR(20) | NOT NULL | Review status |
| repo_path | VARCHAR(500) | NULL | Local repository path (for LOCAL reviews) |
| github_owner | VARCHAR(255) | NULL | GitHub owner (for PR reviews) |
| github_repo | VARCHAR(255) | NULL | GitHub repository |
| pr_number | INTEGER | NULL | PR number (for PR reviews) |
| base_commit | VARCHAR(40) | NULL | Base commit SHA |
| head_commit | VARCHAR(40) | NULL | Head commit SHA |
| ai_model | VARCHAR(100) | NOT NULL | AI model used |
| created_at | TIMESTAMP | NOT NULL | Creation timestamp |
| completed_at | TIMESTAMP | NULL | Completion timestamp |
| user_id | INTEGER | NULL | User identifier |

**Indexes**:
- `idx_reviews_status` on `status`
- `idx_reviews_type` on `review_type`
- `idx_reviews_github` on `github_owner, github_repo, pr_number` (where not null)

#### review_comments Table

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | SERIAL | PRIMARY KEY | Auto-increment ID |
| review_id | INTEGER | FOREIGN KEY | Reference to reviews.id |
| file_path | VARCHAR(500) | NOT NULL | File path relative to repo root |
| line_number | INTEGER | NULL | Line number (null for file-level comments) |
| comment_type | VARCHAR(20) | NOT NULL | Type of comment |
| severity | VARCHAR(20) | NOT NULL | Severity level |
| message | TEXT | NOT NULL | Comment message |
| suggested_code | TEXT | NULL | Suggested code replacement |
| metadata | JSONB | NULL | Additional metadata |
| created_at | TIMESTAMP | NOT NULL | Creation timestamp |

**Indexes**:
- `idx_review_comments_review_id` on `review_id`
- `idx_review_comments_file` on `file_path`
- `idx_review_comments_severity` on `severity`

#### ai_model_configs Table

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | SERIAL | PRIMARY KEY | Auto-increment ID |
| name | VARCHAR(255) | UNIQUE, NOT NULL | Configuration name |
| provider | VARCHAR(50) | NOT NULL | AI provider type |
| model_name | VARCHAR(255) | NOT NULL | Model identifier |
| api_endpoint | VARCHAR(500) | NULL | API endpoint URL |
| api_key_encrypted | TEXT | NULL | Encrypted API key |
| is_default | BOOLEAN | DEFAULT false | Is default model |
| is_active | BOOLEAN | DEFAULT true | Is active |
| config | JSONB | NULL | Provider-specific config |
| created_at | TIMESTAMP | NOT NULL | Creation timestamp |
| updated_at | TIMESTAMP | NOT NULL | Last update timestamp |

#### integration_configs Table

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | SERIAL | PRIMARY KEY | Auto-increment ID |
| type | VARCHAR(50) | NOT NULL | Integration type (GITHUB/JIRA) |
| name | VARCHAR(255) | NOT NULL | Configuration name |
| api_endpoint | VARCHAR(500) | NOT NULL | API endpoint URL |
| credentials_encrypted | TEXT | NOT NULL | Encrypted credentials |
| user_id | INTEGER | NULL | User identifier |
| is_active | BOOLEAN | DEFAULT true | Is active |
| config | JSONB | NULL | Integration-specific config |
| created_at | TIMESTAMP | NOT NULL | Creation timestamp |
| updated_at | TIMESTAMP | NOT NULL | Last update timestamp |

## API Design

### RESTful Principles

The API follows RESTful design principles with clear resource-based URLs and appropriate HTTP methods.

### Endpoint Patterns

#### Review Endpoints

```
POST   /api/reviews/local          # Create local review
POST   /api/reviews/pr             # Create PR review
GET    /api/reviews                 # List reviews
GET    /api/reviews/{id}           # Get review details
GET    /api/reviews/{id}/comments  # Get review comments
DELETE /api/reviews/{id}           # Delete review
POST   /api/reviews/{id}/retry     # Retry failed review
```

#### AI Model Endpoints

```
GET    /api/ai-models               # List available models
GET    /api/ai-models/{id}         # Get model details
POST   /api/ai-models               # Create model configuration
PUT    /api/ai-models/{id}         # Update model configuration
DELETE /api/ai-models/{id}         # Delete model configuration
POST   /api/ai-models/{id}/test    # Test model connection
POST   /api/ai-models/{id}/default # Set as default
```

#### Integration Endpoints

```
GET    /api/integrations/github    # Get GitHub configuration
POST   /api/integrations/github    # Configure GitHub integration
GET    /api/integrations/jira      # Get Jira configuration
POST   /api/integrations/jira     # Configure Jira integration
POST   /api/integrations/{type}/test # Test integration
```

#### Template Endpoints

```
GET    /api/templates               # List review templates
GET    /api/templates/{id}         # Get template details
POST   /api/templates              # Create template
PUT    /api/templates/{id}        # Update template
DELETE /api/templates/{id}        # Delete template
```

### Request/Response Format

**Request Headers**:
```
Content-Type: application/json
Authorization: Bearer <token> (when implemented)
```

**Response Format**:
```json
{
  "success": true,
  "data": { ... },
  "error": null
}
```

**Error Response**:
```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable error message",
    "details": { ... }
  }
}
```

### Example API Calls

#### Create Local Review

```bash
POST /api/reviews/local
Content-Type: application/json

{
  "repo_path": "/path/to/repo",
  "base_commit": "main",
  "head_commit": "feature-branch",
  "options": {
    "ai_model": "ollama",
    "include_design_patterns": true,
    "severity_level": "medium"
  }
}
```

#### Create PR Review

```bash
POST /api/reviews/pr
Content-Type: application/json

{
  "owner": "organization",
  "repo": "repository",
  "pr_number": 123,
  "options": {
    "ai_model": "ollama",
    "post_to_github": false,
    "include_jira_context": true
  }
}
```

## AI Integration

### AI Provider Abstraction

**Design Pattern**: Strategy Pattern + Factory Pattern

```rust
trait AIProvider {
    async fn generate_review(
        &self,
        prompt: &str,
        config: &AIConfig,
    ) -> Result<AIResponse, AIError>;
    
    fn validate_config(&self, config: &AIConfig) -> Result<(), ValidationError>;
}

struct OllamaProvider { ... }
struct OpenAIProvider { ... }
struct AnthropicProvider { ... }
```

### Prompt Engineering

**Prompt Structure**:
1. **Context Section**: Project information, language, framework
2. **Code Section**: Diff/changes with file context
3. **Guidelines Section**: Review standards, design patterns
4. **Output Format**: Structured response format

**Example Prompt Template**:
```
You are an expert code reviewer specializing in {language} and {framework}.

PROJECT CONTEXT:
- Project: {project_name}
- Language: {language}
- Framework: {framework}
- Design Patterns Used: {patterns}

CODE CHANGES:
{diff_content}

REVIEW GUIDELINES:
- Check for security vulnerabilities
- Validate design pattern usage
- Ensure consistency with codebase
- Review performance implications
- Check test coverage

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
```

### Model Selection and Fallback

**Strategy**:
1. Primary model: User-selected or default (Ollama)
2. Fallback: Automatic fallback on failure
3. Retry logic: Exponential backoff
4. Circuit breaker: Prevent cascading failures

### Response Parsing and Validation

**Structured Output**:
- Parse JSON responses
- Validate structure
- Extract comments and suggestions
- Handle parsing errors gracefully

## Integration Architecture

### GitHub Integration

**Capabilities**:
- PR metadata retrieval
- File content and diffs
- Comment posting
- Status updates
- Webhook support (future)

**Authentication**:
- Personal Access Token (PAT)
- OAuth App tokens
- GitHub App (future)

**Rate Limiting**:
- Respect GitHub rate limits
- Implement request queuing
- Caching for frequently accessed data

### Jira Integration

**Capabilities**:
- Issue retrieval by key
- Acceptance criteria extraction
- Project context gathering
- Issue linking

**Authentication**:
- API Token authentication
- OAuth (future)

**Rate Limiting**:
- Respect Jira API limits
- Implement caching

### Git Operations

**Local Repository Access**:
- Path-based repository access
- Commit range processing
- Diff generation
- File content retrieval

**Security Considerations**:
- Validate repository paths
- Sanitize user inputs
- Prevent path traversal attacks

## Design Patterns

### 1. Strategy Pattern

**Usage**: AI Provider abstraction

```rust
trait AIProvider {
    async fn generate_review(&self, prompt: &str) -> Result<AIResponse>;
}

struct OllamaProvider { ... }
struct OpenAIProvider { ... }

struct AIService {
    provider: Box<dyn AIProvider>,
}
```

**Benefits**:
- Easy to add new AI providers
- Runtime model selection
- Testable with mocks

### 2. Factory Pattern

**Usage**: Create AI providers and services

```rust
struct AIProviderFactory;

impl AIProviderFactory {
    fn create(provider_type: ProviderType, config: Config) -> Box<dyn AIProvider> {
        match provider_type {
            ProviderType::Ollama => Box::new(OllamaProvider::new(config)),
            ProviderType::OpenAI => Box::new(OpenAIProvider::new(config)),
            // ...
        }
    }
}
```

### 3. Repository Pattern

**Usage**: Data access abstraction

```rust
trait ReviewRepository {
    async fn create(&self, review: Review) -> Result<Review>;
    async fn find_by_id(&self, id: i64) -> Result<Option<Review>>;
    async fn update(&self, review: Review) -> Result<Review>;
}

struct PostgresReviewRepository {
    db: DatabaseConnection,
}
```

**Benefits**:
- Testability with in-memory implementations
- Database-agnostic business logic
- Easy to mock in tests

### 4. Service Layer Pattern

**Usage**: Business logic separation

```rust
struct ReviewService {
    review_repo: Box<dyn ReviewRepository>,
    git_service: GitService,
    ai_service: AIService,
    context_service: ContextService,
}

impl ReviewService {
    async fn create_local_review(&self, request: LocalReviewRequest) -> Result<Review> {
        // Orchestrate review creation
    }
}
```

**Benefits**:
- Clear separation of concerns
- Reusable business logic
- Centralized error handling

### 5. Builder Pattern

**Usage**: Prompt construction

```rust
struct PromptBuilder {
    context: Option<String>,
    code_changes: Vec<CodeChange>,
    guidelines: Vec<String>,
}

impl PromptBuilder {
    fn with_context(mut self, context: String) -> Self { ... }
    fn with_code_changes(mut self, changes: Vec<CodeChange>) -> Self { ... }
    fn with_guidelines(mut self, guidelines: Vec<String>) -> Self { ... }
    fn build(self) -> String { ... }
}
```

**Benefits**:
- Flexible prompt construction
- Readable and maintainable
- Validates completeness

### 6. Observer Pattern (Future)

**Usage**: Review status updates

```rust
trait ReviewObserver {
    fn on_status_change(&self, review: &Review, status: ReviewStatus);
}

struct ReviewNotifier {
    observers: Vec<Box<dyn ReviewObserver>>,
}
```

### 7. Template Method Pattern

**Usage**: Review process workflow

```rust
trait ReviewWorkflow {
    async fn execute(&self) -> Result<Review> {
        self.prepare_context().await?;
        self.generate_prompt().await?;
        self.get_ai_feedback().await?;
        self.process_results().await?;
        Ok(self.complete())
    }
    
    // Abstract methods to be implemented
    async fn prepare_context(&self) -> Result<()>;
    async fn generate_prompt(&self) -> Result<String>;
    // ...
}
```

## Security Architecture

### Authentication & Authorization

**Planned Features**:
- API key authentication
- OAuth2 integration
- Role-based access control (RBAC)
- User-specific review permissions

**Current Approach**:
- Basic API key validation
- User identification via user_id

### Credential Management

**Encryption**:
- Encrypt sensitive credentials in database
- Use environment variables for secrets
- Support for secret management services (Vault, etc.)

**Credential Storage**:
- Encrypted API keys
- Encrypted GitHub tokens
- Encrypted Jira credentials
- No plaintext storage

### Input Validation

**Security Measures**:
- Path traversal prevention
- SQL injection prevention (via ORM)
- XSS prevention
- Command injection prevention
- Input sanitization

### Rate Limiting

**Implementation**:
- Per-user rate limiting
- Per-IP rate limiting
- Per-endpoint rate limiting
- Configurable limits

## Error Handling

### Error Types

1. **Validation Errors** (400 Bad Request)
   - Invalid request format
   - Missing required fields
   - Invalid parameter values

2. **Authentication Errors** (401 Unauthorized)
   - Missing or invalid API key
   - Expired credentials

3. **Authorization Errors** (403 Forbidden)
   - Insufficient permissions
   - Access denied

4. **Not Found Errors** (404 Not Found)
   - Review not found
   - Resource not found

5. **Integration Errors** (502 Bad Gateway)
   - GitHub API failures
   - Jira API failures
   - AI provider failures

6. **Internal Server Errors** (500 Internal Server Error)
   - Database failures
   - Processing errors
   - Unexpected errors

### Error Response Format

```json
{
  "success": false,
  "error": {
    "code": "REVIEW_NOT_FOUND",
    "message": "Review with ID 123 not found",
    "details": {
      "review_id": 123
    },
    "timestamp": "2024-01-01T00:00:00Z"
  }
}
```

### Error Handling Strategy

- **Early Validation**: Validate requests as early as possible
- **Graceful Degradation**: Handle external API failures gracefully
- **Retry Logic**: Automatic retry for transient failures
- **Detailed Logging**: Log errors with full context
- **User-Friendly Messages**: Provide actionable error messages

## Performance Considerations

### Database Optimization

1. **Indexes**: Proper indexes on frequently queried columns
2. **Connection Pooling**: Efficient connection management
3. **Query Optimization**: Avoid N+1 problems, use efficient queries
4. **Caching**: Cache frequently accessed data

### AI Provider Optimization

1. **Prompt Optimization**: Minimize token usage
2. **Streaming Responses**: Support streaming for long responses (future)
3. **Batch Processing**: Batch similar requests
4. **Caching**: Cache similar review requests

### Integration Optimization

1. **Parallel Requests**: Fetch from multiple sources in parallel
2. **Caching**: Cache external API responses
3. **Rate Limit Management**: Respect and optimize rate limits
4. **Connection Reuse**: Reuse HTTP connections

### Git Operations Optimization

1. **Incremental Processing**: Process only changed files
2. **Parallel File Processing**: Process multiple files in parallel
3. **Caching**: Cache repository metadata

### Server Performance

1. **Async I/O**: Actix-web uses async for non-blocking operations
2. **Resource Management**: Proper cleanup of resources
3. **Memory Management**: Efficient memory usage
4. **CPU Optimization**: Parallel processing where possible

## Scalability

### Horizontal Scaling

**Design**:
- Stateless API design
- Database connection pooling
- Load balancer support
- Shared database

**Considerations**:
- Session management (if added)
- Cache coordination
- Message queue for async operations

### Vertical Scaling

**Resource Management**:
- Monitor CPU and memory usage
- Optimize database queries
- Efficient AI request handling
- Resource limits and quotas

### Future Enhancements

1. **Caching Layer**: Redis for distributed caching
2. **Message Queue**: RabbitMQ/Kafka for async review processing
3. **Monitoring**: Prometheus + Grafana
4. **API Gateway**: Rate limiting, authentication, routing
5. **CDN**: For static assets and responses

## Deployment Architecture

### Containerization

**Docker Support**:
- Multi-stage builds
- Optimized images
- Health checks
- Environment-based configuration

### Docker Compose

**Services**:
- API server
- PostgreSQL database
- Optional: Redis cache
- Optional: Message queue

### Production Considerations

1. **Secrets Management**: Use secrets management service
2. **Backup Strategy**: Regular database backups
3. **Monitoring**: Production monitoring and alerting
4. **Scaling**: Auto-scaling based on load
5. **High Availability**: Multi-instance deployment
6. **Disaster Recovery**: Backup and restore procedures

### Environment Configuration

**Development**:
- Local database
- Local Ollama instance
- Development API keys

**Staging**:
- Shared database
- Staging integrations
- Limited API access

**Production**:
- Production database with backups
- Production integrations
- Full security and monitoring

## Future Enhancements

### Planned Features

1. **Webhook Support**: Real-time PR review triggers
2. **Custom Rules**: User-defined review rules
3. **Review Templates**: Customizable review templates
4. **Review Analytics**: Review statistics and insights
5. **Multi-Repository Support**: Review across multiple repos
6. **Collaborative Reviews**: Team review workflows
7. **Review History**: Historical review tracking
8. **Export Options**: Export reviews in various formats
9. **CLI Tool**: Command-line interface for developers
10. **IDE Integration**: IDE plugins for direct integration

### Technical Improvements

1. **GraphQL API**: More flexible querying
2. **WebSocket Support**: Real-time updates
3. **Streaming Responses**: Stream AI responses
4. **Advanced Caching**: Multi-layer caching strategy
5. **Machine Learning**: Learn from review patterns
6. **Codebase Embeddings**: Semantic code search

---

This document provides a comprehensive overview of the Sonar-Mochi system design. It will be updated as the project evolves and new features are added.

