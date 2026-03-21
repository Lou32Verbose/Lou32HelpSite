---
title: PowerShell Script Templates And Patterns
slug: /powershell/syntax/script-templates-and-patterns/
summary: Reusable PowerShell script patterns for admin checks, transcripts, elevated execution, module management, WinGet sandbox install, and data conversion.
topic: powershell/syntax
type: template
tags: [powershell, scripts, admin-check, transcript, elevation, winget, csv]
aliases: [checking for admin rights, test-iswindowsterminal breakdown, powershell transcript template, open elevated cli, install winget on sandbox, uninstalling powershellget, convert tab to comma csv function]
platforms: [windows, powershell]
related:
  - /powershell/syntax/cmdlet-patterns-and-filtering/
  - /powershell/profiles/console-and-profile-customization/
status: published
updated: 2026-03-21
---

## Use Case

Reusable script patterns and templates for common PowerShell tasks: checking admin privileges, session transcription, running elevated scripts, module management, and data format conversion.

## Template

```powershell
if (-not ([bool](New-Object Security.Principal.WindowsPrincipal(
    [Security.Principal.WindowsIdentity]::GetCurrent()
)).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator))) {
    Write-Error "This script must be run as an administrator."
    exit 1
}
```

## Variables

- `[Security.Principal.WindowsPrincipal]`: .NET class for checking Windows roles
- `Start-Transcript`: begins recording console session to file
- `-ExecutionPolicy Bypass`: overrides script execution restrictions
- `-Verb runAs`: launches a process with elevation

## Examples

### Check for Administrator Privileges

Insert at the top of any script that requires elevation:

```powershell
if (-not ([bool](New-Object Security.Principal.WindowsPrincipal(
    [Security.Principal.WindowsIdentity]::GetCurrent()
)).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator))) {
    Write-Error "This script must be run as an administrator."
    exit 1
}
```

### Detect Windows Terminal (Test-IsWindowsTerminal)

Function that walks the process tree to determine if the current session runs inside Windows Terminal:

```powershell
function Test-IsWindowsTerminal {
    [CmdletBinding()]
    param ()

    if ($PSVersionTable.PSVersion.Major -le 5 -or $IsWindows -eq $true) {
        $currentPid = $PID

        while ($currentPid) {
            try {
                $process = Get-CimInstance Win32_Process -Filter "ProcessId = $currentPid" `
                    -ErrorAction Stop -Verbose:$false
            } catch {
                return $false
            }

            Write-Verbose "ProcessName: $($process.Name), Id: $($process.ProcessId), ParentId: $($process.ParentProcessId)"

            if ($process.Name -eq 'WindowsTerminal.exe') {
                return $true
            } else {
                $currentPid = $process.ParentProcessId
            }
        }
        return $false
    } else {
        Write-Verbose 'Exiting due to non-Windows environment'
        return $false
    }
}
```

Uses `Get-CimInstance Win32_Process` instead of `Get-Process` because Windows PowerShell's `Get-Process` does not expose parent process info.

### Session Transcript Template

```powershell
Start-Transcript -Path "C:\transcript.txt"

# Your commands here
Get-Process
Get-Service

Stop-Transcript
```

### Launch Elevated PowerShell Script (One-Liner)

Run a script with elevation without modifying the system execution policy:

```powershell
powershell "Start-Process powershell -Verb Runas -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File C:\path\script.ps1'"
```

### Install WinGet on Windows Sandbox

Confirmed working script for bootstrapping WinGet in Windows Sandbox:

```powershell
$progressPreference = 'silentlyContinue'
Write-Host "Installing WinGet PowerShell module from PSGallery..."
Install-PackageProvider -Name NuGet -Force | Out-Null
Install-Module -Name Microsoft.WinGet.Client -Force -Repository PSGallery | Out-Null
Write-Host "Using Repair-WinGetPackageManager cmdlet to bootstrap WinGet..."
Repair-WinGetPackageManager
Write-Host "Done."
```

### Uninstall Old Module Versions (PowerShellGet)

Find and remove all but the latest version of a module:

```powershell
$ModuleName = 'PowerShellGet'
$Latest = Get-InstalledModule $ModuleName
Get-InstalledModule $ModuleName -AllVersions |
  Sort-Object -Descending { [Version]$_.Version } |
  Select-Object -Skip 1 |
  Uninstall-Module -WhatIf
```

Remove `-WhatIf` to actually uninstall.

### Convert Tab-Delimited CSV to Comma-Delimited

```powershell
Import-Csv -Path yourfile.csv -Delimiter "`t" -Encoding unicode |
  Export-Csv -Path yourfileoutput.csv -Delimiter ',' -Encoding utf8BOM
```

> **Note:** For Windows PowerShell (not Core), change `utf8BOM` to `UTF8` and add `-NoTypeInformation`.

Change the delimiter character to any other (e.g., `'|'`) as needed.

## Related

- [`PowerShell Cmdlet Patterns And Filtering`](/powershell/syntax/cmdlet-patterns-and-filtering/)
- [`PowerShell Console And Profile Customization`](/powershell/profiles/console-and-profile-customization/)
