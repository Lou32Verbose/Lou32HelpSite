---
title: Bookmarklet Essentials
slug: /browser-web/bookmarklets/essentials/
summary: Small JavaScript bookmarklets for inspecting and simplifying web pages.
topic: browser-web/bookmarklets
type: template
tags: [bookmarklets, javascript, browser]
aliases: [bookmarklet list, browser helpers]
platforms: [browser, javascript]
related:
  - /cli-tools/wget/recursive-download/
status: published
updated: 2026-03-20
---

## Use Case

Keep a short set of reusable browser bookmarklets for inspection, cleanup, and quick navigation.

## Template

```javascript
javascript:(function(){
  alert(document.title + "\n" + location.href);
})();
```

## Variables

- `document.title`: current page title
- `location.href`: current page URL
- `document.links`: collection of links on the page
- `document.images`: collection of images on the page

## Examples

Copy the current page URL:

```javascript
javascript:(function(){
  navigator.clipboard.writeText(location.href);
  alert("Copied URL");
})();
```

List all links on the page:

```javascript
javascript:(function(){
  var list = Array.from(document.links).map(function(link) { return link.href; }).join("\n");
  alert(list || "No links found");
})();
```

## Related

- [`Wget Recursive Download Reference`](/cli-tools/wget/recursive-download/)
