---
title: Font Cache And Imaging Fonts
slug: /windows/maintenance/font-cache-and-imaging-fonts/
summary: Batch script to rebuild the Windows font cache and minimum font list for Windows deployment images.
topic: windows/maintenance
type: recipe
tags: [windows, fonts, font-cache, deployment, imaging]
aliases: [rebuild font cache win10, minimum font list windows images]
platforms: [windows]
related:
  - /windows/display/disable-font-smoothing/
status: published
updated: 2026-03-21
---

## Goal

Rebuild a corrupted Windows font cache and maintain the minimum required font set for Windows deployment images.

## Prerequisites

- Elevated Command Prompt for font cache rebuild
- Administrator access

## Steps

1. Stop the Font Cache service.
2. Grant access to the cache directory and delete cached files.
3. Re-enable and start the service.

## Commands

### Rebuild Font Cache (Batch Script)

Save as a `.bat` file and run as administrator:

```text
@echo off

:: Stop and disable Windows Font Cache Service
:FontCache
sc stop "FontCache"
sc config "FontCache" start=disabled
sc query FontCache | findstr /I /C:"STOPPED"
if not %errorlevel%==0 (goto FontCache)

:: Grant access rights to current user for service profile folder
icacls "%WinDir%\ServiceProfiles\LocalService" /grant "%UserName%":F /C /T /Q

:: Delete font cache files
del /A /F /Q "%WinDir%\ServiceProfiles\LocalService\AppData\Local\FontCache\*FontCache*"
del /A /F /Q "%WinDir%\System32\FNTCACHE.DAT"

:: Enable and start Windows Font Cache Service
sc config "FontCache" start=auto
sc start "FontCache"
```

### Minimum Font List for Windows Images

When building stripped-down Windows deployment images, include at minimum these fonts to avoid rendering issues:

- Arial
- Arial Black
- Calibri
- Cambria
- Comic Sans MS
- Consolas
- Courier
- Courier New
- Georgia
- Lucida Console
- Lucida Sans Unicode
- Malgun Gothic
- Marlett
- Microsoft Sans Serif
- MS Sans Serif
- MS Serif
- MS Gothic / MS UI Gothic
- Segoe UI
- Symbol
- System
- Tahoma
- Terminal
- Times New Roman
- Trebuchet MS

## Verification

- After rebuilding the font cache, verify the FontCache service is running: `sc query FontCache`
- Open applications that use custom fonts and confirm they render correctly.
- Reboot if font rendering issues persist.

## Related

- [`Disable Font Smoothing`](/windows/display/disable-font-smoothing/)
