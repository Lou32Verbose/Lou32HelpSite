---
title: PowerShell Service And System Administration
slug: /powershell/querying/service-and-system-admin/
summary: PowerShell commands for managing services, listing system files, checking for malware, viewing diagnostic data, decoding registry hex values, and system shutdown.
topic: powershell/querying
type: reference
tags: [powershell, services, get-service, diagnostics, registry, shutdown, msc]
aliases: [manage system services powershell, list all msc files in windir, check file infect malware, diagnostic data viewer, decode registry hex, two powershell commands daily, full system shutdown, count saved folder views]
platforms: [windows, powershell]
related:
  - /powershell/querying/system-inspection-patterns/
  - /powershell/syntax/cmdlet-patterns-and-filtering/
status: published
updated: 2026-03-21
---

## Synopsis

PowerShell commands and patterns for service management, system file inspection, malware detection, diagnostic data viewing, registry value decoding, and quick administration tasks.

## Syntax

```powershell
Get-Service | Where-Object { $_.Status -eq 'Running' }
Get-ChildItem -Path C:\Windows\System32\* -Include *.msc
```

## Parameters/Flags

- `Get-Service`: lists Windows services and their status
- `Start-Service` / `Stop-Service`: control service state
- `Select-Object *`: show all properties of an object
- `Get-CimInstance`: query WMI/CIM classes
- `-ErrorAction SilentlyContinue`: suppress errors

## Examples

### Service Management

List all running services:

```powershell
Get-Service | Where-Object { $_.Status -eq 'Running' }
```

Check and start a service if stopped:

```powershell
Get-Service Wsearch | Where-Object { $_.Status -eq 'Stopped' } | Start-Service
```

View all properties of a service:

```powershell
Get-Service BITS | Select-Object *
```

### List All MMC Snap-in Files (.msc)

```powershell
Get-ChildItem -Path C:\Windows\System32\* -Include *.msc |
  Sort-Object -Property Extension |
  Select-Object -Property Name |
  Format-Wide -Column 1
```

### Check for File Infector Malware

```powershell
sc.exe start "sppsvc" > $null 2>&1
Write-Host "Error code: $LASTEXITCODE"
```

If the output is `577` or `225`, the system may be infected with file infector malware.

### Diagnostic Data Viewer

Install and use the Microsoft Diagnostic Data Viewer module:

```powershell
# Install the module
Install-Module -Name Microsoft.DiagnosticDataViewer

# Enable data viewing
Enable-DiagnosticDataViewing

# View diagnostic data
Get-DiagnosticData

# View category definitions
Get-DiagnosticDataTypes

# Filter by time range
Get-DiagnosticData -StartTime (Get-Date).AddHours(-12) -EndTime (Get-Date).AddHours(-6)

# Export to CSV
Get-DiagnosticData | Export-Csv 'mydata.csv'
```

### Decode Registry Hex Values

```powershell
$key = "HKCU:\SOFTWARE\EXAMPLE\EXAMPLE\EXAMPLE"
Get-Item $key | Select-Object -ExpandProperty Property | ForEach-Object {
    $value = (Get-ItemProperty -Path $key -Name $_).$_
    [System.Text.Encoding]::Default.GetString($value) -replace '[\x01-\x1F]'
}
```

The `-replace '[\x01-\x1F]'` strips non-printable characters from the decoded output.

### Two PowerShell Learning Commands

Run daily to discover random cmdlet help and about topics:

```powershell
Get-Command -Module Microsoft*, Cim*, PS* | Get-Random | Get-Help -ShowWindow

Get-Random -InputObject (Get-Help about*) | Get-Help -ShowWindow
```

### Full System Shutdown (CMD)

```text
shutdown /s /f /t 0
```

### Count Saved Explorer Folder Views

```powershell
((Get-ItemProperty "HKCU:\Software\Classes\Local Settings\Software\Microsoft\Windows\Shell\BagMRU").NodeSlots).Count
```

## Related

- [`PowerShell System Inspection Patterns`](/powershell/querying/system-inspection-patterns/)
- [`PowerShell Cmdlet Patterns And Filtering`](/powershell/syntax/cmdlet-patterns-and-filtering/)
