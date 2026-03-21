---
title: PowerShell Console And Profile Customization
slug: /powershell/profiles/console-and-profile-customization/
summary: Reusable profile snippets for completion behavior, transcript logging, history size, startup module setup, aliases, winget tab completion, and Write-Host formatting.
topic: powershell/profiles
type: template
tags: [powershell, profile, psreadline, transcript, modules, aliases, winget, write-host]
aliases: [psprofile customizations, powershell transcript example, get-module examples, import alias csv, winget tab completion, install-and-import]
platforms: [windows, powershell]
related:
  - /powershell/syntax/pause-and-output-patterns/
status: published
updated: 2026-03-21
---

## Use Case

Keep one PowerShell profile with the console behaviors you want on every session instead of retyping setup commands by hand.

## Template

```powershell
# Show a menu when completing with Tab.
Set-PSReadLineKeyHandler -Key Tab -Function MenuComplete

# Keep more history in interactive sessions.
$MaximumHistoryCount = 10000
Set-PSReadLineOption -MaximumHistoryCount 10000

# Auto-import common modules when they are installed.
$preferredModules = @("PowerShellGet", "Microsoft.WinGet.Client")
foreach ($module in $preferredModules) {
    if (Get-Module -ListAvailable -Name $module) {
        Import-Module $module -ErrorAction SilentlyContinue
    }
}

# Optional transcript logging.
$logRoot = Join-Path $env:USERPROFILE "Documents\PowerShell\Transcripts"
if (-not (Test-Path $logRoot)) {
    New-Item -ItemType Directory -Path $logRoot | Out-Null
}
$stamp = Get-Date -Format "yyyyMMdd_HHmmss"
Start-Transcript -Path (Join-Path $logRoot "session_$stamp.txt")
```

## Variables

- `$PROFILE`: current host-specific startup script path
- `$MaximumHistoryCount`: interactive command history size
- `$preferredModules`: modules to import only when present
- `$logRoot`: folder used for transcript output

## Examples

Create or edit your current user profile:

```powershell
if (-not (Test-Path $PROFILE)) {
    New-Item -ItemType File -Path $PROFILE -Force | Out-Null
}
notepad $PROFILE
```

Check which modules are available before adding imports:

```powershell
Get-Module -ListAvailable | Sort-Object Name
Get-Module -ListAvailable PowerShellGet, Microsoft.WinGet.Client
```

Group all modules by name with detailed paths:

```powershell
Get-Module -ListAvailable -All | Format-Table -Property Name, Moduletype, Path -Groupby Name
```

Get all property names of loaded modules:

```powershell
Get-Module | Get-Member -MemberType Property | Format-Table Name
```

Stop transcript logging before exiting:

```powershell
Stop-Transcript
```

### Winget Tab Completion

Register the winget argument completer in your profile so Tab auto-completes winget commands and package names:

```powershell
Register-ArgumentCompleter -Native -CommandName winget -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)
        [Console]::InputEncoding = [Console]::OutputEncoding = $OutputEncoding = [System.Text.Utf8Encoding]::new()
        $Local:word = $wordToComplete.Replace('"', '""')
        $Local:ast = $commandAst.ToString().Replace('"', '""')
        winget complete --word="$Local:word" --commandline "$Local:ast" --position $cursorPosition | ForEach-Object {
            [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
        }
}
```

### Install-And-Import Function

A profile function that auto-installs missing modules from PSGallery before importing them:

```powershell
function install-and-import {
    Param(
        [Parameter(Mandatory=$true)]
        [string]$moduleName,
        [string]$Scope = 'CurrentUser'
    )
    $oldErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Stop"

    Write-Host "Importing $moduleName"
    if ( -not $( Get-Module -Name $moduleName -ListAvailable ) ) {
        Write-Host "`tModule $moduleName is not installed. Installing now with scope ${Scope}..."
        Install-Module -Force -AllowClobber -Scope $Scope $moduleName
    }

    Import-Module $moduleName
    $ErrorActionPreference = $oldErrorActionPreference
}

# Call the function for each module you want available:
install-and-import PSWindowsUpdate
install-and-import ComputerCleanup
install-and-import pswinglue
install-and-import AudioDeviceCmdlets
install-and-import -Scope AllUsers Microsoft.Winget.Client
```

### Uninstall Old Module Versions

Find the specific version of an installed module and remove older versions (useful for cleaning up PowerShellGet duplicates):

```powershell
$ModuleName = 'YourModuleName';
$Latest = Get-InstalledModule $ModuleName;
Get-InstalledModule $ModuleName -AllVersions | Sort-Object -Descending {[Version]$_.Version} | Select-Object -Skip 1 | Uninstall-Module -WhatIf
```

Remove the `-WhatIf` flag to actually uninstall old versions after confirming which will be removed.

### Import Aliases From CSV

Store aliases in a separate CSV file and load them from your profile:

```powershell
# Export an alias to your CSV alias file
$profileDir = Split-Path $PROFILE -Parent
$aliasFile = Join-Path $profileDir aliases.csv
New-Alias -Name npp -Value "C:\Program Files\Notepad++\notepad++.exe" -Description "Notepad++"
Export-Alias -Name npp -Path $aliasFile -Append

# Then in your profile, import all aliases from the CSV
Import-Alias -Path (Join-Path $PSScriptRoot aliases.csv)
```

### Alias And Function Examples

Define aliases and navigation functions directly in your profile:

```powershell
Set-Alias -Name notepad -Value C:\Windows\notepad.exe
notepad

Function CD32 {Set-Location -Path C:\Windows\System32}
Set-Alias -Name Go -Value CD32
```

### Transcript Logging (CurrentUserAllHosts)

An alternative transcript logging approach that writes to a TranscriptLog subfolder next to your profile, using a trap to silently handle errors:

```powershell
# Add to $profile.CurrentUserAllHosts
Function StartTranscript {
    Trap {
        Continue
    }
    $TranScriptFolder = $($(Split-Path $profile) + '\TranscriptLog\')
    if (!(Test-Path -Path $TranScriptFolder )) { New-Item -ItemType directory -Path $TranScriptFolder }
    Start-Transcript -Append ($($TranScriptFolder + $(get-date -format 'yyyyMMdd-HHmmss') + '.txt')) -ErrorVariable Transcript -ErrorAction stop
}
StartTranscript
```

Basic transcript example (start, run commands, stop):

```powershell
Start-Transcript -Path "C:\transcript.txt"

# Your commands here
Get-Process
Get-Service

Stop-Transcript
```

### Write-Host Custom Text Formatting

Common parameters for Write-Host formatting:

- `-ForegroundColor <ConsoleColor>` -- sets text color
- `-BackgroundColor <ConsoleColor>` -- sets background color
- `-NoNewline` -- outputs text without a trailing newline
- `-Separator <string>` -- custom separator when writing multiple objects

Supported ConsoleColor values: `Black`, `DarkBlue`, `DarkGreen`, `DarkCyan`, `DarkRed`, `DarkMagenta`, `DarkYellow`, `Gray`, `DarkGray`, `Blue`, `Green`, `Cyan`, `Red`, `Magenta`, `Yellow`, `White`

```powershell
# Basic colored warning message
Write-Host 'Warning: Disk space low!' `
  -ForegroundColor Yellow `
  -BackgroundColor DarkRed

# Label and value with custom separator
Write-Host 'User:' $env:USERNAME `
  -ForegroundColor Cyan `
  -Separator ' '

# Progress indicator without newline
for ($i = 1; $i -le 5; $i++) {
    Write-Host '.' -NoNewline -ForegroundColor Green
    Start-Sleep -Milliseconds 200
}
Write-Host ' Done.' -ForegroundColor Green
```

ANSI escape sequences for true RGB color (requires Windows Terminal or compatible host):

```powershell
$esc = [char]27
Write-Host "${esc}[38;2;255;165;0mCustom Orange Text${esc}[0m"

# Reset to default after ANSI colored output
Write-Host 'Back to normal colors'
```

## Related

- [`PowerShell Pause And Output Patterns`](/powershell/syntax/pause-and-output-patterns/)
- [`PowerShell File And Text Recipes`](/powershell/filesystem/file-and-text-recipes/)
