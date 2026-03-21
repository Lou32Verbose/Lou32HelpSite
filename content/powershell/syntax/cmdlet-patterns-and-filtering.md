---
title: PowerShell Cmdlet Patterns And Filtering
slug: /powershell/syntax/cmdlet-patterns-and-filtering/
summary: Reference for Where-Object filtering syntax, Copy-Item patterns, alias definitions, wildcard queries, and pwsh.dll execution policy passing.
topic: powershell/syntax
type: reference
tags: [powershell, where-object, copy-item, alias, wildcards, filtering]
aliases: [where-object example syntax, powershell wildcard template, copy-item reference, alias examples, pwsh dll command line, get all variables and values of cmdlet, get-command excluding windows sys folder]
platforms: [windows, powershell]
related:
  - /powershell/syntax/pause-and-output-patterns/
  - /powershell/querying/system-inspection-patterns/
status: published
updated: 2026-03-21
---

## Synopsis

Reference for common PowerShell filtering and data manipulation patterns including Where-Object syntax, Copy-Item operations, alias definitions, and execution policy passing.

## Syntax

```powershell
<command> | Where-Object { $_.<Property> -like '<pattern>' }
Copy-Item -Path "<source>" -Destination "<target>"
Set-Alias -Name <alias> -Value <target>
```

## Parameters/Flags

- `$_`: current pipeline object
- `-like`: wildcard pattern match (supports `*` and `?`)
- `-Recurse`: include subdirectories in Copy-Item
- `-Force`: overwrite existing files
- `-WhatIf`: preview what a command would do without executing
- `-Confirm`: prompt before executing
- `-Include` / `-Exclude`: filter by filename patterns

## Examples

### Where-Object Filtering

Query AppX provisioned packages with a display name starting with "Microsoft.":

```powershell
Get-AppxProvisionedPackage -Online |
  Where-Object { $_.DisplayName -like 'Microsoft.*' } |
  Format-Table DisplayName, PackageName
```

The syntax breakdown:
1. Wrap the condition in curly brackets `{ }`
2. Prefix the property with `$_.` (e.g., `$_.DisplayName`)
3. Place the wildcard inside the quotes (e.g., `'Microsoft.*'`)

### Wildcard Query Template

```powershell
Get-WindowsOptionalFeature -Online |
  Where-Object { $_.FeatureName -like "*<desired-value-here>*" }
```

### Copy-Item Patterns

**Basic file copy:**

```powershell
Copy-Item -Path "C:\Source\SpreadsheetOne.xlsx" -Destination "C:\Destination"
```

**Copy multiple files:**

```powershell
Copy-Item -Path "C:\Source\file1.txt","C:\Source\file2.txt" -Destination "D:\Destination"
```

**Copy with rename:**

```powershell
Copy-Item -Path "C:\Source\file1.txt" -Destination "C:\Destination\NewName1.txt"
```

**Copy folders with subdirectories:**

```powershell
Copy-Item -Path "C:\Source\folder" -Destination "D:\Destination" -Recurse
```

**Copy and merge multiple folders:**

```powershell
Copy-Item -Path "C:\source1\*","C:\source2\*" -Destination "D:\Destination"
```

**Filter with Include and Exclude:**

```powershell
Copy-Item -Path "C:\Source\*" -Destination "D:\Destination" -Include "*.log" -Exclude "*.tmp"
```

**Force overwrite:**

```powershell
Copy-Item -Path "C:\Source\file.txt" -Destination "D:\Destination" -Force
```

**Preview with WhatIf:**

```powershell
Copy-Item -Path "C:\Source\File1.txt" -Destination "C:\Destination" -WhatIf
```

### Alias Definitions

```powershell
Set-Alias -Name notepad -Value C:\Windows\notepad.exe

Function CD32 { Set-Location -Path C:\Windows\System32 }
Set-Alias -Name Go -Value CD32
```

### Passing Execution Policy to pwsh.dll

```text
"C:\Program Files\PowerShell\7-preview\pwsh.dll" -ExecutionPolicy Bypass -NoProfile
```

### Get All Parameter Keys and Values for a Cmdlet

Inspect every parameter a script or cmdlet accepts and display current values:

```powershell
(Get-Command -Name $PSCommandPath).Parameters |
  Format-Table -AutoSize @{
    Label = "Key"; Expression = { $_.Key }
  }, @{
    Label = "Value"; Expression = { (Get-Variable -Name $_.Key -EA SilentlyContinue).Value }
  }
```

### Get-Command Excluding System Binaries

List only non-Windows commands (useful for finding user-installed tools):

```powershell
Get-Command | Where-Object { $_.Source -notlike 'C:\Windows\*' }
```

## Related

- [`PowerShell Pause And Output Patterns`](/powershell/syntax/pause-and-output-patterns/)
- [`PowerShell System Inspection Patterns`](/powershell/querying/system-inspection-patterns/)
