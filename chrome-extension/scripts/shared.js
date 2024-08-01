console.log("shared.js loaded");

function setDarkMode(isDarkMode) {
  console.log("setDarkMode called with:", isDarkMode);
  applyDarkMode(isDarkMode);
  // Dispatch a custom event when the theme changes
  document.dispatchEvent(
    new CustomEvent("themeChanged", { detail: { isDarkMode: isDarkMode } })
  );
}

function applyDarkMode(isDarkMode) {
  console.log("applyDarkMode called with:", isDarkMode);
  document.body.classList.toggle("dark-mode", isDarkMode);
  document.body.classList.toggle("light-mode", !isDarkMode);

  document
    .querySelectorAll(
      ".container, input, button, select, #projects, #file-tree"
    )
    .forEach((el) => {
      el.classList.toggle("dark-mode", isDarkMode);
      el.classList.toggle("light-mode", !isDarkMode);
    });

  document.querySelectorAll("h1, h2, h3, h4").forEach((el) => {
    el.classList.toggle("dark-mode", isDarkMode);
  });
}

function initializeDarkMode() {
  console.log("initializeDarkMode called");
  chrome.storage.sync.get("darkMode", (result) => {
    console.log("Initializing dark mode with:", result.darkMode);
    applyDarkMode(result.darkMode !== undefined ? result.darkMode : false);
  });
}

// Attach functions to window object to make them globally accessible
window.setDarkMode = setDarkMode;
window.applyDarkMode = applyDarkMode;
window.initializeDarkMode = initializeDarkMode;

document.addEventListener("DOMContentLoaded", initializeDarkMode);
