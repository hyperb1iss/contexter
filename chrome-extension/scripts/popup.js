document.addEventListener('DOMContentLoaded', async () => {
  const projectList = document.getElementById('projects');
  const projectMetadata = document.getElementById('project-metadata');
  const projectName = document.getElementById('project-name');
  const projectPath = document.getElementById('project-path');
  const fileTree = document.getElementById('file-tree');
  const contentActions = document.getElementById('content-actions');
  const fetchContentBtn = document.getElementById('fetch-content');
  const copyContentBtn = document.getElementById('copy-content');
  const contentDisplay = document.getElementById('content-display');
  const statusMessage = document.getElementById('status-message');
  const collapsibleToggle = document.querySelector('.collapsible-toggle');
  const collapsibleContent = document.querySelector('.collapsible-content');

  let selectedFiles = new Set();
  let allFiles = [];

  function showStatus(message, isError = false) {
      statusMessage.textContent = message;
      statusMessage.style.color = isError ? 'red' : 'green';
      setTimeout(() => {
          statusMessage.textContent = '';
      }, 3000);
  }

  async function fetchProjects() {
      try {
          const { apiKey, serverUrl } = await chrome.storage.sync.get(['apiKey', 'serverUrl']);
          const response = await fetch(`${serverUrl}/api/v1/projects`, {
              headers: { 'X-API-Key': apiKey }
          });
          if (!response.ok) throw new Error('Failed to fetch projects');
          const data = await response.json();
          return data.projects;
      } catch (error) {
          console.error('Error fetching projects:', error);
          showStatus('Error fetching projects', true);
          return [];
      }
  }

  async function fetchProjectMetadata(projectName) {
      try {
          const { apiKey, serverUrl } = await chrome.storage.sync.get(['apiKey', 'serverUrl']);
          const response = await fetch(`${serverUrl}/api/v1/projects/${projectName}`, {
              headers: { 'X-API-Key': apiKey }
          });
          if (!response.ok) throw new Error('Failed to fetch project metadata');
          return await response.json();
      } catch (error) {
          console.error('Error fetching project metadata:', error);
          showStatus('Error fetching project metadata', true);
          return null;
      }
  }

  async function fetchProjectContent(projectName, paths) {
      try {
          const { apiKey, serverUrl } = await chrome.storage.sync.get(['apiKey', 'serverUrl']);
          const response = await fetch(`${serverUrl}/api/v1/projects/${projectName}`, {
              method: 'POST',
              headers: {
                  'X-API-Key': apiKey,
                  'Content-Type': 'application/json'
              },
              body: JSON.stringify({ paths: paths.length > 0 ? Array.from(paths) : undefined })
          });
          if (!response.ok) throw new Error('Failed to fetch project content');
          const data = await response.json();
          return data.content;
      } catch (error) {
          console.error('Error fetching project content:', error);
          showStatus('Error fetching project content', true);
          return null;
      }
  }

  function createFileTree(files) {
      const tree = {};
      files.forEach(file => {
          const parts = file.split('/');
          let currentLevel = tree;
          parts.forEach((part, index) => {
              if (!currentLevel[part]) {
                  currentLevel[part] = index === parts.length - 1 ? null : {};
              }
              currentLevel = currentLevel[part];
          });
      });
      return tree;
  }

  function renderFileTree(tree, parentElement, path = '') {
      for (const [name, subtree] of Object.entries(tree)) {
          const item = document.createElement('div');
          item.classList.add('file-tree-item');
          const fullPath = path ? `${path}/${name}` : name;

          const checkbox = document.createElement('input');
          checkbox.type = 'checkbox';
          checkbox.checked = true;
          checkbox.value = fullPath;

          const label = document.createElement('label');
          label.appendChild(checkbox);
          label.appendChild(document.createTextNode(name));
          item.appendChild(label);

          if (subtree === null) {
              // File
              selectedFiles.add(fullPath);
              checkbox.addEventListener('change', (e) => {
                  if (e.target.checked) {
                      selectedFiles.add(fullPath);
                  } else {
                      selectedFiles.delete(fullPath);
                  }
              });
          } else {
              // Folder
              item.classList.add('file-tree-folder');
              const folderContent = document.createElement('div');
              folderContent.classList.add('file-tree-folder-content');
              renderFileTree(subtree, folderContent, fullPath);
              item.appendChild(folderContent);

              label.addEventListener('click', (e) => {
                  if (e.target !== checkbox) {
                      e.preventDefault();
                      item.classList.toggle('open');
                      folderContent.classList.toggle('open');
                  }
              });

              checkbox.addEventListener('change', (e) => {
                  const isChecked = e.target.checked;
                  folderContent.querySelectorAll('input[type="checkbox"]').forEach(cb => {
                      cb.checked = isChecked;
                      cb.dispatchEvent(new Event('change'));
                  });
              });
          }

          parentElement.appendChild(item);
      }
  }

  collapsibleToggle.addEventListener('click', () => {
      collapsibleContent.classList.toggle('open');
      collapsibleToggle.textContent = collapsibleContent.classList.contains('open') ? 'Files ▲' : 'Files ▼';
  });

  const projects = await fetchProjects();
  projects.forEach(project => {
      const li = document.createElement('li');
      li.textContent = project.name;
      li.addEventListener('click', async () => {
          const metadata = await fetchProjectMetadata(project.name);
          if (metadata) {
              projectName.textContent = metadata.name;
              projectPath.textContent = metadata.path;
              fileTree.innerHTML = '';
              allFiles = metadata.files;
              selectedFiles = new Set(allFiles);

              const tree = createFileTree(allFiles);
              renderFileTree(tree, fileTree);

              projectMetadata.style.display = 'block';
              contentActions.style.display = 'block';
              collapsibleContent.classList.remove('open');
              collapsibleToggle.textContent = 'Files ▼';
          }
      });
      projectList.appendChild(li);
  });

  fetchContentBtn.addEventListener('click', async () => {
      const projectNameText = projectName.textContent;
      if (projectNameText) {
          const content = await fetchProjectContent(projectNameText, selectedFiles);
          if (content) {
              contentDisplay.value = content;
              contentDisplay.style.display = 'block';
              showStatus('Content fetched successfully');
          }
      } else {
          showStatus('Please select a project first', true);
      }
  });

  copyContentBtn.addEventListener('click', () => {
      contentDisplay.select();
      document.execCommand('copy');
      showStatus('Content copied to clipboard');
  });
});