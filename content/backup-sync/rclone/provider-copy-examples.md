---
title: Rclone Provider Copy Examples
slug: /backup-sync/rclone/provider-copy-examples/
summary: Reusable `rclone copy` patterns for local-to-cloud and cloud-to-local sync jobs across common providers.
topic: backup-sync/rclone
type: template
tags: [rclone, backup, sync, onedrive, google-drive]
aliases: [lou32 rclone command list, rclone example command for google drive]
platforms: [windows, powershell]
related:
  - /backup-sync/rclone/task-scheduler-template/
status: published
updated: 2026-03-20
---

## Use Case

Start from a known-good `rclone copy` command for OneDrive, Google Drive, or Dropbox and then tune transfers, checkers, and chunk sizes to match the provider.

## Template

```powershell
rclone copy 'REMOTE:SourcePath' 'C:\TargetPath' `
  --update `
  --transfers 4 `
  --checkers 8 `
  --fast-list `
  --retries 8 `
  --low-level-retries 20 `
  --progress
```

## Variables

- `REMOTE`: configured rclone remote such as `OneDrive`, `Google`, or `Dropbox`
- `SourcePath`: remote directory to copy from
- `C:\TargetPath`: local destination path
- `--transfers` and `--checkers`: parallelism controls that vary by provider

## Examples

OneDrive to local:

```powershell
rclone copy 'OneDrive:Louis32' 'C:\Louis32' --update --transfers 12 --checkers 12 --buffer-size 16M --multi-thread-streams 4 --progress
```

Google Drive to local:

```powershell
rclone copy 'Google:Louis32' 'C:\Louis32' --update --transfers 12 --checkers 12 --progress
```

Local back to a remote:

```powershell
rclone copy 'C:\Louis32' 'OneDrive:Louis32' --update --transfers 12 --checkers 12 --buffer-size 16M --multi-thread-streams 4 --progress
```

## Related

- [`Rclone Task Scheduler Template`](/backup-sync/rclone/task-scheduler-template/)
