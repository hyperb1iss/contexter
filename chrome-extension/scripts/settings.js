document.getElementById('save-settings').addEventListener('click', () => {
    const apiKey = document.getElementById('api-key').value;
    const serverUrl = document.getElementById('server-url').value;
  
    chrome.storage.sync.set({ apiKey, serverUrl }, () => {
      console.log('Settings saved');
    });
  });
  
  document.getElementById('validate-api-key').addEventListener('click', async () => {
    const apiKey = document.getElementById('api-key').value;
    const serverUrl = document.getElementById('server-url').value;
  
    try {
      const response = await fetch(`${serverUrl}/api/v1/projects`, {
        method: 'GET',
        headers: {
          'X-API-Key': apiKey,
          'Content-Type': 'application/json'
        },
        mode: 'cors'
      });
  
      if (response.ok) {
        alert('API Key is valid');
      } else {
        alert('API Key is invalid');
      }
    } catch (error) {
      console.error('Error:', error);
      alert('Failed to validate API Key');
    }
  });
  
  // Load saved settings
  chrome.storage.sync.get(['apiKey', 'serverUrl'], (result) => {
    if (result.apiKey) {
      document.getElementById('api-key').value = result.apiKey;
    }
    if (result.serverUrl) {
      document.getElementById('server-url').value = result.serverUrl;
    }
  });