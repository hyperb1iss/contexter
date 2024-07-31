document.getElementById('fetch-project').addEventListener('click', async () => {
  chrome.storage.sync.get(['apiKey', 'serverUrl'], async (result) => {
    const apiKey = result.apiKey;
    const serverUrl = result.serverUrl || 'http://localhost:3030';
    const projectName = 'your_project_name_here';

    try {
      const response = await fetch(`${serverUrl}/api/v1/projects/${projectName}`, {
        method: 'POST',
        headers: {
          'X-API-Key': apiKey,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({}),
        mode: 'cors'
      });

      if (!response.ok) {
        throw new Error('Failed to fetch project contents');
      }

      const data = await response.json();
      console.log('Project Contents:', data.content);
      // Handle the project contents as needed (e.g., copy to clipboard, send to LLM)
    } catch (error) {
      console.error('Error:', error);
    }
  });
});