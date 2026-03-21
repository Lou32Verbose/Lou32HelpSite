---
title: PowerToys SmartRename Reference
slug: /developer-tools/reference/powertoys-smart-rename/
summary: Reference for PowerToys SmartRename options including regex patterns, matching modes, and practical renaming examples.
topic: developer-tools/reference
type: reference
tags: [powertoys, smartrename, regex, file-management, windows]
aliases: [smartrename readme, powertoys smart rename regex]
platforms: [windows]
related:
  - /developer-tools/reference/document-and-image-tools/
status: published
updated: 2026-03-21
---

## Synopsis

SmartRename (part of Microsoft PowerToys) provides bulk file renaming with plain text or ECMAScript regular expressions directly from the Explorer context menu.

## Syntax

```text
Search for: <text or regex pattern>
Replace with: <replacement text or regex variables>
```

## Parameters/Flags

| Option | Description |
|--------|-------------|
| **Use Regular Expressions** | Interpret the Search field as an ECMAScript regex. Enables `$1`, `$2` capture group variables in Replace. |
| **Case Sensitive** | Only match text with identical casing |
| **Match All Occurrences** | Replace every match, not just the first (left to right) |
| **Exclude Files** | Skip files, only rename folders |
| **Exclude Folders** | Skip folders, only rename files |
| **Exclude Subfolder Items** | Only rename items in the top-level selection |
| **Enumerate Items** | Append a numeric suffix to modified names (e.g., `foo (1).jpg`) |
| **Item Name Only** | Only modify the filename, not the extension |
| **Item Extension Only** | Only modify the extension, not the filename |

## Examples

### Simple Regex Patterns

| Search | Description |
|--------|-------------|
| `.*` | Match all text in the name |
| `^foo` | Match text beginning with "foo" |
| `bar$` | Match text ending with "bar" |
| `^foo.*bar$` | Match text starting with "foo" and ending with "bar" |
| `.+?(?=bar)` | Match everything up to "bar" |
| `foo[\s\S]*bar` | Match everything between "foo" and "bar" |

### Regex with Capture Groups

Enable **Match All Occurrences** when using variables.

| Search | Replace | Result |
|--------|---------|--------|
| `(.*).png` | `foo_$1.png` | Prepend "foo_" to filename |
| `(.*).png` | `$1_foo.png` | Append "_foo" to filename |
| `(.*)` | `$1.txt` | Append ".txt" extension |
| `(^\w+\.$)\|(^\w+$)` | `$2.txt` | Append ".txt" only if no extension exists |

## Related

- [`Document And Image Tools`](/developer-tools/reference/document-and-image-tools/)
