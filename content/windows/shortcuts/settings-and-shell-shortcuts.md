---
title: Settings And Shell Shortcuts
slug: /windows/shortcuts/settings-and-shell-shortcuts/
summary: Reference for `ms-settings:` shortcuts, shell namespace commands, and quick launcher targets inside Windows.
topic: windows/shortcuts
type: reference
tags: [windows, shortcuts, shell, settings, explorer]
aliases: [ms-settings shortcuts, shell commands list, clsid shell locations]
platforms: [windows]
related:
  - /windows/install/windows-install-and-oobe-notes/
status: published
updated: 2026-03-20
---

## Synopsis

Windows supports both `ms-settings:` URIs and `shell:::{GUID}` targets, which makes it easy to create shortcuts for Settings pages, Control Panel items, and namespace folders.

## Syntax

```text
explorer.exe ms-settings:colors
shell:Downloads
shell:::{20D04FE0-3AEA-1069-A2D8-08002B30309D}
```

## Parameters/Flags

- `ms-settings:<page>`: opens a modern Settings page
- `shell:<name>`: opens a known shell folder alias
- `shell:::{GUID}`: opens a shell namespace object by CLSID

## Examples

Create a desktop shortcut for the Colors settings page:

```text
explorer.exe ms-settings:colors
```

Open common shell folders directly:

```text
shell:Downloads
shell:Startup
shell:AppData
```

Open classic Control Panel views with CLSIDs:

```text
shell:::{21EC2020-3AEA-1069-A2DD-08002B30309D}
shell:::{7007ACC7-3202-11D1-AAD2-00805FC1270E}
```

## Related

- [`Windows Install And OOBE Notes`](/windows/install/windows-install-and-oobe-notes/)
- [`Display Diagnostics`](/windows/display/display-diagnostics/)
