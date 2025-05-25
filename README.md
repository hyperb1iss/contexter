# 🚀 Contexter

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white&color=ff1493)
![License](https://img.shields.io/github/license/hyperb1iss/contexter?style=for-the-badge&logo=apache&logoColor=white&color=00ffff)
![Chrome Extension](https://img.shields.io/badge/Chrome-Extension-green?style=for-the-badge&logo=googlechrome&logoColor=white&color=00ff7f)

*A powerful server and extension ecosystem for gathering intelligent context from codebases, designed for the LLM era.*

[Features](#features) • [Quick Start](#quick-start) • [Server API](#server-api) • [Chrome Extension](#chrome-extension) • [CLI Tools](#cli-tools)

</div>

## ✨ Features

### 🖥️ **Server Mode**
- **RESTful API** for project context management
- **Multi-project support** with persistent configuration
- **API key authentication** for secure access
- **Smart file filtering** with extension and pattern-based exclusions
- **Intelligent content concatenation** with duplicate detection and categorization

### 🌐 **Chrome Extension**
- **Beautiful, intuitive UI** for seamless LLM workflow integration
- **One-click context copying** directly to clipboard
- **Interactive file selection** from project metadata
- **Direct text input** for immediate LLM prompting
- **Configurable server connections** (local or remote)

### 💻 **CLI Tools**
- **Project management** commands for adding/removing projects
- **API key generation** and management
- **Server configuration** (ports, addresses)
- **Direct file gathering** for scripting workflows

### 🧠 **LLM-Optimized**
- **Structured output** with file metadata and categorization
- **Binary file detection** and automatic exclusion
- **Content deduplication** based on file hashes
- **Consistent file ordering** for reproducible context
- **Gitignore integration** for smart exclusions

## 🚀 Quick Start

### 1. Install & Build
```bash
git clone https://github.com/hyperb1iss/contexter.git
cd contexter/server
cargo build --release
```

### 2. Configure Your First Project
```bash
# Generate an API key
./target/release/contexter config generate-key main

# Add a project
./target/release/contexter config add-project my-project /path/to/project

# Start the server
./target/release/contexter server
```

### 3. Install Chrome Extension
1. Open Chrome and navigate to `chrome://extensions/`
2. Enable "Developer mode"
3. Click "Load unpacked" and select the `chrome-extension` folder
4. Configure your API key and server settings in the extension

### 4. Start Gathering Context!
- Click the Contexter extension icon
- Select your project
- Choose specific files or gather the entire project
- Copy context directly to clipboard for your LLM

## 🔧 Server API

The Contexter server provides a clean REST API for programmatic access:

### Authentication
All endpoints require an `X-API-Key` header:
```bash
curl -H "X-API-Key: your_api_key_here" http://localhost:3030/api/v1/projects
```

### Endpoints
- `GET /api/v1/projects` - List all configured projects
- `GET /api/v1/projects/{name}` - Get project metadata and file listing
- `POST /api/v1/projects/{name}` - Generate context for project or specific paths

See [SERVER.md](server/SERVER.md) for complete API documentation.

## 🌐 Chrome Extension

The Chrome extension provides a beautiful interface for common LLM workflows:

### Key Features
- **Project browser** with file tree visualization
- **Smart file selection** with preview
- **One-click copying** to clipboard
- **Direct LLM integration** with popular platforms
- **Custom server configuration** for team setups

See [EXTENSION.md](chrome-extension/EXTENSION.md) for detailed usage instructions.

## 💻 CLI Tools

### Project Management
```bash
# Add projects
contexter config add-project web-app ~/code/my-web-app
contexter config add-project api ~/code/my-api

# List configuration
contexter config list

# Remove projects
contexter config remove-project old-project
```

### API Key Management
```bash
# Generate keys for different users/tools
contexter config generate-key alice
contexter config generate-key ci-system
contexter config generate-key extension

# List and remove keys
contexter config list-keys
contexter config remove-key old-key
```

### Server Configuration
```bash
# Configure server settings
contexter config set-port 8080
contexter config set-address 0.0.0.0  # For remote access

# Start server with options
contexter server --verbose  # Debug mode
contexter server --quiet    # Minimal output
```

### Direct File Gathering
```bash
# Traditional CLI mode for scripts
contexter gather /path/to/project --extensions rs toml --ignore ".*test.*"
```

## 🔧 Configuration

Contexter stores configuration in your system's config directory:
- **macOS**: `~/Library/Application Support/contexter/config.json`
- **Linux**: `~/.config/contexter/config.json`
- **Windows**: `%APPDATA%\contexter\config.json`

Example configuration:
```json
{
  "projects": {
    "my-project": "/path/to/project"
  },
  "port": 3030,
  "listen_address": "127.0.0.1",
  "api_keys": {
    "main": "hashed_api_key_here"
  }
}
```

## 🎯 Use Cases

### For Individual Developers
- **Code reviews**: Generate context for entire features or modules
- **LLM prompting**: Get properly formatted codebase context
- **Documentation**: Create comprehensive project overviews

### For Teams
- **Shared context server**: Central context service for team LLM workflows
- **Code onboarding**: New team members can quickly understand project structure
- **Architecture discussions**: Generate context for specific subsystems

### For CI/CD
- **Automated documentation**: Generate context for documentation tools
- **Code analysis**: Feed context to automated analysis systems
- **Release notes**: Generate context for change summaries

## 🤝 Contributing

Contributions welcome! This project is actively used and maintained. Please see our [CONTRIBUTING.md](CONTRIBUTING.md) file for details.

## 📄 License

Distributed under the Apache 2.0 License. See `LICENSE` for more information.

---

<div align="center">

🐛 [Report Bug](https://github.com/hyperb1iss/contexter/issues) • 💡 [Request Feature](https://github.com/hyperb1iss/contexter/issues)

</div>

---

<div align="center">

Created by [Stefanie Jane 🌠](https://github.com/hyperb1iss)

If you find this project useful, [buy me a Monster Ultra Violet](https://ko-fi.com/hyperb1iss)! ⚡️

</div>

[license-shield]: https://img.shields.io/github/license/hyperb1iss/contexter.svg
