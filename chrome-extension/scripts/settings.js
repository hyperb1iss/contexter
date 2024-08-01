document.addEventListener("DOMContentLoaded", function () {
  console.log("DOM Content Loaded");

  const apiKeyInput = document.getElementById("api-key");
  const serverUrlInput = document.getElementById("server-url");
  const themeSelect = document.getElementById("theme-select");
  const saveSettingsButton = document.getElementById("save-settings");
  const validateApiKeyButton = document.getElementById("validate-api-key");
  const versionSpan = document.getElementById("extension-version");

  // Load and display extension version
  const manifest = chrome.runtime.getManifest();
  console.log("Manifest loaded:", manifest);
  versionSpan.textContent = manifest.version;

  // Load saved settings
  chrome.storage.sync.get(["apiKey", "serverUrl", "theme"], function (result) {
    console.log("Loaded settings:", result);
    if (result.apiKey) {
      apiKeyInput.value = result.apiKey;
    }
    if (result.serverUrl) {
      serverUrlInput.value = result.serverUrl;
    }
    if (result.theme) {
      themeSelect.value = result.theme;
    }

    // Initialize theme
    applyTheme(result.theme || "system");
  });

  // Save settings
  saveSettingsButton.addEventListener("click", function () {
    const apiKey = apiKeyInput.value;
    const serverUrl = serverUrlInput.value;
    const theme = themeSelect.value;

    console.log("Saving settings. Theme:", theme);

    chrome.storage.sync.set({ apiKey, serverUrl, theme }, function () {
      console.log("Settings saved");
      showStatusMessage("Settings saved successfully", false);
      applyTheme(theme);
    });
  });

  // Validate API Key
  validateApiKeyButton.addEventListener("click", async function () {
    const apiKey = apiKeyInput.value;
    const serverUrl = serverUrlInput.value;

    if (!apiKey || !serverUrl) {
      showStatusMessage("Please enter both API Key and Server URL", true);
      return;
    }

    try {
      const response = await fetch(`${serverUrl}/api/v1/projects`, {
        method: "GET",
        headers: {
          "X-API-Key": apiKey,
          "Content-Type": "application/json",
        },
        mode: "cors",
      });

      if (response.ok) {
        showStatusMessage("API Key is valid", false);
      } else {
        showStatusMessage("API Key is invalid", true);
      }
    } catch (error) {
      console.error("Error:", error);
      showStatusMessage("Failed to validate API Key", true);
    }
  });

  // Theme change handler
  themeSelect.addEventListener("change", function () {
    const theme = this.value;
    console.log("Theme changed to:", theme);
    applyTheme(theme);
  });

  function showStatusMessage(message, isError) {
    const statusMessage = document.createElement("div");
    statusMessage.textContent = message;
    statusMessage.style.color = isError ? "red" : "green";
    statusMessage.style.marginTop = "10px";

    document.body.appendChild(statusMessage);

    setTimeout(function () {
      statusMessage.remove();
    }, 3000);
  }

  function applyTheme(theme) {
    console.log("applyTheme called with:", theme);
    if (theme === "system") {
      const systemTheme = window.matchMedia("(prefers-color-scheme: dark)")
        .matches
        ? "dark"
        : "light";
      setTheme(systemTheme);
    } else {
      setTheme(theme);
    }
  }

  function setTheme(theme) {
    document.body.classList.toggle("dark-mode", theme === "dark");
    document.body.classList.toggle("light-mode", theme === "light");

    document
      .querySelectorAll(".container, input, button, select")
      .forEach((el) => {
        el.classList.toggle("dark-mode", theme === "dark");
        el.classList.toggle("light-mode", theme === "light");
      });

    document.querySelectorAll("h1, h2, h3, h4").forEach((el) => {
      el.classList.toggle("dark-mode", theme === "dark");
    });
  }

  // Listen for system theme changes
  window.matchMedia("(prefers-color-scheme: dark)").addListener(function (e) {
    chrome.storage.sync.get("theme", function (result) {
      if (result.theme === "system") {
        applyTheme("system");
      }
    });
  });
});
