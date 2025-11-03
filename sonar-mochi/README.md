# Sonar-Mochi - AI-Powered PR Review System

An intelligent code review system that leverages AI models to help developers and reviewers assess code changes in Git Pull Requests. Sonar-Mochi supports both local pre-push reviews and remote PR reviews, with context integration from GitHub and Jira.

## Overview

Sonar-Mochi is designed to provide intelligent, context-aware code reviews for modern development workflows. It supports multiple AI providers (with Ollama as default), integrates with GitHub and Jira for comprehensive context, and understands common design patterns across Java-SpringBoot, JavaScript/TypeScript-React, and Rust-ActixWeb projects.

## Key Features

### 🔍 Dual Review Modes

- **Developer Mode**: Review your local changes before pushing to remote
  - Pre-commit validation
  - Pre-push quality checks
  - Feature branch review

- **Reviewer Mode**: Review PRs on remote origin server
  - Automated PR analysis
  - Consistency checking
  - Design pattern validation

### 🤖 AI-Powered Analysis

- **Multiple AI Providers**: Support for Ollama (default), OpenAI, Anthropic, and more
- **Context-Aware Reviews**: Understands project structure and history
- **Design Pattern Detection**: Identifies and validates design patterns
- **Intelligent Suggestions**: Actionable feedback with code examples

### 🔗 Context Integration

- **GitHub Integration**: PR metadata, comments, discussions, and history
- **Jira Integration**: Issue details, acceptance criteria, project context
- **Repository Analysis**: Structure, dependencies, and conventions

### 📦 Multi-Language Support

- **Java (Spring Boot)**: Spring Boot conventions and best practices
- **JavaScript/TypeScript (React)**: React patterns and modern JS practices
- **Rust (Actix-web)**: Rust idioms and Actix-web best practices

## Architecture

Built with Rust and Actix-web for high performance and reliability.

```
┌─────────────────┐
│   Client/API    │
└────────┬────────┘
         │
┌────────▼────────────────────────────────────┐
│         Sonar-Mochi API Server              │
│  ┌────────────────────────────────────────┐ │
│  │   Service Layer                        │ │
│  │  - Review Service                      │ │
│  │  - AI Service                          │ │
│  │  - Git Service                         │ │
│  │  - Context Service                     │ │
│  └────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────┐ │
│  │   Integration Layer                    │ │
│  │  - GitHub Client                       │ │
│  │  - Jira Client                         │ │
│  │  - Git Operations                      │ │
│  └────────────────────────────────────────┘ │
└────────┬───────────────────┬────────────────┘
         │                   │
    ┌────▼────┐         ┌────▼─────┐
    │Database │         │AI Models │
    └─────────┘         └──────────┘
```

For detailed architecture documentation, see [SYSTEM_DESIGN.md](./SYSTEM_DESIGN.md).

## Tech Stack

### Backend
- **Language**: Rust (Edition 2021)
- **Web Framework**: Actix-web 4.x
- **Database**: SeaORM with PostgreSQL
- **HTTP Client**: Reqwest for external APIs
- **Logging**: Tracing and tracing-subscriber

### AI Integration
- **Default**: Ollama (local AI models)
- **Supported**: OpenAI, Anthropic Claude, Azure OpenAI
- **Extensible**: Plugin architecture for new providers

### Integrations
- **GitHub**: GitHub REST API
- **Jira**: Jira REST API
- **Git**: Local Git operations via `git2` or `gix` crate

## Getting Started

### Prerequisites

- **Rust** (latest stable version)
- **PostgreSQL** (v12 or higher)
- **Ollama** (for default AI provider) - [Install Ollama](https://ollama.ai)
- **Git** (for local repository access)

### Quick Start

1. **Clone and navigate to the project**:
   ```bash
   cd sonar-mochi
   ```

2. **Set up environment**:
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Set up database**:
   ```bash
   # Run migrations
   cargo install sea-orm-cli
   sea-orm-cli migrate up
   ```

4. **Start the server**:
   ```bash
   cargo run
   ```

5. **Verify installation**:
   ```bash
   curl http://localhost:8080/health
   ```

## Configuration

### Environment Variables

Create a `.env` file with the following variables:

```env
# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# Database
DATABASE_URL=postgresql://user:password@localhost:5432/sonar_mochi

# Default AI Provider (Ollama)
OLLAMA_BASE_URL=http://localhost:11434
OLLAMA_DEFAULT_MODEL=llama2

# GitHub Integration (optional)
GITHUB_TOKEN=your_github_token

# Jira Integration (optional)
JIRA_URL=https://your-domain.atlassian.net
JIRA_EMAIL=your-email@example.com
JIRA_API_TOKEN=your_jira_token

# Logging
RUST_LOG=info
```

### AI Model Configuration

The default AI provider is Ollama. To configure:

1. **Install Ollama**: [https://ollama.ai](https://ollama.ai)
2. **Pull a model**:
   ```bash
   ollama pull llama2
   ```
3. **Configure in application**:
   - Use default Ollama settings, or
   - Configure custom models via API

## Usage Examples

### Local Review

Review local changes before pushing:

```bash
POST /api/reviews/local
Content-Type: application/json

{
  "repo_path": "/path/to/your/repo",
  "base_commit": "main",
  "head_commit": "HEAD",
  "options": {
    "ai_model": "ollama",
    "include_design_patterns": true
  }
}
```

### PR Review

Review a remote PR:

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

## API Documentation

### Review Endpoints

- `POST /api/reviews/local` - Create local review
- `POST /api/reviews/pr` - Create PR review
- `GET /api/reviews` - List reviews
- `GET /api/reviews/{id}` - Get review details
- `GET /api/reviews/{id}/comments` - Get review comments

### AI Model Endpoints

- `GET /api/ai-models` - List available models
- `POST /api/ai-models` - Create model configuration
- `PUT /api/ai-models/{id}` - Update model configuration
- `POST /api/ai-models/{id}/test` - Test model connection

### Integration Endpoints

- `GET /api/integrations/github` - Get GitHub configuration
- `POST /api/integrations/github` - Configure GitHub
- `GET /api/integrations/jira` - Get Jira configuration
- `POST /api/integrations/jira` - Configure Jira

## Project Structure

```
sonar-mochi/
├── src/
│   ├── main.rs              # Application entry point
│   ├── web/                 # Web server and routes
│   │   ├── server.rs        # Server initialization
│   │   ├── middleware.rs    # Custom middleware
│   │   └── handlers/        # Request handlers
│   ├── services/            # Service layer
│   │   ├── review.rs        # Review service
│   │   ├── git.rs           # Git operations
│   │   └── context.rs       # Context aggregation
│   ├── ai/                  # AI integration
│   │   ├── provider.rs      # AI provider abstraction
│   │   ├── ollama.rs        # Ollama provider
│   │   ├── openai.rs        # OpenAI provider
│   │   └── prompt.rs        # Prompt building
│   ├── integrations/        # External integrations
│   │   ├── github.rs        # GitHub client
│   │   └── jira.rs          # Jira client
│   ├── database/            # Database layer
│   │   ├── entities/        # SeaORM entities
│   │   ├── migrations/      # Database migrations
│   │   └── repositories/    # Repository implementations
│   ├── models/              # Data models
│   ├── config/              # Configuration
│   └── utils/               # Utility functions
├── migrations/              # Database migrations
├── tests/                   # Integration tests
├── Cargo.toml               # Rust dependencies
├── README.md                # This file
└── SYSTEM_DESIGN.md         # Detailed system design
```

## Design Patterns

The project implements several design patterns:

1. **Strategy Pattern**: AI provider abstraction
2. **Factory Pattern**: Service and provider creation
3. **Repository Pattern**: Data access abstraction
4. **Service Layer Pattern**: Business logic separation
5. **Builder Pattern**: Prompt construction
6. **Template Method Pattern**: Review workflow

See [SYSTEM_DESIGN.md](./SYSTEM_DESIGN.md) for detailed pattern documentation.

## Development

### Running Tests

```bash
cargo test
```

### Running Migrations

```bash
sea-orm-cli migrate up
sea-orm-cli migrate down
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

[Add license information]

## Support

For issues and questions:
- Create an issue in the repository
- Check [SYSTEM_DESIGN.md](./SYSTEM_DESIGN.md) for architecture details

---

**Note**: This project is in active development. Some features may not be fully implemented yet.

