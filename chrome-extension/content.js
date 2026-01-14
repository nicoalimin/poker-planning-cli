// Content script for Jira Issue Helper
// Runs on https://smma-sqe.atlassian.net

const TARGET_SELECTOR = '[data-testid="issue.views.issue-base.foundation.breadcrumbs.breadcrumb-current-issue-container"]';
const CUSTOM_DIV_ID = 'jira-issue-helper-div';

function addCustomDiv() {
  // Check if we already added the div
  if (document.getElementById(CUSTOM_DIV_ID)) {
    return;
  }

  const targetDiv = document.querySelector(TARGET_SELECTOR);
  
  if (targetDiv) {
    const customDiv = document.createElement('div');
    customDiv.id = CUSTOM_DIV_ID;
    customDiv.textContent = 'Custom Extension Content';
    customDiv.style.cssText = `
      display: inline-flex;
      align-items: center;
      margin-left: 8px;
      padding: 4px 8px;
      background-color: #0052CC;
      color: white;
      border-radius: 4px;
      font-size: 12px;
      font-weight: 500;
    `;
    
    // Insert the custom div right after the target div
    targetDiv.parentNode.insertBefore(customDiv, targetDiv.nextSibling);
    
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
