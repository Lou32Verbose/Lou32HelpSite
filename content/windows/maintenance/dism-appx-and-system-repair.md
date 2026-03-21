---
title: DISM, AppX, And System Repair
slug: /windows/maintenance/dism-appx-and-system-repair/
summary: Practical repair workflow for DISM cleanup, AppX resets, SFC log review, and common Windows maintenance tasks.
topic: windows/maintenance
type: recipe
tags: [windows, dism, sfc, appx, repair]
aliases: [repair windows store apps, dism component cleanup, get sfc details]
platforms: [windows]
related:
  - /windows/policy/local-group-policy-editor-on-home/
status: published
updated: 2026-03-21
---

## Goal

Use a small set of trusted maintenance commands to repair system files, reset AppX packages, manage offline images, and perform common Windows servicing tasks.

## Prerequisites

- Elevated Command Prompt or PowerShell session
- Time to reboot if cleanup or repair commands require it
- For offline image operations: a mounted WIM image (typically at `C:\mount`)
- For Safe Mode operations: boot into Safe Mode before running DISM component cleanup

## Steps

1. Run health-check and cleanup commands first.
2. Repair the image with DISM when corruption is reported.
3. Run `sfc /scannow` and inspect the CBS log if issues remain.
4. Manually replace corrupted system files when SFC cannot fix them automatically.
5. Reset or re-register AppX packages only when the problem points to Store or package registration issues.
6. Use offline image commands when customizing a WIM for deployment.

## Commands

### System File Checker

Run SFC to scan and repair protected system files:

```text
sfc /scannow
```

Extract SFC details from the CBS log using an elevated Command Prompt:

```text
findstr /c:"[SR]" %windir%\Logs\CBS\CBS.log >"%userprofile%\Desktop\sfcdetails.txt"
```

The same extraction via PowerShell:

```powershell
Select-String -Path "$env:windir\Logs\CBS\CBS.log" -Pattern '\[SR\]' |
  Set-Content "$env:USERPROFILE\Desktop\sfc-details.txt"
```

### DISM Component Cleanup

Basic online health check and repair:

```powershell
dism /online /cleanup-image /scanhealth
dism /online /cleanup-image /restorehealth
```

Determine whether the Component Store needs a cleanup:

```text
Dism.exe /Online /Cleanup-Image /AnalyzeComponentStore
```

**Full cleanup sequence (best run in Safe Mode to avoid locked components):**

First, clean up the NTFS file system:

```text
fsutil resource setautoreset true c:\&fsutil usn deletejournal /d /n c:
```

Then run the DISM cleanup sequence:

```text
Dism.exe /online /Cleanup-Image /StartComponentCleanup
Dism.exe /Online /Cleanup-Image /RestoreHealth
Dism.exe /Online /Cleanup-Image /StartComponentCleanup /ResetBase
```

Reboot back to Normal Mode when all commands have completed.

> **Tip:** Optionally run `sfc /scannow` first before the DISM sequence to ensure the component store system files are correct.

### Offline Image Operations

#### Mounting an Image

Mount a WIM image for modification (elevated prompt required):

```text
DISM /Mount-image /imagefile:<path_to_Image_file> {/Index:<image_index> | /Name:<image_name>} /MountDir:<target_mount_directory> [/readonly] [/optimize]
```

> The path must point to a `.wim` file. If starting from an ISO, extract the WIM first -- you cannot modify an ISO directly.

#### Image Info and Analysis

```powershell
# Get detailed info about all images in a WIM
Get-WindowsImage -ImagePath "install.wim" |
  Format-Table ImageIndex, ImageName, ImageSize, Architecture

# Check WIM file health and get compression info
Get-WindowsImage -ImagePath "install.wim" -Index 1 |
  Select-Object ImageName, ImageSize, Architecture, InstallationType, Version

# List all currently mounted images
Get-WindowsImage -Mounted

# Get Windows edition info for the running OS
DISM /Online /Get-CurrentEdition

# Check if a mounted image needs reboot after servicing
Get-WindowsImage -Path "C:\mount" |
  Select-Object ImageName, RestartRequired
```

#### Validation and Repair (Offline)

```powershell
# Check offline image health
DISM /Image:"C:\mount" /Cleanup-Image /CheckHealth

# Scan and repair offline image
DISM /Image:"C:\mount" /Cleanup-Image /ScanHealth
DISM /Image:"C:\mount" /Cleanup-Image /RestoreHealth

# Verify WIM integrity
DISM /Get-WimInfo /WimFile:"install.wim" /CheckIntegrity
```

#### Export a Modified Image

```powershell
Export-WindowsImage -SourceImagePath "C:\temp\install.wim" `
  -SourceIndex 1 `
  -DestinationImagePath "C:\temp\install_modified.wim" `
  -CompressionType Maximum
```

#### Package Update and Management (Offline)

```powershell
# Add all MSU update files from a directory
Get-ChildItem "C:\updates" -Filter "*.msu" |
  ForEach-Object { Add-WindowsPackage -Path "C:\mount" -PackagePath $_.FullName -NoRestart }

# Remove a specific KB update
Get-WindowsPackage -Path "C:\mount" |
  Where-Object { $_.PackageName -like "*KB5012345*" } |
  Remove-WindowsPackage -Path "C:\mount" -NoRestart

# List superseded packages that can be cleaned up
Get-WindowsPackage -Path "C:\mount" |
  Where-Object { $_.PackageState -eq "Superseded" }
```

#### Offline Registry Edits

```powershell
# Set multiple registry values for offline image (mount registry hive first)
@{
  "BypassNRO"             = 1
  "BypassTPMCheck"        = 1
  "BypassSecureBootCheck" = 1
} | ForEach-Object {
  $_.GetEnumerator() | ForEach-Object {
    reg add "HKLM\OFFLINE\Setup\LabConfig" /v $_.Key /t REG_DWORD /d $_.Value /f
  }
}

# Disable Windows Defender in offline registry
reg add "HKLM\OFFLINE\SOFTWARE\Policies\Microsoft\Windows Defender" /v DisableAntiSpyware /t REG_DWORD /d 1 /f

# Set default user profile settings (load and unload default hive)
reg load HKU\Default "C:\mount\Users\Default\NTUSER.DAT"
reg add "HKU\Default\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v HideFileExt /t REG_DWORD /d 0 /f
reg unload HKU\Default
```

#### File Operations (Offline)

```powershell
# Copy files into the mounted image
Copy-Item "C:\custom\*" -Destination "C:\mount\Windows\System32" -Recurse -Force

# Remove Windows.old folder from image
Remove-Item "C:\mount\Windows.old" -Recurse -Force -ErrorAction SilentlyContinue

# Replace a system file in a mounted image
takeown /f "C:\mount\Windows\System32\utilman.exe"
icacls "C:\mount\Windows\System32\utilman.exe" /grant administrators:F
copy "C:\temp\cmd.exe" "C:\mount\Windows\System32\utilman.exe" /y
```

#### Boot Configuration (Offline)

```powershell
# Set default boot entry
bcdedit /store "C:\mount\Boot\BCD" /default {current}

# Add boot menu timeout
bcdedit /store "C:\mount\Boot\BCD" /timeout 10

# Export boot configuration
bcdedit /store "C:\mount\Boot\BCD" /export "C:\temp\bcd_backup"
```

### AppX Package Management

#### Re-register All AppX Packages (Current User)

Repairs broken Store apps and fixes Settings display glitches:

```powershell
Get-AppXPackage | Foreach {
  Add-AppxPackage -DisableDevelopmentMode -Register "$($_.InstallLocation)\AppXManifest.xml"
}
```

#### Re-register All AppX Packages (All Users)

```powershell
Get-AppxPackage -AllUsers | Foreach {
  Add-AppxPackage -Register "$($_.InstallLocation)\AppXManifest.xml" -DisableDevelopmentMode
}
```

#### Re-register a Specific AppX Package (e.g. Settings / Immersive Control Panel)

```powershell
Get-AppXPackage -AllUsers -Name windows.immersivecontrolpanel | Foreach {
  Add-AppxPackage -DisableDevelopmentMode -Register "$($_.InstallLocation)\AppXManifest.xml" -Verbose
}
```

#### List Provisioned Packages

```powershell
Get-AppXProvisionedPackage -Online | Select-Object PackageName
```

#### Remove Provisioned Packages by Name (Online)

Build an array of display names and remove them in bulk:

```powershell
$appname = @(
  "Microsoft.3DBuilder"
  "Microsoft.BingNews"
  "Microsoft.BingWeather"
)

ForEach ($app in $appname) {
  Get-AppxProvisionedPackage -Online |
    Where-Object { $_.PackageName -like $app } |
    Remove-AppxProvisionedPackage -Online -ErrorAction SilentlyContinue
}
```

#### Remove Provisioned Packages from an Offline Image (DISM)

Quick removal by pattern in PowerShell:

```powershell
# Remove all Xbox-related apps
Get-AppxProvisionedPackage -Path "C:\mount" |
  Where-Object { $_.DisplayName -like "*Xbox*" } |
  Remove-AppxProvisionedPackage -Path "C:\mount"

# Remove common bloatware apps
"Microsoft.BingWeather",
"Microsoft.GetHelp",
"Microsoft.Getstarted",
"Microsoft.ZuneMusic",
"Microsoft.ZuneVideo" | ForEach-Object {
  $name = $_
  Remove-AppxProvisionedPackage -Path "C:\mount" -PackageName (
    Get-AppxProvisionedPackage -Path "C:\mount" |
      Where-Object { $_.DisplayName -eq $name }
  ).PackageName
}
```

Individual DISM command-line removal (example list for a mounted image at `C:\mount`):

```text
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.BingWeather_4.25.20211.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.GetHelp_10.1706.13331.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.Getstarted_8.2.22942.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.Microsoft3DViewer_6.1908.2042.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.MicrosoftOfficeHub_18.1903.1152.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.MicrosoftSolitaireCollection_4.4.8204.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.MicrosoftStickyNotes_3.6.73.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.MixedReality.Portal_2000.19081.1301.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.MSPaint_2019.729.2301.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.Office.OneNote_16001.12026.20112.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.People_2019.305.632.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.ScreenSketch_2019.904.1644.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.SkypeApp_14.53.77.0_neutral_~_kzf8qxf38zg5c
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.Windows.Photos_2019.19071.12548.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.WindowsAlarms_2019.807.41.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.WindowsCalculator_2020.1906.55.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.WindowsCamera_2018.826.98.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.windowscommunicationsapps_16005.11629.20316.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.WindowsFeedbackHub_2019.1111.2029.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.WindowsMaps_2019.716.2316.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.WindowsSoundRecorder_2019.716.2313.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.WindowsStore_11910.1002.513.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.Xbox.TCUI_1.23.28002.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.XboxApp_48.49.31001.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.XboxGameOverlay_1.46.11001.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.XboxGamingOverlay_2.34.28001.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.XboxIdentityProvider_12.50.6001.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.XboxSpeechToTextOverlay_1.17.29001.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.YourPhone_2019.430.2026.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.ZuneMusic_2019.19071.19011.0_neutral_~_8wekyb3d8bbwe
dism /Image:C:\mount /Remove-ProvisionedAppxPackage /PackageName:Microsoft.ZuneVideo_2019.19071.19011.0_neutral_~_8wekyb3d8bbwe
```

### Feature Management

List and manage Windows optional features:

```powershell
# List all optional features on the running OS
Get-WindowsOptionalFeature -Online

# Enable an optional feature (e.g. TelnetClient)
Enable-WindowsOptionalFeature -Online -FeatureName TelnetClient
```

Offline feature management (mounted image):

```powershell
# Enable multiple Windows features at once
"IIS-WebServerRole",
"IIS-HttpCompressionStatic",
"IIS-HttpCompressionDynamic" | ForEach-Object {
  Enable-WindowsOptionalFeature -Path "C:\mount" -FeatureName $_ -All
}

# Disable Windows features
"WindowsMediaPlayer",
"Internet-Explorer-Optional-amd64" | ForEach-Object {
  Disable-WindowsOptionalFeature -Path "C:\mount" -FeatureName $_
}

# List all disabled optional features
Get-WindowsOptionalFeature -Path "C:\mount" |
  Where-Object { $_.State -eq "Disabled" } |
  Select-Object FeatureName

# Enable .NET Framework 3.5 with source media
Enable-WindowsOptionalFeature -Path "C:\mount" -FeatureName "NetFx3" -Source "D:\sources\sxs" -All
```

### Driver Management

```powershell
# Add all drivers from a folder recursively (offline image)
Add-WindowsDriver -Path "C:\mount" -Driver "C:\drivers" -Recurse -ForceUnsigned

# Export drivers from the running system
Export-WindowsDriver -Online -Destination "C:\exported_drivers"

# Remove a specific driver by vendor (offline image)
Get-WindowsDriver -Path "C:\mount" |
  Where-Object { $_.ProviderName -eq "VendorName" } |
  Remove-WindowsDriver -Path "C:\mount"
```

### Pagefile Management

PowerShell commands to configure the system pagefile:

```powershell
# Disable "Automatically manage paging file size for all drives"
Get-WmiObject Win32_ComputerSystem -EnableAllPrivileges | ForEach-Object {
  $_.AutomaticManagedPagefile = $False
  $_.Put()
} | Out-Null

# Remove pagefile for drive D:
(Get-WmiObject Win32_PageFileSetting | Where-Object { $_.Name -eq "D:\pagefile.sys" }).Delete()

# Set pagefile for drive C: to system managed (InitialSize=0, MaximumSize=0)
Set-WmiInstance -Class Win32_PageFileSetting `
  -Arguments @{ Name = "C:\pagefile.sys"; InitialSize = 0; MaximumSize = 0 } `
  -EnableAllPrivileges | Out-Null
```

### Disable MRT

Three methods to prevent the Malicious Software Removal Tool from running.

**Method 1 -- Registry (recommended):**

Navigate to or create this key:

```text
HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\MRT
```

Add a DWORD value:

```text
Name:  DontOfferThroughWUAU
Value: 1
```

Or apply via an elevated Command Prompt:

```text
reg add "HKLM\SOFTWARE\Policies\Microsoft\MRT" /v DontOfferThroughWUAU /t REG_DWORD /d 1 /f
```

**Method 2 -- Group Policy:**

Open the Group Policy Editor and navigate to:

```text
Computer Configuration > Administrative Templates > Windows Components > Microsoft Malicious Software Removal Tool
```

Set the policy to **Disabled** to prevent MRT from being offered through Windows Update.

**Method 3 -- Delete MRT.exe (not recommended):**

You can manually delete `%windir%\System32\MRT.exe`, but it may reappear after updates if the registry or Group Policy methods above are not also applied.

### Manual System File Replacement

When SFC cannot automatically fix a corrupted file, replace it manually.

**Step 1 -- Take administrative ownership:**

```text
takeown /f <Path_And_File_Name>
```

Example:

```text
takeown /f C:\Windows\System32\jscript.dll
```

**Step 2 -- Grant administrators full access:**

```text
icacls <Path_And_File_Name> /grant administrators:F
```

Example:

```text
icacls C:\Windows\System32\jscript.dll /grant administrators:F
```

**Step 3 -- Replace the file with a known good copy:**

```text
copy <Source_File> <Destination>
```

Example:

```text
copy E:\temp\jscript.dll C:\Windows\System32\jscript.dll
```

## Verification

- Review DISM and SFC exit messages before moving to the next repair step.
- After component cleanup in Safe Mode, reboot to Normal Mode and confirm the system starts cleanly.
- Check whether the broken app or system feature reproduces after reboot.
- Keep exported log snippets (`sfcdetails.txt`) when comparing multiple repair attempts.
- After AppX re-registration, open the affected app or Settings page to confirm the fix.
- For offline images, run `DISM /Get-WimInfo /WimFile:"install.wim" /CheckIntegrity` after exporting.

## Related

- [`Windows Install And OOBE Notes`](/windows/install/windows-install-and-oobe-notes/)
