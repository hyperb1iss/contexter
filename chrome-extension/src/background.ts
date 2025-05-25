chrome.runtime.onInstalled.addListener(() => {
  console.log('🚀 Contexter extension installed and ready!')
})

// Handle any background tasks here in the future
chrome.action.onClicked.addListener((tab) => {
  // Optional: Handle extension icon clicks if needed
  console.log('Extension clicked on tab:', tab.url)
}) 