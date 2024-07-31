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

## Error Handling

The server returns appropriate HTTP status codes for different scenarios:

- 200 OK: Successful request
- 401 Unauthorized: Invalid or missing API key
- 404 Not Found: Project not found
- 500 Internal Server Error: Server-side error

When an error occurs, the server will return an appropriate error message in the response body.