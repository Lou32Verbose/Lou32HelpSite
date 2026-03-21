---
title: Windows Registry Monitoring Reference
slug: /security/windows/registry-monitoring-reference/
summary: Defensive reference for Windows registry areas that are commonly checked during persistence and startup audits.
topic: security/windows
type: reference
tags: [security, windows, registry, monitoring, persistence]
aliases: [key registry areas to monitor, registry values to monitor for malware]
platforms: [windows]
related:
  - /security/search/defensive-search-audit-reference/
status: published
updated: 2026-03-20
---

## Synopsis

Use this draft as a defensive checklist when auditing Windows startup and persistence locations. It is meant for monitoring systems you administer, not for offensive use.

## Syntax

```text
HKLM\Software\...
HKCU\Software\...
HKEY_CLASSES_ROOT\CLSID
```

## Parameters/Flags

- `Run` and `RunOnce`: common startup keys
- `Services`: service and driver startup definitions
- `Winlogon`: shell and user-init behavior
- `Image File Execution Options`: debugger and image-hijack behavior

## Examples

Areas worth reviewing during a defensive audit:

```text
HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Run
HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run
HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services
HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon
HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options
```

Watch for:

- executables in unexpected paths
- startup entries that do not match installed software
- new services, CLSIDs, or proxy-related settings with no clear origin

## Related

- [`Defensive Search Audit Reference`](/security/search/defensive-search-audit-reference/)
