---
title: Disable Font Smoothing in Windows
slug: /windows/display/disable-font-smoothing/
summary: Fix overly soft text rendering by disabling Windows font smoothing.
topic: windows/display
type: troubleshooting
tags: [windows, fonts, registry, display]
aliases: [disable cleartype, font smoothing fix]
platforms: [windows]
related:
  - /powershell/networking/bits-transfer/
status: published
updated: 2026-03-20
---

## Symptoms

- Text looks blurred or overly softened.
- ClearType settings do not match the current display preference.

## Cause

Windows font smoothing or ClearType remains enabled when the current display setup looks better without it.

## Resolution

Disable the registry values that control font smoothing, then sign out and back in.

```powershell
Set-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name FontSmoothing -Value 0
Set-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name FontSmoothingType -Value 1
```

## Verification

- Sign out and back in.
- Confirm that text rendering looks sharper on the target display.
- Re-open the registry values to ensure they stayed at the intended settings.

## Related

- [`Using BITS Transfer with PowerShell`](/powershell/networking/bits-transfer/)
