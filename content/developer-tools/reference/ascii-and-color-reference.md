---
title: ASCII And Color Reference
slug: /developer-tools/reference/ascii-and-color-reference/
summary: Handy developer reference for common ASCII codes, basic hex color values, and a few frequently reused palette labels.
topic: developer-tools/reference
type: reference
tags: [ascii, color, hex, reference, developer-tools]
aliases: [complete list of ascii codes, color reference, color hex ref]
platforms: [windows, linux, macos]
related:
  - /browser-web/chromium/devtools-and-tab-helpers/
status: published
updated: 2026-03-20
---

## Synopsis

Keep the most frequently used ASCII and color lookups in one small reference instead of digging through separate legacy tables.

## Syntax

```text
ASCII: decimal code -> symbol
Color: #RRGGBB
```

## Parameters/Flags

- `ASCII code`: decimal value from the ASCII table
- `symbol`: character or control-code label
- `#RRGGBB`: two hex digits per red, green, and blue channel

## Examples

Common ASCII values:

```text
9   HT   Horizontal Tab
10  LF   Line Feed
13  CR   Carriage Return
27  ESC  Escape
64  @    At sign
92  \    Backslash
```

Common color values:

```text
#FF0000  Red
#00FF00  Green
#0000FF  Blue
#FFFFFF  White
#000000  Black
#012354  Noble Blue
```

## Related

- [`DevTools And Tab Helpers`](/browser-web/chromium/devtools-and-tab-helpers/)
