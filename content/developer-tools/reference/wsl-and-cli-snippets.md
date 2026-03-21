---
title: WSL And CLI Snippets
slug: /developer-tools/reference/wsl-and-cli-snippets/
summary: Miscellaneous WSL launch commands, CLI tool snippets, and small utility references including Codex, WinThumbsPreloader, and Super God Mode.
topic: developer-tools/reference
type: reference
tags: [wsl, cli, codex, winthumbspreloader, super-god-mode, windows]
aliases: [wsl codex example launch commands, winthumbspreloader help, loucommander super god mode tips]
platforms: [windows, linux]
related:
  - /developer-tools/reference/document-and-image-tools/
status: published
updated: 2026-03-21
---

## Synopsis

Collection of small CLI snippets and tool references that don't warrant their own pages.

## Syntax

```bash
codex --full-auto --search --add-dir "/mnt/c/path"
winthumbspreloader.exe [-r] [-s] <path>
```

## Parameters/Flags

- `--full-auto`: sandboxed automatic execution mode
- `--search`: enable web search functionality
- `--add-dir`: add an additional writable directory
- `-C`: explicitly set the working directory
- `-r`: recursive mode (WinThumbsPreloader)
- `-s`: silent mode (WinThumbsPreloader)

## Examples

### Codex Launch from WSL

Run Codex in full-auto mode with web search and an additional writable Windows directory:

```bash
codex --full-auto --search --add-dir "/mnt/c/LouTemp32/pyimgtest/"
```

- `--full-auto`: sandboxed automatic execution (equivalent to `-a on-request --sandbox workspace-write`)
- `--search`: enables web search
- `--add-dir`: adds a Windows directory (WSL path format `/mnt/c/...`) as writable alongside the current workspace

To explicitly set the current directory as the working root:

```bash
codex --full-auto --search -C . --add-dir "/mnt/c/LouTemp32/pyimgtest/"
```

The `-C .` is redundant since Codex uses the current directory by default.

### WinThumbsPreloader

Pre-generate Windows Explorer thumbnails for a directory:

```text
winthumbspreloader.exe [-r] [-s] <path>
```

- `-r`: recursive (process subdirectories)
- `-s`: silent (no output)

### Super God Mode (Windows Shell Shortcut Generator)

Tips for using the [Super God Mode](https://github.com/ThioJoe/Windows-Super-God-Mode) PowerShell script:

- Enable the **Link Target** column in Explorer (right-click column headers > More > Link Target) to see where generated shortcuts point
- Some links may not work — many are undocumented or leftover from older Windows versions
- The script reads the system dynamically (not a hardcoded list), so output varies by Windows version, enabled features, and installed software
- **Hidden App Links** folder: URLs are extracted from application binaries and are mostly undocumented; some contain variable placeholders (`{1}`, trailing `=`) and cannot be made into shortcuts
- To run the script without the batch launcher:

```powershell
Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process
.\Super_God_Mode.ps1
```

> **Warning:** Always include `-Scope Process` to avoid permanently lowering script execution policy.

## Related

- [`Document And Image Tools`](/developer-tools/reference/document-and-image-tools/)
