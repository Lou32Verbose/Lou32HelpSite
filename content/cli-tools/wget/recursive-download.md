---
title: Wget Recursive Download Reference
slug: /cli-tools/wget/recursive-download/
summary: Reference for downloading a site recursively with wget while preserving useful assets.
topic: cli-tools/wget
type: reference
tags: [wget, cli, mirroring, download]
aliases: [wget mirror, wget recursive]
platforms: [windows, linux, macos]
related:
  - /powershell/networking/bits-transfer/
status: published
updated: 2026-03-20
---

## Synopsis

Use `wget` to mirror a site or subsection while preserving assets and local navigation.

## Syntax

```text
wget -m -p -k -np https://example.com/
```

Common flags:

- `-m`: mirror mode
- `-p`: download page requisites
- `-k`: rewrite links for local browsing
- `-np`: do not ascend to parent directories

## Parameters/Flags

`-m`

Enables recursive retrieval plus timestamping defaults suited for mirroring.

`-p`

Downloads images, stylesheets, and other supporting files needed to render pages locally.

`-k`

Rewrites links so local copies browse correctly.

`-np`

Stops recursion from climbing above the starting path.

## Examples

Mirror a single documentation subtree:

```text
wget -m -p -k -np https://example.com/docs/
```

Throttle the crawl and write to a named directory:

```text
wget -m -p -k -np --wait=1 --directory-prefix=mirror https://example.com/
```

## Related

- [`Using BITS Transfer with PowerShell`](/powershell/networking/bits-transfer/)
