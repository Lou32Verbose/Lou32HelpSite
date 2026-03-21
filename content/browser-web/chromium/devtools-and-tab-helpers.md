---
title: DevTools And Tab Helpers
slug: /browser-web/chromium/devtools-and-tab-helpers/
summary: Quick Chromium and Edge DevTools helpers for inspecting pages, collecting open-tab URLs, and jumping to useful browser debugging resources.
topic: browser-web/chromium
type: reference
tags: [browser, chromium, devtools, edge, tabs]
aliases: [copy urls of all tabs in chrome, devtools secrets]
platforms: [browser, chromium]
related:
  - /browser-web/search/search-operator-reference/
status: published
updated: 2026-03-20
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

Useful DevTools areas to revisit:

- Console filtering and live expressions
- Snippets and overrides
- Edge DevTools integration for VS Code

## Related

- [`Search Operator Reference`](/browser-web/search/search-operator-reference/)
- [`Bookmarklet Essentials`](/browser-web/bookmarklets/essentials/)
