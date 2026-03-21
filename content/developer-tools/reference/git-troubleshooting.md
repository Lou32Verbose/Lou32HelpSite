---
title: Git Troubleshooting
slug: /developer-tools/reference/git-troubleshooting/
summary: Fixes for git push hanging and other common git issues including timeout configuration and verbose debugging.
topic: developer-tools/reference
type: reference
tags: [git, troubleshooting, push, networking]
aliases: [troubleshooting git push hanging]
platforms: [windows, linux, macos]
related:
  - /developer-tools/reference/document-and-image-tools/
status: published
updated: 2026-03-21
---

## Synopsis

Quick fixes for common git issues, starting with push operations that hang indefinitely.

## Syntax

```bash
git -c http.lowSpeedLimit=1000 -c http.lowSpeedTime=10 push
```

## Parameters/Flags

- `http.lowSpeedLimit`: minimum bytes/second before considering the connection stalled
- `http.lowSpeedTime`: seconds to wait below the speed limit before aborting
- `GIT_TRACE`: environment variable that enables general git trace output
- `GIT_CURL_VERBOSE`: environment variable that enables verbose HTTP/curl logging

## Examples

### Git Push Hanging

Set a timeout so push fails fast instead of hanging forever:

```bash
git -c http.lowSpeedLimit=1000 -c http.lowSpeedTime=10 push
```

- `http.lowSpeedLimit=1000`: minimum bytes/second before considering the connection stalled
- `http.lowSpeedTime=10`: abort after 10 seconds below the speed limit

### Enable Verbose Debugging

In PowerShell, set environment variables before pushing to see detailed HTTP/curl output:

```powershell
$env:GIT_TRACE = "1"
$env:GIT_CURL_VERBOSE = "1"
git push
```

## Related

- [`Document And Image Tools`](/developer-tools/reference/document-and-image-tools/)
