# 🗺️ Repository Mapper Documentation

The Contexter Repository Mapper provides intelligent analysis of code structure, dependencies, and relationships using state-of-the-art techniques including topological sorting for dependency-aware processing.

## Overview

Based on research from [DocAgent](https://arxiv.org/abs/2504.08725) and other state-of-the-art tools, the repository mapper implements:

- **Dependency Graph Construction**: AST parsing to identify components and relationships
- **Topological Processing Order**: Dependencies-first processing using Kahn's algorithm  
- **Intelligent Context Generation**: Rich repository insights for LLMs
- **Clean Repository Maps**: Professional text-based visualization of project structure

## Quick Start

The repository mapper is designed around **one simple command** that gives you everything you need:

```bash
# Analyze current directory - shows structure, insights, and key components
contexter map

# Analyze a specific project  
contexter map /path/to/project

# Focus on a specific component
contexter map --focus "my_function"

# Show processing order for LLM context building
contexter map --order

# Show dependency relationships  
contexter map --dependencies

# Save results to file
contexter map --output analysis.txt

# Get JSON output for programmatic use
contexter map --json
```

## CLI Usage

### Basic Repository Map

```bash
contexter map [PATH] [OPTIONS]
```

**Arguments:**
- `PATH`: Repository path to analyze (default: current directory)

**Options:**
- `--dependencies, -d`: Show dependency relationships
- `--order, -o`: Show topological processing order  
- `--output FILE`: Save to file
- `--json, -j`: JSON format output
- `--focus COMPONENT`: Focus on specific component

### Examples

**Simple Analysis**:
```bash
$ contexter map
Repository Map
==============

📍 .
📊 68 components, 1 entry points, 0 cycles

Structure:
  ./src/main.rs
    pub fn main (2→0)
    pub fn setup_logging (0→1)

  ./src/lib.rs  
    pub struct Config (0→3)
    pub fn load_config (1→2)

Entry Points:
  setup_logging (./src/main.rs)

Key Components:
  1. Config (./src/lib.rs)
  2. load_config (./src/lib.rs)
```

**Focus on Component**:
```bash
$ contexter map --focus "handle_gather"
🎯 Focus: handle_gather
   Type: Function | Visibility: Public
   File: ./src/cli_handlers.rs
   Dependencies: 2 | Used by: 1

Dependencies:
  → gather_relevant_files (./src/contexter.rs)
  → concatenate_files (./src/contexter.rs)

Used by:
  ← run_cli (./src/cli.rs)
```

**Processing Order**:
```bash
$ contexter map --order
Processing Order (dependencies first):
  1. gather_relevant_files
  2. concatenate_files  
  3. handle_gather
  4. Config
  5. load_config
  ... and 63 more
```

**JSON Output**:
```bash
$ contexter map --json
{
  "repository": ".",
  "components": 68,
  "entry_points": ["setup_logging"],
  "cycles": 0,
  "most_connected": ["Config", "load_config"],
  "processing_order": null
}
```

## Use Cases

### 🤖 LLM Context Generation

**Dependencies-First Processing**:
```bash
# Get processing order for incremental context building
contexter map --order --json > processing-order.json
```

Use the topological order to process components with proper context:
1. Process dependencies first
2. Include dependency context when analyzing a component
3. Build comprehensive understanding progressively

**Component Analysis**:
```bash
# Get detailed info about a specific component
contexter map --focus "my_function" --dependencies
```

### 📚 Code Documentation

**Project Overview**:
```bash
# Generate comprehensive project overview
contexter map --output project-structure.md
```

**Architecture Analysis**:
```bash
# Understand component relationships
contexter map --dependencies
```

### 🔍 Code Review & Analysis

**Impact Analysis**:
```bash
# Focus on a component you're changing to see its relationships
contexter map --focus "my_component"
```

**Dependency Understanding**:
```bash
# See processing order to understand build dependencies
contexter map --order
```

## Advanced Usage

### Integration with Development Workflow

**Pre-commit Analysis**:
```bash
# Quick check before committing
contexter map | grep "cycles: [1-9]" && echo "Warning: Dependency cycles detected!"
```

**Documentation Generation**:
```bash
# Generate architecture documentation
contexter map --output docs/architecture.md
echo "# Project Architecture" | cat - docs/architecture.md > temp && mv temp docs/architecture.md
```

### LLM Integration

**Progressive Context Building**:
```python
import json
import subprocess

# Get processing order
result = subprocess.run(['contexter', 'map', '--order', '--json'], 
                       capture_output=True, text=True)
order = json.loads(result.stdout)['processing_order']

# Process components in dependency order
for component in order:
    # Get component details
    result = subprocess.run(['contexter', 'map', '--focus', component, '--json'],
                           capture_output=True, text=True)
    component_info = json.loads(result.stdout)
    
    # Build context and send to LLM
    context = build_context_with_dependencies(component_info)
    llm_result = analyze_with_llm(context)
```

**Component Discovery**:
```bash
# Find all entry points for analysis
contexter map --json | jq '.entry_points[]'

# Find most connected components
contexter map --json | jq '.most_connected[]'
```

## Configuration

The repository mapper uses the same configuration as the main Contexter tool:

```bash
# Add project for easier access
contexter config add-project my-project /path/to/project

# Then analyze by name (if project is configured in server mode)
curl -X GET "http://localhost:3030/api/v1/projects/my-project/map" \
     -H "X-API-Key: your_api_key"
```

## Technical Details

### Supported Languages

- **Rust**: Functions, structs, impls, use statements
- **Python**: Functions, classes, imports  
- **JavaScript/TypeScript**: Functions, classes, imports, exports

### Dependency Analysis

**Relationship Types**:
- Function calls
- Module imports  
- Class inheritance
- Field access

**Graph Construction**:
- AST parsing for accurate component extraction
- Topological sorting using Kahn's algorithm
- Cycle detection and handling

### Performance

**Speed**: Optimized for fast analysis
- 10-100 files: < 1 second
- 100-1000 files: < 10 seconds  
- 1000+ files: < 60 seconds

**Memory**: Linear scaling with repository size
- ~1MB per 100 components

## Output Format

### Text Format (Default)

Clean, professional output with:
- Repository overview (components, entry points, cycles)
- File-by-file structure breakdown
- Component visibility and connection counts  
- Key insights (entry points, most connected)
- Optional: processing order, dependencies

### JSON Format

Structured data including:
- Component metadata
- Dependency relationships
- Processing order
- Focus component details (if specified)

Example:
```json
{
  "repository": "/path/to/project",
  "components": 45,
  "entry_points": ["main", "setup"],
  "cycles": 0,
  "most_connected": ["Config", "Utils"],
  "processing_order": ["Utils", "Config", "main"],
  "dependency_graph": [
    {"from": "main", "to": "Config", "edge_type": "FunctionCall"}
  ]
}
```

## Limitations & Future Work

**Current Limitations**:
- Simplified AST parsing (regex-based)  
- Limited cross-language dependency analysis
- Basic complexity scoring

**Planned Improvements**:
- Tree-sitter based AST parsing
- Advanced architectural pattern detection
- Integration with git history for change impact
- ML-based component importance scoring
- Visual graph generation