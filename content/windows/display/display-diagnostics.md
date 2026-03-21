---
title: Display Diagnostics
slug: /windows/display/display-diagnostics/
summary: Reference for DPI queries, font rendering, font cache repair, console font tuning, font installation, and related Windows display troubleshooting notes.
topic: windows/display
type: reference
tags: [windows, display, dpi, fonts, diagnostics, cleartype, font-cache, truetype, hinting]
aliases: [get screen dpi, rebuild font cache, console fonts registry, font rendering guide, font substitutes, install fonts command line]
platforms: [windows]
related:
  - /windows/display/disable-font-smoothing/
status: published
updated: 2026-03-21
---

## Synopsis

Use these notes when you need to inspect screen DPI, configure font rendering and antialiasing, rebuild font-related caches, register console fonts via the registry, install or remove fonts from the command line, or trace display and font behavior that affects readability or shell layout.

## Syntax

```powershell
Get-CimInstance Win32_VideoController
Get-ItemProperty 'HKCU:\Console'
Stop-Service FontCache
```

```cmd
sc stop "FontCache"
sc config "FontCache" start=disabled
mountvol y: /s
```

## Parameters/Flags

- `Win32_VideoController`: basic display adapter details
- `HKCU:\Console`: per-user console font and window settings
- `FontCache`: Windows font cache service (`{B0D17FC2-7BC4-11d1-BDFA-00C04FA31009}`)
- `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Console\TrueTypeFont`: registry key for adding console-available TrueType fonts
- `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts`: master font registration key
- `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\FontSubstitutes`: font substitution mappings

## Examples

### Get Screen DPI

Query primary screen DPI via PowerShell:

```powershell
Add-Type -AssemblyName System.Windows.Forms
$screen = [System.Windows.Forms.Screen]::PrimaryScreen
$dpiX = $screen.Bounds.Width / ($screen.Bounds.Width - $screen.WorkingArea.Width + $screen.Bounds.Width)
$dpiY = $screen.Bounds.Height / ($screen.Bounds.Height - $screen.WorkingArea.Height + $screen.Bounds.Height)
Write-Host "DPI-X: $($dpiX)"
Write-Host "DPI-Y: $($dpiY)"
```

View display adapter information:

```powershell
Get-CimInstance Win32_VideoController |
  Select-Object Name, CurrentHorizontalResolution, CurrentVerticalResolution
```

### DPI Scaling Reference for Custom Start Button Images

When creating custom start button images, use these dimensions based on your app scaling percentage:

| DPI Scaling | Image Size     |
|-------------|----------------|
| 100%        | 54 x 162 px    |
| 125%        | 66 x 198 px    |
| 150%        | 81 x 243 px    |

---

### Font Rendering Guide (Antialiasing by Display Type)

Most computer monitors use an RGB subpixel layout, so most software (including Windows) optimizes text rendering for that arrangement. This optimization can cause color fringing on displays with non-standard pixel structures.

#### Changing Font Rendering Settings in Windows

**Method 1: ClearType Tuner**
Windows > Settings > search: ClearType. Lets you change several font rendering settings. Works well for displays that use standard RGB or BGR subpixel layouts.

**Method 2: Better ClearType Tuner (open source)**
<https://github.com/bp2008/BetterCleartypeTuner> -- simpler interface that clarifies what each setting does. Works well for RGB or BGR layouts.

**Method 3: MacType**
<https://www.mactype.net/> -- advanced font rendering customization. Excellent choice for panels with unusual pixel structures where you want better grayscale antialiased text.

#### Font Rendering in Chrome

Chrome supports grayscale rendering via the command line flag `--disable-lcd-text`. The [ColorControl](https://github.com/Maassoft/ColorControl) program exposes this as a checkbox in its Options tab.

#### Font Rendering in Firefox and Thunderbird

Access advanced config: in Firefox enter `about:config` in the address bar; in Thunderbird go to Tools > Preferences > Config Editor. Search for `font_rendering`.

- **Grayscale antialiasing**: set `gfx.font_rendering.cleartype_params.cleartype_level` to `0`
- **BGR antialiasing**: set `gfx.font_rendering.cleartype_params.pixel_structure` to `2` (use `-1` for RGB)

#### Display Type Recommendation Table

| Display Type | Found In | Best Text Rendering | Explanation |
|---|---|---|---|
| RGB subpixels | Most LCD monitors | RGB subpixel AA | Default in most OSes. Uses known subpixel layout for higher apparent horizontal resolution. |
| BGR subpixels | Many LCD televisions | BGR subpixel AA | Reversed form of RGB; commonly supported in OS settings. |
| CRT | CRT monitors/TVs | Grayscale AA or none | CRTs lack a predictable subpixel arrangement. |
| Rotated monitor | Any display technology | Grayscale AA | Rotating 90/270 degrees yields a vertical subpixel layout rarely supported in software. |
| DLP | DLP projectors | Grayscale AA | DLP does not use subpixels; whole pixels are uniform color. |
| OLED (2022 and earlier) | LG Display OLED TVs/monitors | Grayscale AA | No software offers effective subpixel AA for this arrangement. |
| QD-OLED (2022) | Samsung Display QD-OLED TVs/monitors | RGB or Grayscale AA | Triangular RGB layout (green above red and blue) can cause some color fringing. |

---

### Rebuild the Windows Font Cache

#### PowerShell method

```powershell
Stop-Service FontCache
Remove-Item "$env:WinDir\ServiceProfiles\LocalService\AppData\Local\FontCache\*" -Force
Start-Service FontCache
```

#### Batch script method (full reset)

```bat
@echo off

:: Stop and disable "Windows Font Cache Service"
:FontCache
sc stop "FontCache"
sc config "FontCache" start=disabled
sc query FontCache | findstr /I /C:"STOPPED"
if not %errorlevel%==0 (goto FontCache)

:: Grant access rights to current user for font cache folder
icacls "%WinDir%\ServiceProfiles\LocalService" /grant "%UserName%":F /C /T /Q

:: Delete font cache files
del /A /F /Q "%WinDir%\ServiceProfiles\LocalService\AppData\Local\FontCache\*FontCache*"
del /A /F /Q "%WinDir%\System32\FNTCACHE.DAT"

:: Re-enable and start "Windows Font Cache Service"
sc config "FontCache" start=auto
sc start "FontCache"
```

IFontCache CLSID: `{B0D17FC2-7BC4-11d1-BDFA-00C04FA31009}`

---

### Console Font Registry Setup (KB247815)

Fonts available in a command session window must meet these criteria:

- Must be a fixed-pitch font
- Cannot be an italic font
- Cannot have a negative A or C space
- If TrueType, must be `FF_MODERN`
- If not TrueType, must be `OEM_CHARSET`

Additional criteria for Asian installations:

- If not TrueType, the face name must be "Terminal"
- If an Asian TrueType font, it must also be an Asian character set

To add a console font, create a String Value in:

```
HKLM\Software\Microsoft\Windows NT\CurrentVersion\Console\TrueTypeFont
```

| Value Name | Data |
|---|---|
| `00` | Font Name (first additional font) |
| `000` | Font Name (second additional font) |
| `0000` | Font Name (third additional font) |

The name is incremented with an additional `0` for each font. The data entry must match the font's entry in:

```
HKLM\Software\Microsoft\Windows NT\CurrentVersion\Fonts
```

---

### Inspect Console Font Settings

```powershell
Get-ItemProperty 'HKCU:\Console'
```

---

### Installing Fonts via Command Line

#### CMD: copy and register

```cmd
copy "FontName.ttf" "%WINDIR%\Fonts"
reg add "HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts" /v "FontName (TrueType)" /t REG_SZ /d FontName.ttf /f
```

#### VBScript: Shell.Application install

```vbs
Set objShell = CreateObject("Shell.Application")
Set objFolder = objShell.Namespace("<Folder or Share Location>")
Set objFolderItem = objFolder.ParseName("<TTF File Name>")
objFolderItem.InvokeVerb("Install")
```

Example:

```vbs
Set objShell = CreateObject("Shell.Application")
Set objFolder = objShell.Namespace("C:\Windows\Font")
Set objFolderItem = objFolder.ParseName("Myriad Pro.ttf")
objFolderItem.InvokeVerb("Install")
```

#### Temporary font installation (current session only)

Run `fontview.exe` for each font, making it available to other Windows applications:

```cmd
for /F "delims=;" %%a in ('dir C:\ExtraFonts /B /A-D-H-S /S') do fontview %%a
```

---

### Minimum Needed Fonts for Windows Images

The following fonts are the minimum set needed for a functional Windows image:

| Font |
|---|
| Arial |
| Arial Black |
| Calibri |
| Cambria |
| Comic Sans MS |
| Consolas |
| Courier |
| Courier New |
| Georgia |
| Lucida Console |
| Lucida Sans Unicode |
| Malgun Gothic |
| Marlett |
| Microsoft Sans Serif |
| MS Sans Serif |
| MS Serif |
| MS Gothic & MS UI Gothic |
| Segoe UI |
| Symbol |
| System |
| Tahoma |
| Terminal |
| Times New Roman |
| Trebuchet MS |

---

### Default Font Substitutes (Windows 10)

Registry key: `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\FontSubstitutes`

| Substitute Name | Maps To |
|---|---|
| Arabic Transparent | Arial |
| Arabic Transparent Bold | Arial Bold |
| Arabic Transparent Bold,0 | Arial Bold,178 |
| Arabic Transparent,0 | Arial,178 |
| Arial Baltic,186 | Arial,186 |
| Arial CE,238 | Arial,238 |
| Arial CYR,204 | Arial,204 |
| Arial Greek,161 | Arial,161 |
| Arial TUR,162 | Arial,162 |
| Courier New Baltic,186 | Courier New,186 |
| Courier New CE,238 | Courier New,238 |
| Courier New CYR,204 | Courier New,204 |
| Courier New Greek,161 | Courier New,161 |
| Courier New TUR,162 | Courier New,162 |
| Helv | MS Sans Serif |
| Helvetica | Arial |
| MS Shell Dlg | Microsoft Sans Serif |
| MS Shell Dlg 2 | Tahoma |
| Tahoma Armenian | Tahoma |
| Times | Times New Roman |
| Times New Roman Baltic,186 | Times New Roman,186 |
| Times New Roman CE,238 | Times New Roman,238 |
| Times New Roman CYR,204 | Times New Roman,204 |
| Times New Roman Greek,161 | Times New Roman,161 |
| Times New Roman TUR,162 | Times New Roman,162 |
| Tms Rmn | MS Serif |

---

### Remove Fonts from System Reserved Partition

For Windows upgrade error codes `0xc1900104` and `0x800f0922` ("We couldn't update the system reserved partition"), fonts can be removed from the system reserved partition.

Reference: <https://support.microsoft.com/en-us/topic/-we-couldn-t-update-system-reserved-partition-error-installing-windows-10-46865f3f-37bb-4c51-c69f-07271b6672ac>

#### GPT partition (Windows 10)

Run as Administrator:

```cmd
mountvol y: /s
Y:
cd EFI\Microsoft\Boot\Fonts
del *.*
Y
```

#### MBR partition (Windows 10)

1. Open `diskmgmt.msc`, right-click the **System Reserve** partition, select **Change Drive Letter and Paths**, click **Add**, assign `Y:`
2. Run as Administrator:

```cmd
Y:
cd Boot\Fonts
takeown /d y /r /f .
```

3. Back up permissions:

```cmd
icacls Y:\* /save %systemdrive%\NTFSp.txt /c /t
```

4. Grant your user full control (run `whoami` to get username):

```cmd
icacls . /grant <username>:F /t
```

5. Delete the font files:

```cmd
del *.*
```

6. **Restore permissions** (required):

```cmd
icacls Y:\ /restore %systemdrive%\NTFSp.txt /c /t
icacls . /grant system:f /t
icacls Y: /setowner "SYSTEM" /t /c
```

7. In Disk Management, refresh data to confirm free space increased, then remove the `Y:` drive letter.

---

### TrueType Hinting: Philosophy and Instructions

Reference from Microsoft Typography on the three global TrueType hinting tables.

#### Control Value Table (`cvt`)

The `cvt` table stores measurements of font features (e.g., lowercase stem widths). Control values ensure that similar features render at the same pixel width at low resolutions, while at higher resolutions the natural outline distance is used. Examples of stored values:

- Cap heights, x-heights, overshoot distances
- Ascender heights, baselines, figure heights
- Serif lengths and heights, italic angle
- Group stem distances (uppercase, lowercase)
- Specialized distances (math signs, braces, brackets, parentheses)

#### Preprogram (`prep`)

Executed each time a glyph size or resolution changes, before glyph-local instructions run. Common uses:

- **Hint range control**: Microsoft fonts disable hints below 8 ppem and above 2048 ppem
- **Dropout control**: checks for missing pixels in continuous strokes where the outline passes between pixel centers
- **CVT value adjustments**: control the size at which a stem breaks from one pixel to two; force round and straight features to be equal until a desired size
- **CVT cut-in limit**: threshold determining whether a CVT value or the actual distance is used (small ppem = always use CVT; medium = reasonable threshold; high ppem = always use actual distance)
- **Minimum distance**: maintain at least one pixel between outline points (e.g., across a stem)

#### Font Program (`fpgm`)

Stores reusable functions called from `prep` or glyph instructions. Common example: controlling diacritic placement in composite glyphs to ensure the accent centers over the base glyph and does not touch it.

#### Key terms

| Term | Definition |
|---|---|
| ppem | Pixels per em. Calculated as: `pt_size * resolution / 72` |
| Bitmap | The displayed character image after hints are processed (not a bitmap font file) |
| Glyph | An image in a font file; a character may map to multiple glyphs in modern font technologies |
| Overshoot | The difference between round and flat glyph heights; round characters project higher/lower to appear equal height |
| Diacritic | A mark (accent) used in conjunction with another glyph |

---

### Making TrueType Bitmap Fonts

Reference by Vincent Connare, Microsoft Typography.

Advance widths of glyphs containing bitmaps must scale to approximately the same advance width as the bitmaps, because text engines use outline metrics for page layout.

#### Calculating advance widths

For a 2048 units-per-em font: divide 2048 by the ppem size, then multiply by the bitmap advance width.

**Example**: at 13 ppem, if lowercase `a` is 7 pixels wide:
`2048 / 13 = 157.53` then `157.53 * 7 = 1102.7` -- use advance width **1103**.

If bitmaps do not scale linearly across sizes, average to find the best width.

#### Point size to ppem conversion

Formula: `pt_size * resolution / 72 = ppem`

| Point Size | VGA (96 dpi) ppem | SVGA (120 dpi) ppem |
|---|---|---|
| 8 pt  | 11 | 13 |
| 10 pt | 13 | 17 |
| 12 pt | 16 | 20 |
| 14 pt | 19 | 23 |

To support both VGA and SVGA you need ppem sizes: 11, 13, 16, 17, 19, 20, 23.

#### Build process

1. Create an outline `.ttf` with placeholder glyphs on calculated advance widths
2. Use Fontographer to import bitmaps at correct Windows point sizes (Mac 11 pt = Windows VGA 8 pt equivalent)
3. Generate a `.ttf` for PC and a `.bdf` bitmap format (Macintosh encoding)
4. Edit the `.bdf` in a text editor: remove entries for `space` or glyphs without images
5. Use `sbit32.exe` to import the `.bdf` into the `.ttf`
6. Verify vertical metrics in the `OS/2` table (`WinAscent` and `WinDescent`); calculate from tallest/lowest characters to avoid bitmap clipping

## Related

- [`Disable Font Smoothing In Windows`](/windows/display/disable-font-smoothing/)
- [`Lenovo Tablet Mode And Sensor Fixes`](/windows/troubleshooting/lenovo-tablet-mode-and-sensor-fixes/)
