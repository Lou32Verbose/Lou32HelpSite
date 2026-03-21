---
title: PowerShell System Inspection Patterns
slug: /powershell/querying/system-inspection-patterns/
summary: Common query patterns for filtering objects, exploring CIM classes, and inspecting processes, services, devices, and network state.
topic: powershell/querying
type: reference
tags: [powershell, cim, where-object, services, diagnostics, network, devices, drivers, netstat, svchost]
aliases: [where-object example, search for cim classnames, query devices and drivers, view open ports, manage services powershell]
platforms: [windows, powershell]
related:
  - /powershell/filesystem/file-and-text-recipes/
status: published
updated: 2026-03-21
---

## Synopsis

This page groups the PowerShell inspection patterns that show up repeatedly when you need to filter system data, enumerate CIM classes, inspect processes and services, query devices and drivers, monitor network traffic, check open ports, and perform security checks.

## Syntax

```powershell
Get-Command | Where-Object { $_.Name -like 'Get-*' }
Get-CimClass -ClassName Win32_* | Where-Object { $_.CimClassName -like '*Service*' }
Get-CimInstance Win32_Service | Format-Table Name, State, StartMode
Get-PnpDevice | Where-Object { $_.Status -ne "OK" } | Format-Table
Get-NetTCPConnection -State Listen | Format-Table LocalPort, LocalAddress
Get-NetAdapterStatistics -Name "Ethernet"
```

## Parameters/Flags

- `$_`: the current object in the pipeline
- `-like`: wildcard comparison operator
- `-match`: regex comparison operator
- `-eq`: equality comparison operator
- `-ne`: not-equal comparison operator
- `Get-CimClass`: explore available class definitions
- `Get-CimInstance`: query live system instances from a class
- `Get-PnpDevice`: enumerate Plug and Play devices
- `Get-NetTCPConnection`: query TCP connection state
- `Get-NetAdapterStatistics`: retrieve adapter traffic counters
- `Get-Service`: list Windows services and their status
- `Get-Process`: list running processes
- `Get-ItemProperty`: retrieve properties of a file or registry item

## Examples

### CIM/WMI Queries

Search for CIM classes by wildcard pattern:

```powershell
Get-CimClass Win32*Disk*
```

Find likely service-related CIM classes:

```powershell
Get-CimClass -ClassName Win32_* |
  Where-Object { $_.CimClassName -like '*Service*' } |
  Select-Object CimClassName
```

Get system architecture type (recommended CIM method):

```powershell
Get-CimInstance -Class Win32_OperatingSystem | Format-List OSArchitecture
```

Legacy WMI equivalent (deprecated, kept for reference):

```powershell
Get-WmiObject -Class Win32_OperatingSystem | Format-List OSArchitecture
```

Get all installed Win32 applications (preferred secure method):

```powershell
Get-CimInstance -ClassName Win32_InstalledWin32Program | Select-Object Name, Version, ProgramId
```

Alternative (less secure) method:

```powershell
Get-WMIObject Win32_InstalledWin32Program | Select-Object Name, Version, ProgramId
```

Get file properties using Get-ItemProperty:

```powershell
Get-ItemProperty -Path C:\path\to\file.txt | Get-Member -MemberType property
```

Access specific file properties via Get-ChildItem:

```powershell
$file = Get-ChildItem C:\path\to\file.txt
$file.Name
$file.Extension
$file.Length
$file.BaseName
```

Install and use the Diagnostic Data Viewer module:

```powershell
Install-Module -Name Microsoft.DiagnosticDataViewer
Enable-DiagnosticDataViewing
Get-DiagnosticData
Get-DiagnosticDataTypes
```

Filter diagnostic events by time range:

```powershell
Get-DiagnosticData -StartTime (Get-Date).AddHours(-12) -EndTime (Get-Date).AddHours(-6)
```

Export diagnostic data to CSV:

```powershell
Get-DiagnosticData | Export-Csv 'mydata.csv'
```

### Service Management

List all running services:

```powershell
Get-Service | Where-Object { $_.Status -eq 'Running' }
```

Check a service and start it if stopped:

```powershell
Get-Service Wsearch | Where-Object { $_.Status -eq 'Stopped' } | Start-Service
```

View all properties of a specific service:

```powershell
Get-Service BITS | Select-Object *
```

Inspect services hosted in `svchost.exe`:

```powershell
Get-CimInstance Win32_Service |
  Where-Object { $_.PathName -match 'svchost' } |
  Format-Table Name, State, StartMode, PathName -AutoSize
```

List all running svchost.exe process IDs:

```powershell
Get-Process -Name svchost | Select-Object Name, Id
```

View the command line for a specific svchost instance (replace `$pid` with the actual PID):

```powershell
$pid = <YourPID>
Get-CimInstance -ClassName Win32_Process -Filter "ProcessId = $pid" | Select-Object CommandLine
```

List services hosted by a specific svchost instance:

```powershell
Get-CimInstance -ClassName Win32_Service -Filter "ProcessId = $pid" |
  Format-Table Name, DisplayName, State, StartMode
```

Query a service configuration with sc:

```powershell
sc qc <ServiceName>
```

### Process Inspection

Get all running processes with virtual memory usage in a table:

```powershell
Get-Process | Format-Table Name, @{N='VIRTUAL MEMORY'; E={$_.VM/1MB}; FormatString='N2'} -AutoSize
```

### Network Monitoring

Monitor network adapter statistics:

```powershell
Get-NetAdapterStatistics -Name "Ethernet"
```

Real-time continuous network traffic monitoring:

```powershell
while ($true) {
    Get-NetAdapterStatistics -Name "Ethernet"
    Start-Sleep -Seconds 2
}
```

Track all active TCP connections sorted by state:

```powershell
Get-NetTCPConnection | Sort-Object State | Format-Table
```

Filter for established connections only:

```powershell
Get-NetTCPConnection -State Established | Format-Table
```

Measure bandwidth usage over a 10-second interval:

```powershell
$before = Get-NetAdapterStatistics -Name "Ethernet"
Start-Sleep -Seconds 10
$after = Get-NetAdapterStatistics -Name "Ethernet"
$bytesReceived = $after.ReceivedBytes - $before.ReceivedBytes
$bytesSent = $after.SentBytes - $before.SentBytes
Write-Output "Bytes received: $bytesReceived"
Write-Output "Bytes sent: $bytesSent"
```

Check network adapter link speeds:

```powershell
Get-NetAdapter | Select-Object Name, LinkSpeed
```

Automate daily network traffic report via scheduled task:

```powershell
$trigger = New-ScheduledTaskTrigger -Daily -At 6:00AM
$action = New-ScheduledTaskAction -Execute 'Powershell.exe' -Argument 'Get-NetAdapterStatistics -Name "Ethernet" | Out-File -FilePath C:\NetworkLogs\DailyTraffic.txt'
Register-ScheduledTask -Action $action -Trigger $trigger -TaskName "Daily Network Traffic Monitoring"
```

NTTTCP network throughput testing (receiver side):

```powershell
ntttcp.exe -r -m 4,*,192.168.242.5 -l 64k -a 2 -t 30
```

NTTTCP sender side (use same IP -- the receiver's address):

```powershell
ntttcp.exe -s -m 4,*,192.168.242.5 -l 64k -a 2 -t 30
```

NTTTCP must be run on both sides of the connection. Use `-s` for sender and `-r` for receiver. Output can be saved as XML for detailed throughput, CPU usage, and data transfer analysis.

### Device/Driver Queries

List all devices with a non-OK status:

```powershell
Get-PnpDevice | Where-Object { $_.Status -ne "OK" } | Format-Table
```

Get all installed driver information:

```powershell
Get-CimInstance Win32_PnPSignedDriver |
  Select-Object DeviceName, Manufacturer, DriverVersion, DriverDate |
  Format-Table -AutoSize
```

Export driver list to CSV:

```powershell
Get-CimInstance Win32_PnPSignedDriver |
  Select-Object DeviceName, Manufacturer, DriverVersion, DriverDate |
  Export-Csv -Path "$env:USERPROFILE\Desktop\DriverLog.csv" -NoTypeInformation
```

Filter for audio devices:

```powershell
Get-PnpDevice | Where-Object { $_.FriendlyName -like "*Audio*" -or $_.Class -eq "MEDIA" } | Format-Table
```

Filter for graphics/display drivers:

```powershell
Get-CimInstance Win32_PnPSignedDriver |
  Where-Object { $_.DeviceName -like "*Graphics*" -or $_.DeviceName -like "*Display*" } |
  Select-Object DeviceName, Manufacturer, DriverVersion, DriverDate |
  Format-Table -AutoSize
```

List hardware IDs for unknown/missing devices (useful for finding drivers online):

```powershell
Get-PnpDevice -PresentOnly |
  Where-Object { $_.Class -eq "Unknown" } |
  Select-Object FriendlyName, InstanceId
```

Copy the `InstanceId` value (e.g., `PCI\VEN_8086&DEV_15D7`) and search online to find the correct manufacturer and driver.

### Admin Rights Check

Check if the current session is running as administrator:

```powershell
if (-not ([bool](New-Object Security.Principal.WindowsPrincipal(
    [Security.Principal.WindowsIdentity]::GetCurrent()
)).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator))) {
    Write-Error "This script must be run as an administrator."
    exit 1
}
```

Check for file infector malware via sppsvc exit code (run as admin):

```powershell
sc.exe start "sppsvc" > $null 2>&1; Write-Host "Error code: $LASTEXITCODE"
```

If the output is error code 577 or 225, the system is likely infected with file infector malware.

### Where-Object Patterns

Basic Where-Object syntax with `-like` wildcard filtering:

```powershell
Get-AppxProvisionedPackage -Online |
  Where-Object { $_.DisplayName -like 'Microsoft.*' } |
  Format-Table DisplayName, PackageName
```

Where-Object syntax breakdown:

1. Wrap the filter expression in curly brackets `{ }`
2. Reference the current pipeline object property with `$_.PropertyName`
3. Place wildcards inside the quotes with the search string (e.g., `'Microsoft.*'`)
4. Pipe the output to `Format-Table` with specific property names for useful formatted results

Filter services by status using `-eq`:

```powershell
Get-Service | Where-Object { $_.Status -eq 'Running' }
```

Filter devices by multiple conditions using `-or`:

```powershell
Get-PnpDevice | Where-Object { $_.FriendlyName -like "*Audio*" -or $_.Class -eq "MEDIA" } | Format-Table
```

Filter by not-equal with `-ne`:

```powershell
Get-PnpDevice | Where-Object { $_.Status -ne "OK" } | Format-Table
```

Filter with regex using `-match`:

```powershell
Get-CimInstance Win32_Service |
  Where-Object { $_.PathName -match 'svchost' } |
  Format-Table Name, State, StartMode, PathName -AutoSize
```

### Open Ports

Check open listening ports from PowerShell:

```powershell
Get-NetTCPConnection -State Listen |
  Sort-Object LocalPort |
  Format-Table LocalAddress, LocalPort, OwningProcess
```

Monitor listening ports with Where-Object:

```powershell
Get-NetTCPConnection |
  Where-Object { $_.State -eq 'Listen' } |
  Format-Table LocalPort, LocalAddress
```

View open ports with netstat (works in both cmd and PowerShell, no admin required):

```powershell
netstat -a
```

List all connections with PIDs:

```powershell
netstat -ano
```

List all connections with executable names:

```powershell
netstat -ab
```

Identify which application owns a specific port by PID:

```powershell
tasklist /FI "PID eq <PID_number>"
```

## Related

- [`PowerShell File And Text Recipes`](/powershell/filesystem/file-and-text-recipes/)
- [`Using BITS Transfer With PowerShell`](/powershell/networking/bits-transfer/)
