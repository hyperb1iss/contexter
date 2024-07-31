document.getElementById('fetch-project').addEventListener('click', async () => {
  const apiKey = 'your_api_key_here';
  const projectName = 'your_project_name_here';
  const serverUrl = 'http://localhost:3030/api/v1/projects';

  try {
    const response = await fetch(`${serverUrl}/${projectName}`, {
      method: 'POST',
      headers: {
        'X-API-Key': apiKey,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({})
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