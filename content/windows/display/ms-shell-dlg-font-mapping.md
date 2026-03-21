---
title: MS Shell Dlg Font Mapping
slug: /windows/display/ms-shell-dlg-font-mapping/
summary: Reference for MS Shell Dlg and MS Shell Dlg 2 logical font face names used by Windows for locale-aware UI font substitution.
topic: windows/display
type: reference
tags: [windows, fonts, internationalization, registry, ui]
aliases: [using ms shell dlg and ms shell dlg 2]
platforms: [windows]
related:
  - /windows/display/disable-font-smoothing/
  - /windows/maintenance/font-cache-and-imaging-fonts/
status: published
updated: 2026-03-21
---

## Synopsis

Windows uses two logical font face names — `MS Shell Dlg` and `MS Shell Dlg 2` — to select the correct UI font for the active locale. Applications that reference these logical names instead of hard-coding a font face will display correctly across all Windows languages without modification.

## Syntax

```text
HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\Current Version\FontSubstitutes
```

The registry key above stores the mappings from logical font names to physical fonts.

## Parameters/Flags

- **MS Shell Dlg**: maps to the default shell font for the current locale. On most Western systems this resolves to **Microsoft Sans Serif**; on Japanese systems it resolves to **MS UI Gothic**.
- **MS Shell Dlg 2**: always maps to **Tahoma** regardless of locale. Tahoma has a native bold face but may not exist on very old systems.

## Examples

### Physical Font Mapping by Platform

| Platform | MS Shell Dlg | MS Shell Dlg 2 |
|----------|-------------|----------------|
| Windows 95/98/Me | Code-page-specific MS Sans Serif | N/A |
| Windows NT 4.0 | MS Sans Serif (Western), MS UI Gothic (Japanese), Gulim (Korean), Simsun (Simplified Chinese), PMinglu (Traditional Chinese) | N/A |
| Windows 2000+ | Microsoft Sans Serif (or MS UI Gothic if install language is Japanese) | Tahoma |
| Windows Vista/7+ | Microsoft Sans Serif (or MS UI Gothic if default UI language is Japanese) | Tahoma |

### Why Avoid Hard-Coded Font Names

- A hard-coded font that covers one script may not contain glyphs for another language, causing garbled text in localized UIs.
- Treat font names and font sizes as localizable resources.
- Use `MS Shell Dlg` or `MS Shell Dlg 2` in dialog templates so localizers only need to translate text, not change font specifications.

### Font Linking

Characters not present in Tahoma or Microsoft Sans Serif are rendered through Windows font linking, which automatically selects fallback fonts that contain the needed glyphs.

## Related

- [`Disable Font Smoothing`](/windows/display/disable-font-smoothing/)
- [`Font Cache And Imaging Fonts`](/windows/maintenance/font-cache-and-imaging-fonts/)
