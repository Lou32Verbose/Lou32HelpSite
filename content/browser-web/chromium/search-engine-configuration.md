---
title: Browser Search Engine Configuration
slug: /browser-web/chromium/search-engine-configuration/
summary: Group Policy settings for Edge default search provider and manual steps to add Google search to ungoogled Chromium.
topic: browser-web/chromium
type: reference
tags: [edge, chromium, search-engine, group-policy, google]
aliases: [edge default search provider google gp settings, ungoogled chromium add google as search engine]
platforms: [browser, windows]
related:
  - /browser-web/chromium/devtools-and-tab-helpers/
  - /browser-web/chromium/edge-keyboard-shortcuts/
status: published
updated: 2026-03-21
---

## Synopsis

Reference for configuring the default search provider in Chromium-based browsers — via Group Policy for managed Edge deployments, and manually for ungoogled Chromium.

## Syntax

```text
Computer Configuration > Administrative Templates > Microsoft Edge > Default search provider
```

## Parameters/Flags

- `{searchTerms}`: placeholder replaced with the user's query
- `{google:baseURL}`: resolves to the appropriate Google domain
- `{google:RLZ}`: promotional tracking parameter
- `%s`: standard Chromium placeholder for search terms in custom engine URLs

## Examples

### Edge: Set Google as Default Search Provider via Group Policy

Open `gpmc.msc` and navigate to:

```text
Computer Configuration > Administrative Templates > Classic Administrative Templates (ADM) > Microsoft Edge > Default search provider
```

Configure the following policies:

| Policy | Value |
|--------|-------|
| **Enable the default search provider** | Enabled |
| **Default Search Provider Name** | `Google` |
| **Default Search Provider Search URL** | `{google:baseURL}search?q={searchTerms}&{google:RLZ}{google:originalQueryForSuggestion}{google:assistedQueryStats}{google:searchFieldtrialParameter}{google:searchClient}{google:sourceId}ie={inputEncoding}` |
| **Default Search Provider URL for Suggestions** | `{google:baseURL}complete/search?output=chrome&q={searchTerms}` |
| **Specifies the search-by-image feature** | `{google:baseURL}searchbyimage/upload` |
| **Parameters for an image URL that uses POST** | `encoded_image={google:imageThumbnail},image_url={google:imageURL},sbisrc={google:imageSearchSource},original_width={google:imageOriginalWidth},original_height={google:imageOriginalHeight}` |
| **Configure the new tab page search box experience** | Address bar |

### Ungoogled Chromium: Add Google as Search Engine

1. Open `chrome://settings/searchEngines`
2. Click **Add** next to "Other search engines"
3. Fill in:
   - **Search engine**: `Google`
   - **Keyword**: `google.com`
   - **URL with %s**: `https://www.google.com/search?q=%s`
   - **Suggestions URL with %s**: `https://www.google.com/complete/search?client=chrome&q=%s`
4. Click **Save**
5. Click the three-dot menu on the new entry and select **Make default**

## Related

- [`DevTools And Tab Helpers`](/browser-web/chromium/devtools-and-tab-helpers/)
- [`Edge Keyboard Shortcuts`](/browser-web/chromium/edge-keyboard-shortcuts/)
