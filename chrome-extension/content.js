// Content script for Jira Issue Helper
// Runs on https://smma-sqe.atlassian.net

const TARGET_SELECTOR = '[data-testid="issue.views.issue-base.foundation.breadcrumbs.breadcrumb-current-issue-container"]';
const ISSUE_LINK_SELECTOR = '[data-testid="issue.views.issue-base.foundation.breadcrumbs.current-issue.item"]';
const CUSTOM_DIV_ID = 'jira-issue-helper-div';
const SERVER_URL = 'http://localhost:8889';

let eventSource = null;
let currentStatus = null;

// Get the issue number from the breadcrumb
function getIssueNumber() {
  const issueLink = document.querySelector(ISSUE_LINK_SELECTOR);
  if (issueLink) {
    const span = issueLink.querySelector('span');
    if (span) {
      return span.textContent.trim();
    }
  }
  return null;
}

// Start voting session
async function startVoting() {
  const issueNumber = getIssueNumber();
  
  try {
    const response = await fetch(`${SERVER_URL}/api/start-voting`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        issue_number: issueNumber || null,
      }),
    });
    
    const data = await response.json();
    console.log('[Jira Issue Helper] Start voting response:', data);
    updateButtonStates();
  } catch (error) {
    console.error('[Jira Issue Helper] Failed to start voting:', error);
    showError('Failed to connect to server');
  }
}

// Reveal votes
async function revealVotes() {
  try {
    const response = await fetch(`${SERVER_URL}/api/reveal`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    });
    
    const data = await response.json();
    console.log('[Jira Issue Helper] Reveal votes response:', data);
    showResults(data);
    updateButtonStates();
  } catch (error) {
    console.error('[Jira Issue Helper] Failed to reveal votes:', error);
    showError('Failed to connect to server');
  }
}

// Connect to SSE status stream
function connectToStatusStream() {
  if (eventSource) {
    eventSource.close();
  }

  eventSource = new EventSource(`${SERVER_URL}/api/status`);

  eventSource.onmessage = (event) => {
    try {
      currentStatus = JSON.parse(event.data);
      console.log('[Jira Issue Helper] Status update:', currentStatus);
      updateStatusDisplay();
      updateButtonStates();
    } catch (error) {
      console.error('[Jira Issue Helper] Failed to parse status:', error);
    }
  };

  eventSource.onerror = (error) => {
    console.error('[Jira Issue Helper] SSE connection error:', error);
    // Retry connection after 5 seconds
    setTimeout(connectToStatusStream, 5000);
  };
}

// Update the status display in the UI
function updateStatusDisplay() {
  const statusDiv = document.getElementById('poker-status-display');
  if (!statusDiv || !currentStatus) return;

  const { phase, connected_players, votes_cast, total_players, issue_number } = currentStatus;

  let statusHtml = `<strong>${total_players}</strong> connected`;
  
  if (phase === 'voting') {
    statusHtml += ` | Voting: <strong>${votes_cast}/${total_players}</strong>`;
    if (issue_number) {
      statusHtml += ` | Issue: ${issue_number}`;
    }
  } else if (phase === 'revealed') {
    statusHtml += ' | Votes revealed';
  }

  // Show player names
  if (connected_players && connected_players.length > 0) {
    const playerNames = connected_players.map(p => {
      const votedIndicator = p.has_voted ? '✓' : '';
      return `${p.name}${votedIndicator}`;
    }).join(', ');
    statusHtml += `<br><small>${playerNames}</small>`;
  }

  statusDiv.innerHTML = statusHtml;
}

// Update button states based on current phase
function updateButtonStates() {
  const startBtn = document.getElementById('poker-start-btn');
  const revealBtn = document.getElementById('poker-reveal-btn');
  
  if (!startBtn || !revealBtn || !currentStatus) return;

  const { phase } = currentStatus;

  if (phase === 'voting') {
    startBtn.disabled = true;
    startBtn.style.opacity = '0.5';
    revealBtn.disabled = false;
    revealBtn.style.opacity = '1';
  } else {
    startBtn.disabled = false;
    startBtn.style.opacity = '1';
    revealBtn.disabled = true;
    revealBtn.style.opacity = '0.5';
  }
}

// Show voting results
function showResults(data) {
  const resultsDiv = document.getElementById('poker-results-display');
  if (!resultsDiv) return;

  const { votes, statistics, issue_number } = data;

  let html = '';
  
  if (issue_number) {
    html += `<div style="margin-bottom: 4px;"><strong>Issue:</strong> ${issue_number}</div>`;
  }

  if (statistics.votes_cast > 0) {
    html += `<div style="margin-bottom: 4px;">`;
    html += `Avg: <strong>${statistics.average?.toFixed(1) || '-'}</strong> | `;
    html += `Median: <strong>${statistics.median?.toFixed(1) || '-'}</strong> | `;
    html += `Range: ${statistics.min || '-'}-${statistics.max || '-'}`;
    html += `</div>`;
  }

  html += '<div style="font-size: 11px;">';
  votes.forEach(v => {
    const voteText = v.vote !== null ? v.vote : 'No vote';
    html += `${v.player_name}: <strong>${voteText}</strong> | `;
  });
  html = html.slice(0, -3); // Remove trailing " | "
  html += '</div>';

  resultsDiv.innerHTML = html;
  resultsDiv.style.display = 'block';
}

// Show error message
function showError(message) {
  const resultsDiv = document.getElementById('poker-results-display');
  if (!resultsDiv) return;

  resultsDiv.innerHTML = `<span style="color: #ff5630;">${message}</span>`;
  resultsDiv.style.display = 'block';
}

// Create the custom UI
function addCustomDiv() {
  // Check if we already added the div
  if (document.getElementById(CUSTOM_DIV_ID)) {
    return;
  }

  const targetDiv = document.querySelector(TARGET_SELECTOR);
  
  if (targetDiv) {
    const customDiv = document.createElement('div');
    customDiv.id = CUSTOM_DIV_ID;
    customDiv.style.cssText = `
      display: inline-flex;
      flex-direction: column;
      align-items: flex-start;
      margin-left: 12px;
      padding: 8px 12px;
      background-color: #f4f5f7;
      border: 1px solid #dfe1e6;
      border-radius: 6px;
      font-size: 12px;
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    `;

    customDiv.innerHTML = `
      <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 6px;">
        <span style="font-weight: 600; color: #172b4d;">🃏 Poker Planning</span>
        <button id="poker-start-btn" style="
          padding: 4px 10px;
          background-color: #0052CC;
          color: white;
          border: none;
          border-radius: 3px;
          cursor: pointer;
          font-size: 11px;
          font-weight: 500;
        ">Start Vote</button>
        <button id="poker-reveal-btn" style="
          padding: 4px 10px;
          background-color: #00875A;
          color: white;
          border: none;
          border-radius: 3px;
          cursor: pointer;
          font-size: 11px;
          font-weight: 500;
          opacity: 0.5;
        " disabled>Reveal</button>
      </div>
      <div id="poker-status-display" style="color: #5e6c84; font-size: 11px;">
        Connecting to server...
      </div>
      <div id="poker-results-display" style="
        display: none;
        margin-top: 6px;
        padding: 6px 8px;
        background: white;
        border-radius: 3px;
        border: 1px solid #dfe1e6;
        color: #172b4d;
        font-size: 11px;
        max-width: 400px;
      "></div>
    `;
    
    // Insert the custom div right after the target div
    targetDiv.parentNode.insertBefore(customDiv, targetDiv.nextSibling);
    
    // Add event listeners
    document.getElementById('poker-start-btn').addEventListener('click', startVoting);
    document.getElementById('poker-reveal-btn').addEventListener('click', revealVotes);

    // Connect to status stream
    connectToStatusStream();
    
    console.log('[Jira Issue Helper] Custom div added successfully');
  }
}

// Use MutationObserver to handle dynamic content loading (Jira is a SPA)
function observeDOM() {
  const observer = new MutationObserver((mutations) => {
    addCustomDiv();
  });

  observer.observe(document.body, {
    childList: true,
    subtree: true
  });

  // Also try immediately in case the element is already there
  addCustomDiv();
}

// Start observing when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', observeDOM);
} else {
  observeDOM();
}

// Cleanup on page unload
window.addEventListener('beforeunload', () => {
  if (eventSource) {
    eventSource.close();
  }
});
