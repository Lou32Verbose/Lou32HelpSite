---
title: winget Package Management Reference
slug: /cli-tools/winget/package-management-reference/
summary: Reference for common `winget` package-management tasks, return-code lookups, and related client setup notes.
topic: cli-tools/winget
type: reference
tags: [winget, packages, windows, cli]
aliases: [microsoft winget client, winget return codes, creating and uploading a winget package]
platforms: [windows]
related:
  - /powershell/profiles/console-and-profile-customization/
status: published
updated: 2026-03-20
---

## Synopsis

`winget` covers package search, install, upgrade, export, and manifest workflows, and it often appears alongside PowerShell profile helpers or Windows setup notes.

## Syntax

```text
winget search <query>
winget install --id <package-id>
winget upgrade --all
winget export -o packages.json
```

## Parameters/Flags

- `search`: look up package identifiers
- `install --id`: install a specific package ID directly
- `upgrade --all`: upgrade everything with available updates
- `export -o`: save the current package list

## Examples

Search and install a package:

```text
winget search powertoys
winget install --id Microsoft.PowerToys
```

Export the current package set:

```text
winget export -o packages.json
```

Import a saved package set:

```text
winget import -i packages.json
```

## Related

- [`PowerShell Console And Profile Customization`](/powershell/profiles/console-and-profile-customization/)
- [`Windows Install And OOBE Notes`](/windows/install/windows-install-and-oobe-notes/)
