
# 🚀 Contexter

<div align="center">

[![License][license-shield]](LICENSE)

*A powerful command-line tool for gathering context from files, perfect for feeding into Language Models (LLMs).*

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [Contributing](#contributing) • [License](#license)

</div>

## ✨ Features

- 🗂️ **Directory Traversal:** Recursively walks through directories to gather files.
- 🔍 **Extension Filtering:** Includes files based on specified extensions.
- ❌ **Exclusion Patterns:** Excludes files matching specified regex patterns.
- 📋 **Clipboard Support:** Optionally copies the concatenated content to the clipboard.
- 🔄 **Duplicate Detection:** Skips duplicate file contents based on content hashes.
- 📑 **Consistent Output Order:** Ensures the output order of files is consistent.

## 🛠️ Installation
<a name="installation"></a>

### Prerequisites

- Rust and Cargo installed on your system.

### Building from Source

1. Clone the repository:
    ```bash
    git clone https://github.com/hyperb1iss/contexter.git
    cd contexter
    ```
2. Build the project:
    ```bash
    cargo build --release
    ```
3. The binary will be located in `target/release/contexter`.

## 🚀 Usage
<a name="usage"></a>

```bash
contexter [OPTIONS] <DIRECTORY> [EXTENSIONS]...
```

### Options

- `-c, --clipboard`  
  Copy the concatenated result to the clipboard.

- `-e, --exclude <PATTERN>`  
  Exclude filename patterns (supports regex).

### Examples

#### Basic Usage

To gather all files from a directory and print their contents to stdout:

```bash
contexter /path/to/directory
```

#### Filtering by Extensions

To include only `.rs` and `.toml` files:

```bash
contexter /path/to/directory rs toml
```

#### Excluding Patterns

To exclude files matching certain patterns:

```bash
contexter /path/to/directory --exclude ".*test.*" --exclude ".*ignore.*"
```

#### Copy to Clipboard

To copy the concatenated content to the clipboard:

```bash
contexter /path/to/directory -c
```

## Example Output

When running the following command:

```bash
contexter /path/to/directory rs -e ".*test.*"
```

You might get an output like this:

```plaintext
========================================
File: "/path/to/directory/src/main.rs"
========================================
fn main() {
    println!("Hello, world!");
}
========================================
File: "/path/to/directory/src/lib.rs"
========================================
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## 🤝 Contributing
<a name="contributing"></a>

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**. Please see our [CONTRIBUTING.md](CONTRIBUTING.md) file for details on how to get started.

## 📄 License
<a name="license"></a>

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
