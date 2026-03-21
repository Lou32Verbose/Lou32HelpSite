---
title: Rclone Task Scheduler Template
slug: /backup-sync/rclone/task-scheduler-template/
summary: Template commands for running scheduled `rclone copy` jobs with timestamped logs.
topic: backup-sync/rclone
type: template
tags: [rclone, backup, sync, scheduler]
aliases: [rclone scheduled copy, rclone task scheduler]
platforms: [windows, powershell]
related:
  - /powershell/networking/bits-transfer/
status: published
updated: 2026-03-20
---

## Use Case

Schedule repeatable `rclone copy` jobs from Task Scheduler while keeping per-run log files.

## Template

```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {
  $ts = Get-Date -Format yyyyMMdd_HHmmss
  $log = 'C:\WinMgmt\logs\rclone-copy-' + $ts + '.log'
  if (-not (Test-Path 'C:\WinMgmt\logs')) {
    New-Item -ItemType Directory -Path 'C:\WinMgmt\logs' | Out-Null
  }
  rclone copy '<REMOTE>:<SOURCE>' '<DESTINATION>' --log-file $log --log-level INFO
}"
```

## Variables

- `<REMOTE>`: configured remote name
- `<SOURCE>`: source path inside the remote
- `<DESTINATION>`: local or remote destination path
- `--log-file`: per-run log output path

## Examples

OneDrive to local path:

```powershell
rclone copy 'OneDrive:Louis32' 'C:\Louis32' --fast-list --retries 8
```

Google Drive to local path:

```powershell
rclone copy 'Google:Louis32' 'C:\Louis32' --drive-chunk-size 256M --retries 6
```

## Related

- [`Using BITS Transfer with PowerShell`](/powershell/networking/bits-transfer/)
