---
title: Windows Error Codes Reference
slug: /windows/maintenance/windows-error-codes/
summary: Master reference of Windows system error codes with hex values and descriptions, covering basic system errors through DNS and directory service errors.
topic: windows/maintenance
type: reference
tags: [windows, error-codes, troubleshooting, reference]
aliases: [windows error codes master]
platforms: [windows]
related:
  - /windows/maintenance/dism-appx-and-system-repair/
  - /windows/troubleshooting/bitlocker-and-boot-troubleshooting/
status: published
updated: 2026-03-21
---

## Synopsis

Master lookup reference for Windows system error codes. Error codes appear in event logs, command output, and dialog boxes. Use this reference to decode hex error codes to human-readable descriptions.

## Syntax

```text
No.    Code    Description
```

Error codes can appear in decimal or hexadecimal format. Convert between them using `[Convert]::ToInt32("0x0005", 16)` in PowerShell.

## Parameters/Flags

- `No.`: decimal error number
- `Code`: hexadecimal representation (prefix `0x`)
- `[System.ComponentModel.Win32Exception]::new($code)`: .NET class that converts error codes to human-readable messages
- `[Convert]::ToInt32($hex, 16)`: convert hex string to decimal

## Examples

### Common System Errors (1-100)

| No. | Code | Description |
|-----|------|-------------|
| 1 | 0x0001 | Incorrect function |
| 2 | 0x0002 | The system cannot find the file specified |
| 3 | 0x0003 | The system cannot find the path specified |
| 4 | 0x0004 | The system cannot open the file |
| 5 | 0x0005 | Access is denied |
| 6 | 0x0006 | The handle is invalid |
| 7 | 0x0007 | The storage control blocks were destroyed |
| 8 | 0x0008 | Not enough memory resources |
| 13 | 0x000D | The data is invalid |
| 15 | 0x000F | The system cannot find the drive specified |
| 18 | 0x0012 | There are no more files |
| 19 | 0x0013 | The media is write protected |
| 21 | 0x0015 | The device is not ready |
| 32 | 0x0020 | The file is being used by another process |
| 33 | 0x0021 | Another process has locked a portion of the file |
| 39 | 0x0027 | The disk is full |
| 50 | 0x0032 | The request is not supported |
| 51 | 0x0033 | Windows cannot find the network path |
| 53 | 0x0035 | The network path was not found |
| 55 | 0x0037 | The specified network resource is no longer available |
| 65 | 0x0041 | Network access is denied |
| 67 | 0x0043 | The network name cannot be found |
| 80 | 0x0050 | The file exists |
| 87 | 0x0057 | The parameter is incorrect |

> **Note:** The full legacy document contains 3,400+ lines covering error codes from 0x0001 through 0x80013, including registry errors (1009-1022), service control errors (1051-1084), application/DLL errors (1150-1471), Windows Installer errors (1601-1662), RPC errors (1700-1836), network errors (2000-2999), encryption errors (6000-6023), Active Directory errors (8200-8657), and DNS errors (9001-9996). The complete list is available in the legacy source file `docs/LOU32HELP_windowserrorcodes_master.txt`.

### Quick Lookup with PowerShell

Look up an error code by number:

```powershell
# Convert hex to description using .NET
$errorCode = 5
$exception = [System.ComponentModel.Win32Exception]::new($errorCode)
Write-Host "$errorCode (0x$($errorCode.ToString('X4'))): $($exception.Message)"
```

Look up by hex code:

```powershell
$hex = "0x0005"
$decimal = [Convert]::ToInt32($hex, 16)
$exception = [System.ComponentModel.Win32Exception]::new($decimal)
Write-Host "$hex ($decimal): $($exception.Message)"
```

### Error Code Categories

| Range | Category |
|-------|----------|
| 1-999 | Basic system errors (file, memory, disk, pipe) |
| 1009-1022 | Registry errors |
| 1051-1084 | Service control manager errors |
| 1100-1129 | Tape and media errors |
| 1150-1471 | Application and DLL errors |
| 1500-1552 | Event log errors |
| 1601-1662 | Windows Installer errors |
| 1700-1836 | RPC errors |
| 1898-1930 | Printer errors |
| 2000-2999 | Network / NERR errors |
| 3000-3022 | Print spooler errors |
| 4000-4006 | WINS errors |
| 5000-5999 | Cluster errors |
| 6000-6023 | EFS encryption errors |
| 6600-6648 | Log service errors |
| 6700-6855 | Transaction errors |
| 7001-7070 | Terminal Services errors |
| 8001-8017 | File Replication Service errors |
| 8200-8657 | Active Directory / Directory Service errors |
| 9001-9996 | DNS errors |

## Related

- [`DISM, AppX, And System Repair`](/windows/maintenance/dism-appx-and-system-repair/)
- [`BitLocker And Boot Troubleshooting`](/windows/troubleshooting/bitlocker-and-boot-troubleshooting/)
