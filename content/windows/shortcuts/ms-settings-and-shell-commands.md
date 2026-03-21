---
title: MS Settings And Shell Commands
slug: /windows/shortcuts/ms-settings-and-shell-commands/
summary: Reference for creating Settings app shortcuts with ms-settings URIs, shell protocol paths, and DPI settings for custom Start button images.
topic: windows/shortcuts
type: reference
tags: [windows, ms-settings, shell, shortcuts, dpi]
aliases: [settings app shortcuts ms-settings, shell protocol path list, window switcher shell command, dpi settings custom start menu, office 365 normal dotx default path]
platforms: [windows]
related:
  - /windows/shortcuts/settings-and-shell-shortcuts/
  - /windows/policy/registry-clsid-and-shell-reference/
status: published
updated: 2026-03-21
---

## Synopsis

Reference for creating desktop shortcuts to Windows Settings pages using `ms-settings:` URIs, the complete `shell:` protocol path list, Window Switcher shell command, and DPI pixel size reference for custom Start button creation.

## Syntax

```text
explorer.exe ms-settings:<page>
shell:<folder-name>
```

## Parameters/Flags

- `ms-settings:<page>`: opens a specific Settings app page (e.g., `ms-settings:colors`)
- `shell:<name>`: opens a known folder by its shell protocol name
- `shell:::{GUID}`: opens a shell folder by CLSID (see the CLSID reference)

## Examples

### Creating a Settings App Shortcut

1. Right-click on the desktop and select **New > Shortcut**.
2. Enter the command in the format `explorer.exe ms-settings:<command>` as the location. For example:

```text
explorer.exe ms-settings:colors
```

3. Name the shortcut and optionally change the icon.

### Window Switcher Shell Command

Open the Window Switcher via Run dialog or Command Prompt (does not work in PowerShell):

```text
C:\WINDOWS\explorer.exe shell:::{3080F90E-D7AD-11D9-BD98-0000947B0257}
```

To create a shortcut, set this as the shortcut target.

### DPI Settings for Custom Start Button Images

Use these pixel dimensions based on your display scaling:

| DPI Scale | Image Size |
|-----------|-----------|
| 100% | 54 x 162 px |
| 125% | 66 x 198 px |
| 150% | 81 x 243 px |

### Shell Protocol Paths

Common `shell:` paths and their resolved locations:

| Shell Path | Location |
|-----------|----------|
| `shell:Desktop` | Desktop |
| `shell:Personal` | `%UserProfile%\Documents` |
| `shell:My Music` | `%UserProfile%\Music` |
| `shell:My Pictures` | `%UserProfile%\Pictures` |
| `shell:My Video` | `%UserProfile%\Videos` |
| `shell:Downloads` | `%UserProfile%\Downloads` |
| `shell:Favorites` | `%UserProfile%\Favorites` |
| `shell:Contacts` | `%UserProfile%\Contacts` |
| `shell:SavedGames` | `%UserProfile%\Saved Games` |
| `shell:Searches` | `%UserProfile%\Searches` |
| `shell:Links` | `%UserProfile%\Links` |
| `shell:Profile` | `%UserProfile%` |
| `shell:AppData` | `%AppData%` |
| `shell:Local AppData` | `%LocalAppData%` |
| `shell:LocalAppDataLow` | `%UserProfile%\AppData\LocalLow` |
| `shell:Start Menu` | `%AppData%\Microsoft\Windows\Start Menu` |
| `shell:Programs` | `%AppData%\...\Start Menu\Programs` |
| `shell:Startup` | `%AppData%\...\Start Menu\Programs\Startup` |
| `shell:Administrative Tools` | `%AppData%\...\Start Menu\Programs\Administrative Tools` |
| `shell:SendTo` | `%AppData%\Microsoft\Windows\SendTo` |
| `shell:Templates` | `%AppData%\Microsoft\Windows\Templates` |
| `shell:Recent` | `%AppData%\Microsoft\Windows\Recent` |
| `shell:Quick Launch` | `%AppData%\...\Internet Explorer\Quick Launch` |
| `shell:Cookies` | `%LocalAppData%\Microsoft\Windows\INetCookies` |
| `shell:Cache` | `%LocalAppData%\Microsoft\Windows\INetCache` |
| `shell:History` | `%LocalAppData%\Microsoft\Windows\History` |
| `shell:CD Burning` | `%LocalAppData%\Microsoft\Windows\Burn\Burn` |
| `shell:Common Desktop` | `%Public%\Desktop` |
| `shell:Common Documents` | `%Public%\Documents` |
| `shell:CommonDownloads` | `%Public%\Downloads` |
| `shell:CommonMusic` | `%Public%\Music` |
| `shell:CommonPictures` | `%Public%\Pictures` |
| `shell:CommonVideo` | `%Public%\Videos` |
| `shell:Common AppData` | `%ProgramData%` |
| `shell:Common Programs` | `%ProgramData%\...\Start Menu\Programs` |
| `shell:Common Startup` | `%ProgramData%\...\Start Menu\Programs\Startup` |
| `shell:Common Start Menu` | `%ProgramData%\...\Start Menu` |
| `shell:Common Templates` | `%ProgramData%\Microsoft\Windows\Templates` |
| `shell:ProgramFiles` | `%ProgramFiles%` |
| `shell:ProgramFilesX86` | `%ProgramFiles(x86)%` |
| `shell:ProgramFilesCommon` | `%ProgramFiles%\Common Files` |
| `shell:System` | `%WinDir%\System32` |
| `shell:SystemX86` | `%WinDir%\SysWOW64` |
| `shell:Fonts` | `%WinDir%\Fonts` |
| `shell:Windows` | `%WinDir%` |
| `shell:UserProfiles` | `%HomeDrive%\Users` |
| `shell:Public` | `%Public%` |
| `shell:OneDrive` | OneDrive root |
| `shell:Screenshots` | `%UserProfile%\Pictures\Screenshots` |
| `shell:Camera Roll` | `%UserProfile%\Pictures\Camera Roll` |
| `shell:AccountPictures` | `%AppData%\...\AccountPictures` |
| `shell:Libraries` | Libraries |
| `shell:MyComputerFolder` | This PC |
| `shell:RecycleBinFolder` | Recycle Bin |
| `shell:NetworkPlacesFolder` | Network |
| `shell:ControlPanelFolder` | All Control Panel Items |
| `shell:PrintersFolder` | All Printers |
| `shell:ConnectionsFolder` | Network Connections |
| `shell:InternetFolder` | Internet Explorer |
| `shell:AppsFolder` | Applications |

### Microsoft Office Default Template Path

The default Word template (`normal.dotx`) that controls new document formatting is stored at:

```text
C:\Users\<username>\AppData\Roaming\Microsoft\Templates\normal.dotx
```

Delete or rename this file to reset Word to factory default formatting.

## Related

- [`Settings And Shell Shortcuts`](/windows/shortcuts/settings-and-shell-shortcuts/)
- [`Registry CLSID And Shell Reference`](/windows/policy/registry-clsid-and-shell-reference/)
