# Chrome Web Store Publishing Guide

This document contains all the required information for publishing the Jira Issue Helper extension.

---

## Privacy Practices Tab

### Single Purpose Description

```
This extension adds poker planning voting controls to Jira issues, allowing teams to start voting sessions and reveal results directly from the Jira interface.
```

### Host Permission Justification

The extension requires the following host permission:
- `http://localhost:8887/*`

**Justification:**

```
This extension communicates with a locally-running Poker Planning server on localhost:8887. The server runs on the user's own machine and provides:
1. Starting voting sessions for agile story point estimation
2. Revealing voting results and statistics
3. Real-time status updates via Server-Sent Events (SSE)

No external servers are contacted. All communication is strictly between the browser extension and the user's local server instance.
```

### Remote Code Justification

**Does your extension use remote code?** No

**Justification:**

```
This extension does not use any remote code. All JavaScript code is bundled within the extension package:
- content.js: UI injection and server communication
- background.js: Service worker for extension lifecycle

No code is fetched from external servers, CDNs, or dynamically evaluated. All functionality is self-contained within the extension files.
```

### Data Usage Disclosure

**What user data does this extension collect?**

```
This extension does NOT collect, transmit, or store any personal user data.

Data handling:
- Jira issue numbers are read from the current page URL to identify voting sessions
- This data is only sent to localhost (user's own machine)
- No data is sent to external servers
- No analytics, tracking, or telemetry is implemented
- No user credentials or personal information is accessed
```

**Data Usage Certification Checklist:**

- [x] I do not sell user data to third parties
- [x] I do not use or transfer user data for purposes unrelated to the item's single purpose
- [x] I do not use or transfer user data to determine creditworthiness or for lending purposes

---

## Store Listing Tab

### Description (for store listing)

```
Jira Poker Planning Integration

Seamlessly integrate poker planning voting sessions directly into your Jira workflow.

Features:
• Start Vote - Begin a poker planning session for the current Jira issue
• Reveal Results - View voting statistics including average, median, and individual votes
• Live Status - Real-time updates showing connected players and voting progress

Requirements:
• Poker Planning CLI server running locally (available at github.com/your-repo)
• Access to your Jira instance

This extension is designed for agile teams using story point estimation. It connects to a local server that manages voting sessions, allowing team members to vote using terminal clients while the Scrum Master controls the session from Jira.

Privacy: This extension only communicates with localhost. No data is sent to external servers.
```

### Category

```
Productivity
```

### Language

```
English
```

---

## Account Tab

### Contact Email

You need to:
1. Enter your contact email on the Account tab
2. Click "Verify" and check your inbox
3. Click the verification link in the email

---

## Summary Checklist

Before publishing, ensure you have:

- [ ] Filled in Single Purpose Description
- [ ] Provided Host Permission Justification  
- [ ] Confirmed "No" for remote code use with justification
- [ ] Certified data usage compliance
- [ ] Entered contact email
- [ ] Verified contact email
- [ ] Uploaded at least 1 screenshot (1280x800 or 640x400)
- [ ] Uploaded the extension ZIP file

---

## Screenshots

You'll need at least one screenshot. Recommended content:
1. The extension widget visible on a Jira issue page
2. The voting in progress state
3. The results view with statistics

Screenshot dimensions: **1280x800** or **640x400** pixels
