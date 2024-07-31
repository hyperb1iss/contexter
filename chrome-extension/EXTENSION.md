# Contexter Chrome Extension Documentation

The Contexter Chrome Extension is designed to interact with the Contexter server to fetch project contents and facilitate seamless integration with Language Learning Models (LLMs). This document outlines the key features and functionalities of the extension.

## Features

### 1. Beautiful UI

- **Intuitive Design**: The extension features a clean and intuitive user interface, ensuring a smooth user experience.
- **Responsive Layout**: The UI is responsive and adapts to different screen sizes, making it accessible on various devices.

### 2. List Configured Projects

- **Project Listing**: Users can view a list of configured projects directly from the extension's popup.
- **Project Details**: Each project entry displays basic metadata such as project name and path.

### 3. Show Project Metadata

- **Metadata Display**: Users can select a project to view detailed metadata, including a list of files and directories.
- **Interactive Elements**: The metadata display includes interactive elements to facilitate file selection.

### 4. Get Project Contents and Copy to Clipboard or Enter Directly into a Text Box

- **Fetch Project Contents**: Users can fetch the contents of a selected project or specific files.
- **Copy to Clipboard**: The fetched contents can be copied to the clipboard with a single click.
- **Direct Input**: Users can also directly input the fetched contents into a text box for further processing or integration with LLMs.

### 5. Allow the User to Select Paths / Files from the Project Metadata to Run On

- **File Selection**: Users can select specific files or directories from the project metadata to fetch their contents.
- **Custom Queries**: The extension allows for custom queries to fetch specific parts of the project.

### 6. Choose Which API Keys to Use

- **API Key Management**: Users can manage and select which API keys to use for authentication with the Contexter server.
- **Secure Storage**: API keys are securely stored and can be easily switched within the extension.

### 7. Select Server Hostname and Port (Default localhost:3030)

- **Server Configuration**: Users can configure the hostname and port of the Contexter server within the extension settings.
- **Default Settings**: The default server configuration is set to `localhost:3030`, but users can customize this to connect to different instances of the server.

## Usage

### Installation

1. **Download the Extension**: Obtain the extension package from the official distribution channel.
2. **Load the Extension**: Open Chrome and go to `chrome://extensions/`, enable "Developer mode", and click "Load unpacked" to load the extension.

### Configuration

1. **API Keys**: Navigate to the extension settings to add and manage API keys.
2. **Server Settings**: Configure the server hostname and port in the extension settings.

### Fetching Project Contents

1. **Open the Extension**: Click on the extension icon in the Chrome toolbar to open the popup.
2. **Select a Project**: Choose a project from the list to view its metadata.
3. **Select Files/Paths**: Select specific files or directories from the metadata.
4. **Fetch Contents**: Click the "Fetch" button to retrieve the contents.
5. **Copy or Input**: Copy the contents to the clipboard or input them directly into a text box.
