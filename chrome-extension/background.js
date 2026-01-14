// Background service worker for Jira Issue Helper
// Handles API requests to localhost to bypass CORS/Private Network Access restrictions

const SERVER_URL = 'http://localhost:8887';

// Handle messages from content script
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.type === 'API_REQUEST') {
    handleApiRequest(request)
      .then(sendResponse)
      .catch(error => sendResponse({ error: error.message }));
    return true; // Keep the message channel open for async response
  }
});

async function handleApiRequest(request) {
  const { endpoint, method, body } = request;
  
  try {
    const options = {
      method: method || 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    };
    
    if (body) {
      options.body = JSON.stringify(body);
    }
    
    const response = await fetch(`${SERVER_URL}${endpoint}`, options);
    const data = await response.json();
    return { success: true, data };
  } catch (error) {
    console.error('[Background] API request failed:', error);
    return { success: false, error: error.message };
  }
}
