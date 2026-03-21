---
title: Search Operator Reference
slug: /browser-web/search/search-operator-reference/
summary: General-purpose search operators for refining web searches, targeting specific sites, and narrowing results by title, URL, or file type.
topic: browser-web/search
type: reference
tags: [search, browser, google, operators, web]
aliases: [google search operators reference, search filters]
platforms: [browser]
related:
  - /browser-web/chromium/devtools-and-tab-helpers/
status: published
updated: 2026-03-20
---

## Synopsis

Search operators help you narrow results to a site, file type, URL pattern, title, or date range without needing a complex toolchain.

## Syntax

```text
site:example.com keyword
filetype:pdf report
intitle:"keyword"
inurl:"keyword"
```

## Parameters/Flags

- `site:`: limit results to one host or domain
- `filetype:`: limit results to one document type
- `intitle:`: require a term in the page title
- `inurl:`: require a term in the URL

## Examples

Search only one site:

```text
site:example.com troubleshooting
```

Find PDF documents on a domain:

```text
site:example.com filetype:pdf guide
```

Combine title and URL filters:

```text
intitle:"index of" inurl:downloads
```

Use date filters when supported by the search engine:

```text
filetype:pdf before:2025-01-01 after:2024-01-01 release notes
```

## Related

- [`DevTools And Tab Helpers`](/browser-web/chromium/devtools-and-tab-helpers/)
