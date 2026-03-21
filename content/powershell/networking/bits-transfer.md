---
title: Using BITS Transfer with PowerShell
slug: /powershell/networking/bits-transfer/
summary: Reliable large-file downloads with BITS from PowerShell.
topic: powershell/networking
type: recipe
tags: [powershell, bits, transfer, downloads]
aliases: [start-bitstransfer, bits transfer]
platforms: [windows, powershell]
related:
  - /cli-tools/wget/recursive-download/
status: published
updated: 2026-03-20
---

## Goal

Download large files reliably without restarting the entire transfer if the connection drops.

## Prerequisites

- Windows with BITS available
- PowerShell session with permission to start downloads
- A destination path with enough free disk space

## Steps

1. Define the source URL and destination path.
2. Start the transfer with `Start-BitsTransfer`.
3. Check status if the transfer is long-running.
4. Resume or complete the job if needed.

## Commands

```powershell
$url = "https://download.example.com/tool.iso"
$destination = "D:\Downloads\tool.iso"
Start-BitsTransfer -Source $url -Destination $destination
```

For a named asynchronous job:

```powershell
$job = Start-BitsTransfer -Source $url -Destination $destination -Asynchronous -DisplayName "tool-download"
Get-BitsTransfer -Name "tool-download"
Resume-BitsTransfer -BitsJob $job
Complete-BitsTransfer -BitsJob $job
```

## Verification

- Confirm the destination file exists.
- Compare file size or checksum with the original source when available.
- Review the BITS job state for any retry or transient errors.

## Related

- [`Wget Recursive Download Reference`](/cli-tools/wget/recursive-download/)
