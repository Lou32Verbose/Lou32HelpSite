---
title: ExifTool One-Line Reference
slug: /cli-tools/exiftool/one-line-reference/
summary: Compact reference for inspecting metadata, exporting CSV data, and renaming files with `exiftool`.
topic: cli-tools/exiftool
type: reference
tags: [exiftool, metadata, files, images]
aliases: [useful one-line exiftool commands]
platforms: [windows, linux, macos]
related:
  - /powershell/filesystem/file-and-text-recipes/
status: published
updated: 2026-03-20
---

## Synopsis

`exiftool` is useful when you need to inspect metadata, export selected tags, or rename media files from embedded date or camera information.

## Syntax

```text
exiftool <file>
exiftool -TagName <file>
exiftool "-FileName<expression" <file>
```

## Parameters/Flags

- `-csv`: export metadata as CSV
- `-ext`: restrict processing to one extension
- `-r`: recurse through subdirectories
- `-if`: process only files that match a condition

## Examples

Show all metadata:

```text
exiftool photo.jpg
```

Export selected tags from all JPG files:

```text
exiftool -Make -Model -DateTimeOriginal -csv -ext jpg /path/to/photos
```

Rename files from `DateTimeOriginal`:

```text
exiftool "-FileName<DateTimeOriginal" -d "%Y%m%dT%H%M%S.%%le" photo.jpg
```

Rename only Canon files:

```text
exiftool '-filename<CANON.%le' -if '$make eq "Canon"' photo.jpg
```

## Related

- [`PowerShell File And Text Recipes`](/powershell/filesystem/file-and-text-recipes/)
