# Contexter Server Documentation

The Contexter server provides a RESTful API for managing and retrieving project contexts. This document outlines the available endpoints and how to interact with them.

## API Version

The current API version is v1. All endpoints are prefixed with `/api/v1/`.

## Authentication

All API endpoints require authentication using an API key. The API key should be included in the `X-API-Key` header of each request.

To generate a new API key, use the following command:

```bash
contexter config generate-key
```

## Endpoints

### List Projects

Retrieves a list of all available projects.

- **URL:** `/api/v1/projects`
- **Method:** GET
- **Headers:**
  - `X-API-Key`: Your API key

**Example curl command:**

```bash
curl -X GET "http://localhost:3030/api/v1/projects" \
     -H "X-API-Key: your_api_key_here"
```

**Example response:**

```json
{
  "projects": [
    {
      "name": "project1",
      "path": "/path/to/project1"
    },
    {
      "name": "project2",
      "path": "/path/to/project2"
    }
  ]
}
```

### Get Project Metadata

Retrieves metadata for a specific project, including the list of files.

- **URL:** `/api/v1/projects/{project-name}`
- **Method:** GET
- **Headers:**
  - `X-API-Key`: Your API key

**Example curl command:**

```bash
curl -X GET "http://localhost:3030/api/v1/projects/project1" \
     -H "X-API-Key: your_api_key_here"
```

**Example response:**

```json
{
  "name": "project1",
  "path": "/path/to/project1",
  "files": [
    "file1.rs",
    "file2.rs",
    "subfolder/file3.rs"
  ]
}
```

### Run Contexter

Runs the Contexter on a project, optionally specifying paths to include.

- **URL:** `/api/v1/projects/{project-name}`
- **Method:** POST
- **Headers:**
  - `X-API-Key`: Your API key
  - `Content-Type: application/json`
- **Body (optional):**
  ```json
  {
    "paths": ["file1.rs", "subfolder", "file2.rs"]
  }
  ```
  If no body is provided, the contexter will run on the entire project.

**Example curl command:**

```bash
curl -X POST "http://localhost:3030/api/v1/projects/project1" \
     -H "X-API-Key: your_api_key_here" \
     -H "Content-Type: application/json" \
     -d '{"paths": ["file1.rs", "subfolder"]}'
```

**Example response:**

```json
{
  "content": "... concatenated content of specified files or entire project ..."
}
```

## Error Handling

The API uses standard HTTP status codes to indicate the success or failure of requests. In case of an error, the response will include a JSON object with an `error` field containing a description of the error.

Example error response:

```json
{
  "error": "Project 'nonexistent_project' not found"
}
```

Common status codes:

- 200 OK: Successful request
- 400 Bad Request: Invalid request parameters
- 401 Unauthorized: Invalid or missing API key
- 404 Not Found: Requested resource not found
- 500 Internal Server Error: Server-side error

## Server Management

### Starting the Server

To start the Contexter server, use the following command:

```bash
contexter server
```

You can add the following flags:
- `--quiet`: Run the server in quiet mode (minimal output)
- `--verbose`: Run the server in verbose mode (debug output)

### Configuring the Server

You can configure the server using the following commands:

```bash
# Set the server port
contexter config set-port 8080

# Set the listen address
contexter config set-address 127.0.0.1

# Add a project
contexter config add-project project_name /path/to/project

# Remove a project
contexter config remove-project project_name

# List current configuration
contexter config list
```

## API Versioning

The current API version is v1. All endpoints are prefixed with `/api/v1/`. Future versions of the API may introduce changes or new features and will use a different version prefix (e.g., `/api/v2/`).