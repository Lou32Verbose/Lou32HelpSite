---
title: Robocopy Backup Reference
slug: /windows/maintenance/robocopy-backup-reference/
summary: Robocopy templates for backup operations with attribute preservation and workarounds for the hidden/system attribute bug.
topic: windows/maintenance
type: template
tags: [windows, robocopy, backup, attributes]
aliases: [robocopy system hidden attribute bug, robocopy backup template preserve desktop ini]
platforms: [windows]
related:
  - /windows/maintenance/dism-appx-and-system-repair/
status: published
updated: 2026-03-21
---

## Use Case

Use Robocopy for local backup operations while preserving file attributes (System, Hidden) and avoiding the known bug where destination folders inherit System/Hidden attributes.

## Template

```text
robocopy "<source>" "<destination>" /E /COPY:DAT /DCOPY:T /R:3 /W:5 /A-:SH
```

## Variables

- `<source>`: source directory path
- `<destination>`: destination directory path
- `/E`: copy all subdirectories, including empty ones
- `/COPY:DAT`: copy Data, Attributes, and Timestamps (preserves system and hidden flags on files)
- `/DCOPY:T`: copy directory timestamps
- `/R:3`: retry 3 times on failure
- `/W:5`: wait 5 seconds between retries
- `/A-:SH`: remove System and Hidden attributes from destination folder (prevents the known bug)

## Examples

### Backup with Attribute Preservation

```text
robocopy "C:\Louis32" "D:\DIR32BACK\Louis32" /E /COPY:DAT /DCOPY:T /R:3 /W:5
```

This preserves System and Hidden flags on individual files while copying all subdirectories and timestamps.

### Prevent Hidden/System Attribute Bug

Robocopy has a known bug where the destination folder itself gets the System and Hidden attributes applied, making it invisible in Explorer. Append `/A-:SH` to prevent this:

```text
ROBOCOPY I:\DATA\ K:\DATA\ DB_DataFile.bak /A-:SH
```

> **Note:** The `/A-:SH` flag may not work from PowerShell -- use Command Prompt instead.

## Related

- [`DISM, AppX, And System Repair`](/windows/maintenance/dism-appx-and-system-repair/)
