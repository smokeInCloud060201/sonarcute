# SonarCute API - API Documentation

Complete API endpoint reference for the SonarCute backend service.

## Table of Contents

- [Base Information](#base-information)
- [Authentication](#authentication)
- [Endpoints](#endpoints)
  - [Admin Token Management](#admin-token-management)
  - [Project Management](#project-management)
  - [Analysis & Results](#analysis--results)
  - [Quality Gate Management](#quality-gate-management)
- [Error Responses](#error-responses)
- [Examples](#examples)

## Base Information

### Base URL
```
http://localhost:8888/api
```

### Content Type
All requests and responses use `application/json`.

### Response Format
Successful responses return data directly. Error responses follow the error format (see [Error Responses](#error-responses)).

## Authentication

The API uses SonarQube token-based authentication. Tokens must be created before using the API:

1. Create a `USER_TOKEN` for administrative operations (creating/deleting projects)
2. Create a `GLOBAL_ANALYSIS_TOKEN` for reading analysis results

Tokens are stored in the database and automatically used when making SonarQube API calls.

## Endpoints

### Admin Token Management

#### Create Admin Token

Create an admin token for SonarQube operations.

**Endpoint**: `POST /api/admin-token`

**Description**: Creates an admin token in SonarQube and stores it in the database. Required before creating projects or fetching results.

**Request Body**:
```json
{
  "username": "string",          // SonarQube username
  "password": "string",          // SonarQube password
  "token_name": "string",        // Token identifier name
  "token_type": "string",        // "USER_TOKEN" or "GLOBAL_ANALYSIS_TOKEN"
  "sonar_host_url": "string"     // SonarQube instance URL
}
```

**Token Types**:
- `USER_TOKEN`: Used for administrative operations (create/delete projects). Requires admin privileges.
- `GLOBAL_ANALYSIS_TOKEN`: Used for reading analysis results (issues, coverage, quality gates).

**Response** (200 OK):
```json
{
  "id": 1,
  "username": "admin",
  "token_name": "api_admin_token",
  "token_value": "squ_xxxxxxxxxxxxxxxxxxxxxxxxxxxx",
  "token_type": "USER_TOKEN",
  "sonar_host_url": "http://localhost:9000",
  "created_at": "2024-12-01T10:00:00",
  "updated_at": "2024-12-01T10:00:00"
}
```

**Error Responses**:
- `400 Bad Request`: Invalid request body
- `500 Internal Server Error`: Failed to create token in SonarQube or database

**Example**:
```bash
curl -X POST http://localhost:8888/api/admin-token \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin",
    "token_name": "api_user_token",
    "token_type": "USER_TOKEN",
    "sonar_host_url": "http://localhost:9000"
  }'
```

---

### Project Management

#### Get All Projects

Retrieve all registered projects.

**Endpoint**: `GET /api/projects`

**Description**: Returns a list of all projects stored in the database.

**Response** (200 OK):
```json
[
  {
    "id": 1,
    "project_key": "my-project",
    "project_name": "My Project",
    "project_path": "/path/to/project",
    "sonar_token": "squ_xxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    "sonar_host_url": "http://localhost:9000",
    "language": "java",
    "sources_path": "src/main/java",
    "tests_path": "src/test/java",
    "coverage_report_path": "build/reports/jacoco/test/jacocoTestReport.xml",
    "created_at": "2024-12-01T10:00:00",
    "updated_at": "2024-12-01T10:00:00"
  }
]
```

**Error Responses**:
- `500 Internal Server Error`: Database error

**Example**:
```bash
curl http://localhost:8888/api/projects
```

---

#### Create Project

Create a new SonarQube project and register it in the database.

**Endpoint**: `POST /api/projects`

**Description**: Creates a project in SonarQube and stores it in the database. Also generates a project-specific analysis token.

**Prerequisites**: A `USER_TOKEN` must exist for the SonarQube instance.

**Request Body**:
```json
{
  "project_key": "string",              // Unique SonarQube project key
  "project_name": "string",             // Display name
  "project_path": "string",             // Local file system path (unique)
  "language": "string",                 // Programming language (e.g., "java", "js")
  "sources_path": "string",             // Source code directory
  "tests_path": "string",               // Test code directory
  "coverage_report_path": "string"      // Optional: Coverage report path
}
```

**Response** (200 OK):
```json
{
  "id": 1,
  "project_key": "my-project",
  "project_name": "My Project",
  "project_path": "/path/to/project",
  "sonar_token": "squ_xxxxxxxxxxxxxxxxxxxxxxxxxxxx",
  "sonar_host_url": "http://localhost:9000",
  "language": "java",
  "sources_path": "src/main/java",
  "tests_path": "src/test/java",
  "coverage_report_path": "build/reports/jacoco/test/jacocoTestReport.xml",
  "created_at": "2024-12-01T10:00:00",
  "updated_at": "2024-12-01T10:00:00"
}
```

**Error Responses**:
- `400 Bad Request`: No USER_TOKEN found for SonarQube instance
- `400 Bad Request`: Invalid request body
- `500 Internal Server Error`: Failed to create project in SonarQube or database
- `500 Internal Server Error`: Failed to create project token

**Example**:
```bash
curl -X POST http://localhost:8888/api/projects \
  -H "Content-Type: application/json" \
  -d '{
    "project_key": "my-java-project",
    "project_name": "My Java Project",
    "project_path": "/home/user/projects/my-java-project",
    "language": "java",
    "sources_path": "src/main/java",
    "tests_path": "src/test/java",
    "coverage_report_path": "build/reports/jacoco/test/jacocoTestReport.xml"
  }'
```

---

#### Delete Project

Delete a project from both SonarQube and the database.

**Endpoint**: `DELETE /api/projects`

**Description**: Deletes a project from SonarQube (if privileges allow) and removes it from the database.

**Prerequisites**: A `USER_TOKEN` with admin privileges must exist.

**Request Body**:
```json
{
  "project_path": "string"    // Project path to identify the project
}
```

**Response** (200 OK):
```json
{
  "message": "Project deleted successfully from both SonarQube and database",
  "project_key": "my-project",
  "project_path": "/path/to/project"
}
```

**Error Responses**:
- `400 Bad Request`: No USER_TOKEN found
- `403 Forbidden`: Insufficient privileges to delete from SonarQube
- `404 Not Found`: Project not found
- `500 Internal Server Error`: Database error

**Note**: If SonarQube deletion fails due to insufficient privileges, the project is NOT deleted from the database. The response will indicate this.

**Example**:
```bash
curl -X DELETE http://localhost:8888/api/projects \
  -H "Content-Type: application/json" \
  -d '{
    "project_path": "/home/user/projects/my-java-project"
  }'
```

---

### Analysis & Results

#### Get Project Results

Retrieve code quality analysis results for a project.

**Endpoint**: `POST /api/results`

**Description**: Fetches issues, coverage metrics, and quality gate status from SonarQube for the specified project.

**Prerequisites**: 
- A `GLOBAL_ANALYSIS_TOKEN` must exist for the SonarQube instance
- Project must have been analyzed in SonarQube

**Request Body**:
```json
{
  "project_path": "string"    // Project path to identify the project
}
```

**Response** (200 OK):
```json
{
  "project": {
    "id": 1,
    "project_key": "my-project",
    "project_name": "My Project",
    "project_path": "/path/to/project",
    "sonar_token": "squ_xxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    "sonar_host_url": "http://localhost:9000",
    "language": "java",
    "sources_path": "src/main/java",
    "tests_path": "src/test/java",
    "coverage_report_path": "build/reports/jacoco/test/jacocoTestReport.xml",
    "created_at": "2024-12-01T10:00:00",
    "updated_at": "2024-12-01T10:00:00"
  },
  "issues": {
    "issues": [
      {
        "key": "AXxxxxx",
        "rule": "java:S1234",
        "severity": "MAJOR",
        "component": "my-project:src/main/java/Example.java",
        "project": "my-project",
        "line": 42,
        "message": "Remove this unused method parameter.",
        "status": "OPEN",
        "type": "CODE_SMELL",
        "creationDate": "2024-12-01T10:00:00+0000",
        "updateDate": "2024-12-01T10:00:00+0000",
        "tags": ["unused"]
      }
    ],
    "paging": {
      "pageIndex": 1,
      "pageSize": 500,
      "total": 42
    }
  },
  "coverage": {
    "component": {
      "id": "AXxxxxx",
      "key": "my-project",
      "name": "My Project",
      "qualifier": "TRK",
      "measures": [
        {
          "metric": "coverage",
          "value": "85.5"
        },
        {
          "metric": "branch_coverage",
          "value": "78.2"
        },
        {
          "metric": "line_coverage",
          "value": "87.3"
        }
      ]
    }
  },
  "quality_gate": {
    "projectStatus": {
      "status": "OK",
      "conditions": [
        {
          "status": "OK",
          "metricKey": "new_coverage",
          "comparator": "LT",
          "errorThreshold": "80",
          "actualValue": "85.5"
        }
      ],
      "ignoredConditions": false
    }
  }
}
```

**Error Responses**:
- `400 Bad Request`: No GLOBAL_ANALYSIS_TOKEN found
- `404 Not Found`: Project not found
- `500 Internal Server Error`: Database error
- Partial errors may be included in response:
  - `issues_error`: Error fetching issues
  - `coverage_error`: Error fetching coverage (if not a decoding error)
  - Missing coverage/quality gate may return default messages if data unavailable

**Notes**:
- Issues, coverage, and quality gate are fetched in parallel for performance
- If coverage or quality gate data is not available, appropriate messages are included instead of errors
- Issues are limited to 500 per request (SonarQube default)

**Example**:
```bash
curl -X POST http://localhost:8888/api/results \
  -H "Content-Type: application/json" \
  -d '{
    "project_path": "/home/user/projects/my-java-project"
  }'
```

---

#### Generate Sonar Command

Generate a SonarQube scanner command for a project.

**Endpoint**: `POST /api/generate-command`

**Description**: Generates a ready-to-use SonarQube scanner command with all necessary parameters pre-configured.

**Request Body**:
```json
{
  "project_path": "string"    // Project path to identify the project
}
```

**Response** (200 OK):
```json
{
  "command": "./gradlew test sonar -Dsonar.token=squ_xxx -Dsonar.host.url=http://localhost:9000 -Dsonar.projectKey=my-project -Dsonar.projectName=My Project -Dsonar.coverage.jacoco.xmlReportPaths=build/reports/jacoco/test/jacocoTestReport.xml -Dsonar.language=java -Dsonar.sources=src/main/java -Dsonar.tests=src/test/java",
  "project_path": "/path/to/project"
}
```

**Command Format**:
The generated command includes:
- SonarQube token (for authentication)
- SonarQube host URL
- Project key and name
- Coverage report path (if available)
- Language
- Sources and tests paths

**Error Responses**:
- `404 Not Found`: Project not found
- `500 Internal Server Error`: Database error

**Example**:
```bash
curl -X POST http://localhost:8888/api/generate-command \
  -H "Content-Type: application/json" \
  -d '{
    "project_path": "/home/user/projects/my-java-project"
  }'
```

**Usage**:
Copy the generated command and run it in your project directory:
```bash
cd /path/to/project
./gradlew test sonar -Dsonar.token=squ_xxx ...
```

---

### Quality Gate Management

#### Get All Quality Gates

Retrieve all quality gates from SonarQube.

**Endpoint**: `GET /api/quality-gates`

**Description**: Returns a list of all quality gates available in SonarQube.

**Prerequisites**: A `USER_TOKEN` must exist for the SonarQube instance.

**Response** (200 OK):
```json
{
  "qualitygates": [
    {
      "id": "1",
      "name": "Sonar way",
      "isDefault": true,
      "isBuiltIn": true
    },
    {
      "id": "2",
      "name": "Kiosk Gate",
      "isDefault": false,
      "isBuiltIn": false
    }
  ]
}
```

**Error Responses**:
- `400 Bad Request`: No USER_TOKEN found
- `500 Internal Server Error`: Failed to fetch quality gates from SonarQube

**Example**:
```bash
curl http://localhost:8888/api/quality-gates
```

---

#### Get Quality Gate Details

Get detailed information about a specific quality gate, including all conditions.

**Endpoint**: `GET /api/quality-gates/details?name={gate_name}`

**Description**: Returns detailed information about a quality gate, including all its conditions (metrics, operators, thresholds).

**Prerequisites**: A `USER_TOKEN` must exist for the SonarQube instance.

**Query Parameters**:
- `name` (required): Name of the quality gate

**Response** (200 OK):
```json
{
  "id": "2",
  "name": "Kiosk Gate",
  "conditions": [
    {
      "id": "1",
      "metric": "new_coverage",
      "op": "LT",
      "error": "80"
    },
    {
      "id": "2",
      "metric": "new_security_hotspots_reviewed",
      "op": "LT",
      "error": "100"
    }
  ]
}
```

**Error Responses**:
- `400 Bad Request`: No USER_TOKEN found
- `500 Internal Server Error`: Failed to fetch quality gate details

**Example**:
```bash
curl "http://localhost:8888/api/quality-gates/details?name=Kiosk%20Gate"
```

---

#### Create Quality Gate

Create a new quality gate in SonarQube.

**Endpoint**: `POST /api/quality-gates`

**Description**: Creates a new quality gate. Optionally, a single condition can be added during creation.

**Prerequisites**: A `USER_TOKEN` with admin privileges must exist.

**Request Body**:
```json
{
  "name": "string",                    // Quality gate name
  "condition_metric": "string",        // Optional: Metric name
  "condition_op": "string",            // Optional: Operator (LT, GT, etc.)
  "condition_error": "string"          // Optional: Error threshold
}
```

**Response** (200 OK):
```json
{
  "message": "Quality gate created successfully",
  "name": "My Quality Gate"
}
```

**Error Responses**:
- `400 Bad Request`: No USER_TOKEN found
- `500 Internal Server Error`: Failed to create quality gate or add condition

**Example**:
```bash
curl -X POST http://localhost:8888/api/quality-gates \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Quality Gate",
    "condition_metric": "new_coverage",
    "condition_op": "LT",
    "condition_error": "80"
  }'
```

---

#### Update Quality Gate

Update an existing quality gate (rename, add conditions, delete conditions).

**Endpoint**: `PUT /api/quality-gates`

**Description**: Updates a quality gate. Supports renaming, adding multiple conditions, and deleting conditions by ID.

**Prerequisites**: A `USER_TOKEN` with admin privileges must exist.

**Request Body**:
```json
{
  "name": "string",                              // Current quality gate name
  "new_name": "string",                          // Optional: New name for the gate
  "condition_metric": "string",                  // Optional: Single condition metric (backward compatibility)
  "condition_op": "string",                      // Optional: Single condition operator
  "condition_error": "string",                   // Optional: Single condition threshold
  "add_conditions": [                            // Optional: Multiple conditions to add
    {
      "metric": "string",                        // Metric name (e.g., "new_coverage", "coverage")
      "op": "string",                           // Operator: "LT" (less than), "GT" (greater than)
      "error": "string"                         // Error threshold (e.g., "80", "1")
    }
  ],
  "delete_condition_ids": ["string"]             // Optional: Array of condition IDs to delete
}
```

**Common Metrics**:
- `new_coverage`, `coverage`: Code coverage percentage
- `new_security_hotspots_reviewed`, `security_hotspots_reviewed`: Security hotspots reviewed
- `new_software_quality_reliability_rating`, `software_quality_reliability_rating`: Reliability rating (1-5)
- `new_software_quality_security_rating`, `software_quality_security_rating`: Security rating (1-5)
- `new_software_quality_maintainability_rating`, `software_quality_maintainability_rating`: Maintainability rating (1-5)
- `new_software_quality_blocker_issues`, `software_quality_blocker_issues`: Blocker issues count
- `new_software_quality_high_issues`, `software_quality_high_issues`: High severity issues count
- `new_duplicated_lines_density`, `duplicated_lines_density`: Duplicated lines percentage

**Response** (200 OK):
```json
{
  "message": "Quality gate updated successfully",
  "name": "Updated Gate Name"
}
```

**Error Responses**:
- `400 Bad Request`: No USER_TOKEN found
- `500 Internal Server Error`: Failed to update quality gate, add condition, or delete condition

**Example - Add Multiple Conditions**:
```bash
curl -X PUT http://localhost:8888/api/quality-gates \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Kiosk Gate",
    "add_conditions": [
      {
        "metric": "new_coverage",
        "op": "LT",
        "error": "80"
      },
      {
        "metric": "new_security_hotspots_reviewed",
        "op": "LT",
        "error": "100"
      }
    ]
  }'
```

**Example - Delete Conditions and Add New Ones**:
```bash
curl -X PUT http://localhost:8888/api/quality-gates \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Kiosk Gate",
    "delete_condition_ids": ["1", "2"],
    "add_conditions": [
      {
        "metric": "coverage",
        "op": "LT",
        "error": "80"
      }
    ]
  }'
```

---

#### Delete Quality Gate

Delete a quality gate from SonarQube.

**Endpoint**: `DELETE /api/quality-gates`

**Description**: Deletes a quality gate. Built-in quality gates cannot be deleted.

**Prerequisites**: A `USER_TOKEN` with admin privileges must exist.

**Request Body**:
```json
{
  "name": "string"    // Quality gate name
}
```

**Response** (200 OK):
```json
{
  "message": "Quality gate deleted successfully",
  "name": "My Quality Gate"
}
```

**Error Responses**:
- `400 Bad Request`: No USER_TOKEN found
- `500 Internal Server Error`: Failed to delete quality gate (e.g., if it's built-in or assigned to projects)

**Example**:
```bash
curl -X DELETE http://localhost:8888/api/quality-gates \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Quality Gate"
  }'
```

---

#### Set Default Quality Gate

Set a quality gate as the default for new projects.

**Endpoint**: `POST /api/quality-gates/set-default`

**Description**: Sets a quality gate as the default. All new projects will use this quality gate.

**Prerequisites**: A `USER_TOKEN` with admin privileges must exist.

**Request Body**:
```json
{
  "name": "string"    // Quality gate name
}
```

**Response** (200 OK):
```json
{
  "message": "Quality gate set as default",
  "name": "Kiosk Gate"
}
```

**Error Responses**:
- `400 Bad Request`: No USER_TOKEN found
- `500 Internal Server Error`: Failed to set default quality gate

**Example**:
```bash
curl -X POST http://localhost:8888/api/quality-gates/set-default \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Kiosk Gate"
  }'
```

---

#### Assign Quality Gate to Project

Assign a quality gate to a specific project.

**Endpoint**: `POST /api/quality-gates/assign`

**Description**: Assigns a quality gate to a project. The project will use this quality gate for analysis evaluation.

**Prerequisites**: A `USER_TOKEN` with admin privileges must exist.

**Request Body**:
```json
{
  "project_key": "string",    // SonarQube project key
  "gate_name": "string"      // Quality gate name
}
```

**Response** (200 OK):
```json
{
  "message": "Quality gate assigned to project successfully",
  "project_key": "my-project",
  "gate_name": "Kiosk Gate"
}
```

**Error Responses**:
- `400 Bad Request`: No USER_TOKEN found
- `500 Internal Server Error`: Failed to assign quality gate to project

**Example**:
```bash
curl -X POST http://localhost:8888/api/quality-gates/assign \
  -H "Content-Type: application/json" \
  -d '{
    "project_key": "my-project",
    "gate_name": "Kiosk Gate"
  }'
```

## Error Responses

All error responses follow this format:

```json
{
  "error": "Human-readable error message",
  "suggestion": "Optional suggestion for resolution"
}
```

### Common Error Codes

| Status Code | Description | Common Causes |
|-------------|-------------|---------------|
| 400 | Bad Request | Invalid request body, missing required fields, missing tokens |
| 403 | Forbidden | Insufficient privileges for SonarQube operation |
| 404 | Not Found | Project not found, resource doesn't exist |
| 500 | Internal Server Error | Database error, SonarQube API error, internal processing error |

### Error Response Examples

**Missing Token**:
```json
{
  "error": "No USER_TOKEN found for this SonarQube instance. Please create a USER_TOKEN first.",
  "suggestion": "Use POST /api/admin-token with token_type: 'USER_TOKEN' (must be created with a user that has admin privileges)"
}
```

**Project Not Found**:
```json
{
  "error": "Project not found"
}
```

**Insufficient Privileges**:
```json
{
  "error": "Insufficient privileges to delete project from SonarQube",
  "details": "The admin token does not have the necessary permissions to delete projects. Please ensure the token has admin privileges in SonarQube, or create a new admin token with proper permissions.",
  "sonar_error": "Insufficient privileges",
  "suggestion": "You may need to recreate the admin token with a user that has administrator permissions, or manually delete the project from SonarQube UI."
}
```

## Examples

### Complete Workflow

1. **Create Admin Token**:
```bash
curl -X POST http://localhost:8888/api/admin-token \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin",
    "token_name": "api_admin",
    "token_type": "USER_TOKEN",
    "sonar_host_url": "http://localhost:9000"
  }'
```

2. **Create Analysis Token**:
```bash
curl -X POST http://localhost:8888/api/admin-token \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin",
    "token_name": "api_analysis",
    "token_type": "GLOBAL_ANALYSIS_TOKEN",
    "sonar_host_url": "http://localhost:9000"
  }'
```

3. **Create Project**:
```bash
curl -X POST http://localhost:8888/api/projects \
  -H "Content-Type: application/json" \
  -d '{
    "project_key": "example-project",
    "project_name": "Example Project",
    "project_path": "/home/user/projects/example",
    "language": "java",
    "sources_path": "src/main/java",
    "tests_path": "src/test/java"
  }'
```

4. **Generate Command**:
```bash
curl -X POST http://localhost:8888/api/generate-command \
  -H "Content-Type: application/json" \
  -d '{
    "project_path": "/home/user/projects/example"
  }'
```

5. **Run the command in your project directory** (from step 4 response)

6. **Get Results**:
```bash
curl -X POST http://localhost:8888/api/results \
  -H "Content-Type: application/json" \
  -d '{
    "project_path": "/home/user/projects/example"
  }'
```

### Quality Gate Workflow

1. **Create Quality Gate**:
```bash
curl -X POST http://localhost:8888/api/quality-gates \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Kiosk Gate"
  }'
```

2. **Update Quality Gate with Conditions**:
```bash
curl -X PUT http://localhost:8888/api/quality-gates \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Kiosk Gate",
    "add_conditions": [
      {
        "metric": "new_coverage",
        "op": "LT",
        "error": "80"
      },
      {
        "metric": "new_security_hotspots_reviewed",
        "op": "LT",
        "error": "100"
      }
    ]
  }'
```

3. **Get Quality Gate Details**:
```bash
curl "http://localhost:8888/api/quality-gates/details?name=Kiosk%20Gate"
```

4. **Set as Default** (optional):
```bash
curl -X POST http://localhost:8888/api/quality-gates/set-default \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Kiosk Gate"
  }'
```

5. **Assign to Project** (optional):
```bash
curl -X POST http://localhost:8888/api/quality-gates/assign \
  -H "Content-Type: application/json" \
  -d '{
    "project_key": "my-project",
    "gate_name": "Kiosk Gate"
  }'
```

---

For system architecture details, see [SYSTEM_DESIGN.md](SYSTEM_DESIGN.md).

