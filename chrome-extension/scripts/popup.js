console.log("Popup script started");
console.log("jQuery version:", $.fn.jquery);
console.log("jsTree version:", $.fn.jstree.version);

$(document).ready(async function () {
  console.log("Document ready, initializing...");

  const projectList = $("#projects");
  const projectMetadata = $("#project-metadata");
  const projectName = $("#project-name");
  const projectPath = $("#project-path");
  const fileTree = $("#file-tree");
  const fetchContentBtn = $("#fetch-content");
  const copyContentBtn = $("#copy-content");
  const contentDisplay = $("#content-display");
  const statusMessage = $("#status-message");
  const collapsibleToggle = $(".collapsible-toggle");
  const collapsibleContent = $(".collapsible-content");

  let selectedFiles = new Set();
  let allFiles = [];
  let currentProject = null;

  function showStatus(message, isError = false) {
    console.log(`Status: ${message} (${isError ? "Error" : "Success"})`);
    statusMessage.text(message).css("color", isError ? "red" : "green");
    setTimeout(() => statusMessage.text(""), 3000);
  }

  async function fetchProjects() {
    try {
      const { apiKey, serverUrl } = await chrome.storage.sync.get([
        "apiKey",
        "serverUrl",
      ]);
      console.log("API Key:", apiKey ? "Set" : "Not set");
      console.log("Server URL:", serverUrl);

      if (!apiKey || !serverUrl) {
        throw new Error("API Key or Server URL is missing");
      }

      const response = await fetch(`${serverUrl}/api/v1/projects`, {
        headers: { "X-API-Key": apiKey },
      });
      if (!response.ok)
        throw new Error(
          `Failed to fetch projects: ${response.status} ${response.statusText}`
        );
      const data = await response.json();
      console.log("Fetched projects:", data.projects);
      return data.projects;
    } catch (error) {
      console.error("Error fetching projects:", error);
      showStatus("Error fetching projects: " + error.message, true);
      return [];
    }
  }

  async function fetchProjectMetadata(projectName) {
    try {
      console.log("Fetching metadata for project:", projectName);
      const { apiKey, serverUrl } = await chrome.storage.sync.get([
        "apiKey",
        "serverUrl",
      ]);
      const response = await fetch(
        `${serverUrl}/api/v1/projects/${projectName}`,
        {
          headers: { "X-API-Key": apiKey },
        }
      );
      if (!response.ok)
        throw new Error(
          `Failed to fetch project metadata: ${response.status} ${response.statusText}`
        );
      const metadata = await response.json();
      console.log("Fetched metadata:", metadata);
      return metadata;
    } catch (error) {
      console.error("Error fetching project metadata:", error);
      showStatus("Error fetching project metadata: " + error.message, true);
      return null;
    }
  }

  async function fetchProjectContent(projectName, selectedFiles, allFiles) {
    try {
      console.log("Fetching content for project:", projectName);
      const { apiKey, serverUrl } = await chrome.storage.sync.get([
        "apiKey",
        "serverUrl",
      ]);

      let paths;
      if (selectedFiles.size === allFiles.length) {
        console.log("All files selected, sending empty list");
        paths = [];
      } else {
        console.log("Sending list of selected files");
        paths = Array.from(selectedFiles);
      }

      console.log("Paths to fetch:", paths);

      const response = await fetch(
        `${serverUrl}/api/v1/projects/${projectName}`,
        {
          method: "POST",
          headers: {
            "X-API-Key": apiKey,
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ paths }),
        }
      );
      if (!response.ok)
        throw new Error(
          `Failed to fetch project content: ${response.status} ${response.statusText}`
        );
      const data = await response.json();
      console.log("Fetched content:", data.content ? "Received" : "Empty");
      return data.content;
    } catch (error) {
      console.error("Error fetching project content:", error);
      showStatus("Error fetching project content: " + error.message, true);
      return null;
    }
  }

  function createJsTreeData(files) {
    console.log("Creating jsTree data structure");
    const tree = [];
    const paths = {};

    files.forEach((file) => {
      const parts = file.split("/");
      let currentPath = "";
      parts.forEach((part, index) => {
        const isLast = index === parts.length - 1;
        currentPath += (currentPath ? "/" : "") + part;
        if (!paths[currentPath]) {
          const node = {
            id: currentPath,
            text: part,
            children: isLast ? false : [],
            icon: isLast ? "jstree-file" : "jstree-folder",
            state: {
              selected: true,
              opened: false, // Ensure all nodes start closed
            },
          };
          if (index === 0) {
            tree.push(node);
          } else {
            paths[
              currentPath.substring(0, currentPath.lastIndexOf("/"))
            ].children.push(node);
          }
          paths[currentPath] = node;
        }
      });
    });

    console.log("jsTree data structure created:", tree);
    return tree;
  }

  function selectAllNodes(treeElement) {
    console.log("Selecting all nodes");
    treeElement.jstree("check_all");
    selectedFiles = new Set(allFiles);
    console.log("All nodes selected, selectedFiles:", selectedFiles);
  }

  collapsibleToggle.on("click", function () {
    console.log("Toggling file tree visibility");
    collapsibleContent.toggleClass("open");
    collapsibleToggle.text(
      collapsibleContent.hasClass("open") ? "Files ▲" : "Files ▼"
    );
  });

  try {
    console.log("Fetching projects...");
    const projects = await fetchProjects();
    console.log("Projects received:", projects);

    if (projects.length === 0) {
      console.log("No projects available");
      projectList.append("<li>No projects available</li>");
    } else {
      projects.forEach((project) => {
        console.log("Adding project to list:", project.name);
        const li = $("<li>")
          .text(project.name)
          .on("click", async function () {
            console.log("Project clicked:", project.name);
            const metadata = await fetchProjectMetadata(project.name);
            if (metadata) {
              currentProject = metadata.name;
              projectName.text(metadata.name);
              projectPath.text(metadata.path);
              allFiles = metadata.files;
              selectedFiles = new Set(allFiles);

              console.log("Initializing jsTree");
              fileTree.jstree("destroy");
              fileTree
                .jstree({
                  core: {
                    data: createJsTreeData(allFiles),
                    themes: {
                      name: "default",
                      dots: false,
                      icons: true,
                    },
                    expand_selected_onload: false, // Prevent auto-expanding selected nodes
                  },
                  plugins: ["checkbox", "wholerow"],
                  checkbox: {
                    three_state: true,
                    whole_node: false,
                    tie_selection: false,
                  },
                })
                .on("ready.jstree", function (e, data) {
                  selectAllNodes($(this));
                  $(this).jstree("close_all"); // Ensure all nodes are closed after initialization
                });

              projectMetadata.show();
              collapsibleContent.removeClass("open");
              collapsibleToggle.text("Files ▼");
            }
          });
        projectList.append(li);
      });
    }
  } catch (error) {
    console.error("Error in main execution:", error);
    showStatus("Error loading projects: " + error.message, true);
  }

  fileTree.on("check_node.jstree uncheck_node.jstree", function (e, data) {
    const node = data.node;
    const isChecked = node.state.checked;

    function updateSelectedFiles(nodeId, isSelected) {
      if (isSelected) {
        selectedFiles.add(nodeId);
      } else {
        selectedFiles.delete(nodeId);
      }

      // If it's a folder, update all child files
      const childNodes = fileTree.jstree(true).get_node(nodeId).children_d;
      childNodes.forEach((childId) => {
        if (allFiles.includes(childId)) {
          // Only add if it's a file, not a folder
          if (isSelected) {
            selectedFiles.add(childId);
          } else {
            selectedFiles.delete(childId);
          }
        }
      });
    }

    updateSelectedFiles(node.id, isChecked);
    console.log("Selected files:", selectedFiles);
  });

  fetchContentBtn.on("click", async function () {
    if (currentProject) {
      console.log("Fetching content for project:", currentProject);
      const content = await fetchProjectContent(
        currentProject,
        selectedFiles,
        allFiles
      );
      if (content) {
        contentDisplay.val(content).show();
        showStatus("Content fetched successfully");
      }
    } else {
      console.log("No project selected");
      showStatus("Please select a project first", true);
    }
  });

  copyContentBtn.on("click", async function () {
    console.log("Fetching content before copying to clipboard");
    if (currentProject) {
      const content = await fetchProjectContent(currentProject, selectedFiles, allFiles);
      if (content) {
        contentDisplay.val(content).show();
        contentDisplay.select();
        document.execCommand("copy");
        showStatus("Content fetched and copied to clipboard");
      }
    } else {
      console.log("No project selected");
      showStatus("Please select a project first", true);
    }
  });

  console.log("Popup script initialization complete");
});
