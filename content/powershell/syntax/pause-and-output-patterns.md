---
title: PowerShell Pause And Output Patterns
slug: /powershell/syntax/pause-and-output-patterns/
summary: Reference for pausing scripts, printing readable console output, and choosing the right prompt style for the current host.
topic: powershell/syntax
type: reference
tags: [powershell, console, read-host, write-host, rawui]
aliases: [powershell pause methods, write-host formatting, break line in write-host]
platforms: [windows, powershell]
related:
  - /powershell/profiles/console-and-profile-customization/
status: published
updated: 2026-03-20
---

## Synopsis

Use different pause and console-output patterns depending on whether the script runs in a real console, needs cross-platform behavior, or only needs a simple Enter prompt.

## Syntax

```powershell
$null = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown')
[void][System.Console]::ReadKey($true)
Read-Host -Prompt 'Press ENTER to continue'
Write-Host "Line one`nLine two"
```

## Parameters/Flags

- `NoEcho`: do not print the key that was pressed
- `IncludeKeyDown`: trigger on key-down instead of waiting for key-up
- `$true` in `ReadKey($true)`: suppress console echo
- `` `n ``: newline escape sequence for `Write-Host`

## Examples

Pause for any key in a console host:

```powershell
Write-Host -NoNewLine "Press any key to continue..."
$null = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown')
```

Pause in a cross-platform PowerShell session:

```powershell
Write-Host -NoNewLine "Press any key to continue..."
[void][System.Console]::ReadKey($true)
```

Use a simple Enter-only prompt that works in more hosts:

```powershell
Read-Host -Prompt "Press ENTER to exit"
```

Print multiple lines with light formatting:

```powershell
Write-Host "Name: $env:USERNAME`nHome: $env:USERPROFILE"
```

## Related

- [`PowerShell Console And Profile Customization`](/powershell/profiles/console-and-profile-customization/)
- [`PowerShell System Inspection Patterns`](/powershell/querying/system-inspection-patterns/)
