function fetchVersionAndShowBanner() {
  const manifest = chrome.runtime.getManifest();
  const version = manifest.version;
  showStartupBanner(version);
}

function showStartupBanner(version) {
  console.log(
    `%c 🚀✨🌟 Contexter v${version} launched! 🌠🛸🌈 `,
    "background: linear-gradient(90deg, #000033 0%, #0033cc 50%, #6600cc 100%); color: #00ffff; font-weight: bold; padding: 5px 10px; border-radius: 5px; text-shadow: 0 0 5px #fff, 0 0 10px #fff, 0 0 15px #fff, 0 0 20px #00ffff, 0 0 35px #00ffff, 0 0 40px #00ffff, 0 0 50px #00ffff, 0 0 75px #00ffff;"
  );
}

console.debug("Popup script started");
console.debug("jQuery version:", $.fn.jquery);
console.debug("jsTree version:", $.fn.jstree.version);

document.addEventListener("DOMContentLoaded", () => {
  document.body.style.width = "800px";
  chrome.storage.sync.get("darkMode", (result) => {
    setDarkMode(result.darkMode);
  });
});

$(document).ready(async function () {
  console.debug("Document ready, initializing...");

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
  const loadingIndicator = $("<div>")
    .attr("id", "loading-indicator")
    .text("Loading...");

  let selectedFiles = new Set();
  let allFiles = [];
  let currentProject = null;

  function showStatus(message, isError = false) {
    console.log(`Status: ${message} (${isError ? "Error" : "Success"})`);
    statusMessage.text(message).css("color", isError ? "red" : "green");
    setTimeout(() => statusMessage.text(""), 3000);
  }

  function showLoading() {
    $("body").append(loadingIndicator);
  }

  function hideLoading() {
    loadingIndicator.remove();
  }

  async function fetchProjects() {
    showLoading();
    try {
      const { apiKey, serverUrl } = await chrome.storage.sync.get([
        "apiKey",
        "serverUrl",
      ]);
      console.debug("API Key:", apiKey ? "Set" : "Not set");
      console.debug("Server URL:", serverUrl);

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
      console.debug("Fetched projects:", data.projects);
      return data.projects;
    } catch (error) {
      console.error("Error fetching projects:", error);
      showStatus("Error fetching projects: " + error.message, true);
      return [];
    } finally {
      hideLoading();
    }
  }

  async function fetchProjectMetadata(projectName) {
    showLoading();
    try {
      console.debug("Fetching metadata for project:", projectName);
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
      console.debug("Fetched metadata:", metadata);
      return metadata;
    } catch (error) {
      console.error("Error fetching project metadata:", error);
      showStatus("Error fetching project metadata: " + error.message, true);
      return null;
    } finally {
      hideLoading();
    }
  }

  async function fetchProjectContent(projectName, selectedFiles, allFiles) {
    showLoading();
    try {
      console.debug("Fetching content for project:", projectName);
      const { apiKey, serverUrl } = await chrome.storage.sync.get([
        "apiKey",
        "serverUrl",
      ]);

      let paths;
      if (selectedFiles.size === allFiles.length) {
        console.debug("All files selected, sending empty list");
        paths = [];
      } else {
        console.debug("Sending list of selected files");
        paths = Array.from(selectedFiles);
      }

      console.debug("Paths to fetch:", paths);

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
      console.debug("Fetched content:", data.content ? "Received" : "Empty");
      return data.content;
    } catch (error) {
      console.error("Error fetching project content:", error);
      showStatus("Error fetching project content: " + error.message, true);
      return null;
    } finally {
      hideLoading();
    }
  }

  function createJsTreeData(files) {
    console.debug("Creating jsTree data structure");
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
              selected: false,
              checked: true,
              opened: false,
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

    console.debug("jsTree data structure created:", tree);
    return tree;
  }

  function selectAllNodes(treeElement) {
    console.debug("Selecting all nodes");
    treeElement.jstree("check_all");
    selectedFiles = new Set(allFiles);
    console.debug("All nodes selected, selectedFiles:", selectedFiles);
  }

  function initializeJsTree(files) {
    console.debug("Initializing jsTree");
    fileTree.jstree("destroy");
    fileTree
      .jstree({
        core: {
          data: createJsTreeData(files),
          themes: {
            name: isDarkMode() ? "default-dark" : "default",
            dots: false,
            icons: true,
          },
          expand_selected_onload: false,
        },
        plugins: ["checkbox"],
        checkbox: {
          keep_selected_style: false,
          three_state: true,
          whole_node: false,
          tie_selection: false,
        },
      })
      .on("ready.jstree", function (e, data) {
        selectAllNodes($(this));
        $(this).jstree("close_all");
      });
  }

  function isDarkMode() {
    return document.body.classList.contains("dark-mode");
  }

  collapsibleToggle.on("click", function () {
    console.debug("Toggling file tree visibility");
    collapsibleContent.toggleClass("open");
    collapsibleToggle.text(
      collapsibleContent.hasClass("open") ? "Files ▲" : "Files ▼"
    );
  });

  try {
    console.debug("Fetching projects...");
    const projects = await fetchProjects();
    console.debug("Projects received:", projects);

    if (projects.length === 0) {
      console.debug("No projects available");
      projectList.append("<li>No projects available</li>");
    } else {
      projects.forEach((project) => {
        console.debug("Adding project to list:", project.name);
        const li = $("<li>")
          .text(project.name)
          .on("click", async function () {
            console.debug("Project clicked:", project.name);
            const metadata = await fetchProjectMetadata(project.name);
            if (metadata) {
              currentProject = metadata.name;
              projectName.text(metadata.name);
              projectPath.text(metadata.path);
              allFiles = metadata.files;
              selectedFiles = new Set(allFiles);

              initializeJsTree(allFiles);

              projectMetadata.show();
              collapsibleContent.removeClass("open");
              collapsibleToggle.text("Files ▼");
              fetchVersionAndShowBanner();
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

      const childNodes = fileTree.jstree(true).get_node(nodeId).children_d;
      childNodes.forEach((childId) => {
        if (allFiles.includes(childId)) {
          if (isSelected) {
            selectedFiles.add(childId);
          } else {
            selectedFiles.delete(childId);
          }
        }
      });
    }

    updateSelectedFiles(node.id, isChecked);
    console.debug("Selected files:", selectedFiles);
  });

  fetchContentBtn.on("click", async function () {
    if (currentProject) {
      console.debug("Fetching content for project:", currentProject);
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
      console.debug("No project selected");
      showStatus("Please select a project first", true);
    }
  });

  copyContentBtn.on("click", async function () {
    console.debug("Fetching content before copying to clipboard");
    if (currentProject) {
      const content = await fetchProjectContent(
        currentProject,
        selectedFiles,
        allFiles
      );
      if (content) {
        contentDisplay.val(content).show();
        contentDisplay.select();
        document.execCommand("copy");
        showStatus("Content fetched and copied to clipboard");
      }
    } else {
      console.debug("No project selected");
      showStatus("Please select a project first", true);
    }
  });

  // Add an event listener for theme changes
  document.addEventListener("themeChanged", function (e) {
    if (fileTree.jstree(true)) {
      fileTree.jstree(true).settings.core.themes.name = isDarkMode()
        ? "default-dark"
        : "default";
      fileTree.jstree(true).redraw(true);
    }
  });

  console.debug("Popup script initialization complete");
});
