# Contexter Server Documentation

The Contexter server provides an API for managing and retrieving project contexts. This document outlines the available endpoints and how to interact with them.

## Authentication

All API endpoints require authentication using an API key. The API key should be included in the `X-API-Key` header of each request.

To generate a new API key, use the following command:

```bash
contexter config generate-key
```

## Endpoints

### List Projects

Retrieves a list of all available projects.

- **URL:** `/projects`
- **Method:** GET
- **Headers:**
  - `X-API-Key`: Your API key

**Example curl command:**

```bash
curl -X GET "http://localhost:3030/projects" \
     -H "X-API-Key: your_api_key_here"
```

**Example response:**

```json
{
  "projects": ["project1", "project2", "project3"]
}
```

### Get Project Content

Retrieves the concatenated content of all files in a specific project.

- **URL:** `/project/{project_name}`
- **Method:** GET
- **Headers:**
  - `X-API-Key`: Your API key

**Example curl command:**

```bash
curl -X GET "http://localhost:3030/project/project1" \
     -H "X-API-Key: your_api_key_here"
```

**Example response:**

```json
{
  "content": "... concatenated content of all files in the project ..."
}
```

### List Project Files

Retrieves a list of all files in a specific project.

- **URL:** `/project/{project_name}/files`
- **Method:** GET
- **Headers:**
  - `X-API-Key`: Your API key

**Example curl command:**

```bash
curl -X GET "http://localhost:3030/project/project1/files" \
     -H "X-API-Key: your_api_key_here"
```

**Example response:**

```json
{
  "files": ["file1.rs", "file2.rs", "subfolder/file3.rs"]
}
```

### Run Contexter

Runs the Contexter on specified files or a path within a project.

- **URL:** `/project/{project_name}/contexter`
- **Method:** POST
- **Headers:**
  - `X-API-Key`: Your API key
  - `Content-Type: application/json`
- **Body:**
  ```json
  {
    "files": ["file1.rs", "file2.rs"],
    "path": "subfolder"
  }
  ```
  Note: Either `files` or `path` must be specified, but not both.

**Example curl command:**

```bash
curl -X POST "http://localhost:3030/project/project1/contexter" \
     -H "X-API-Key: your_api_key_here" \
     -H "Content-Type: application/json" \
     -d '{"files": ["file1.rs", "file2.rs"]}'
```

**Example response:**

```json
{
  "content": "... concatenated content of specified files ..."
}
```

## Server Management

(The rest of the documentation remains the same...)