---
title: Defensive Search Audit Reference
slug: /security/search/defensive-search-audit-reference/
summary: Defensive guidance for using public search operators to audit accidental exposure of secrets or sensitive artifacts belonging to systems you own.
topic: security/search
type: reference
tags: [security, search, audit, github, google]
aliases: [github search dorks, google dork cheatsheet]
platforms: [browser]
related:
  - /security/windows/registry-monitoring-reference/
status: published
updated: 2026-03-20
---

## Synopsis

This draft reframes the legacy search-dork notes as defensive exposure checks for assets you own or are explicitly authorized to review.

## Syntax

```text
site:example.com filetype:pdf
filename:.env OR filename:.git-credentials
org:your-org path:/ ".npmrc"
```

## Parameters/Flags

- `site:`: scope a public-web audit to one domain
- `filename:`: search for a specific file name or extension
- `org:`: scope a GitHub search to one organization
- quoted strings: require exact matches for suspicious filenames or keys

## Examples

Check your own site for unexpectedly indexed documents:

```text
site:example.com filetype:pdf
site:example.com intitle:"index of"
```

Check your own GitHub organization for risky filenames:

```text
org:your-org filename:.env
org:your-org filename:.npmrc
org:your-org extension:pem private
```

Safe-use reminders:

- only search assets you own or are authorized to assess
- treat results as leads that still need manual confirmation
- rotate credentials immediately if a real exposure is confirmed

## Related

- [`Windows Registry Monitoring Reference`](/security/windows/registry-monitoring-reference/)
