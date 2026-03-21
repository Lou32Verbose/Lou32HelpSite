---
title: DevTools And Tab Helpers
slug: /browser-web/chromium/devtools-and-tab-helpers/
summary: Quick Chromium and Edge DevTools helpers for inspecting pages, collecting open-tab URLs, and jumping to useful browser debugging resources.
topic: browser-web/chromium
type: reference
tags: [browser, chromium, devtools, edge, tabs]
aliases: [copy urls of all tabs in chrome, devtools secrets, 7 developer tools secrets]
platforms: [browser, chromium]
related:
  - /browser-web/search/search-operator-reference/
status: published
updated: 2026-03-21
---

## Synopsis

This page keeps the lightweight browser-debugging helpers that are useful during inspection sessions, especially in Chromium-based browsers.

## Syntax

```text
chrome://inspect/#pages
Array.from(document.querySelectorAll(selector)).map(...)
```

## Parameters/Flags

- `chrome://inspect/#pages`: page that lists debuggable tabs
- `document.querySelectorAll(...)`: select DOM nodes inside the inspected page
- `map(...).join("\n")`: turn selected values into copyable text output

## Examples

Copy open-tab URLs from Chromium DevTools:

```javascript
Array.from(document.querySelectorAll('#pages-list .row .url'))
  .map((node) => node.innerText)
  .join('\n');
```

### DevTools Resources

**Console:**

- Console filtering — filter by log level, text, or regex
- Live expressions — pin and auto-evaluate JS expressions in the console
- Console.log enhancements — styled output, tables, grouping
- Browser debugging and Console integration in VS Code

**Snippets and Overrides:**

- Snippets — save and run reusable JS scripts from the Sources panel
- Overrides — persist local changes to network resources across reloads

**Edge DevTools for VS Code:**

- Extension: `ms-edgedevtools.vscode-edge-devtools`
- Provides embedded browser preview, Elements panel, and Network inspection inside VS Code
- CSS Mirror Editing — changes in VS Code reflect in DevTools and vice versa

## Related

- [`Search Operator Reference`](/browser-web/search/search-operator-reference/)
- [`Bookmarklet Essentials`](/browser-web/bookmarklets/essentials/)
- [`Edge Keyboard Shortcuts`](/browser-web/chromium/edge-keyboard-shortcuts/)
