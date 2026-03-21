---
title: Windows Install And OOBE Notes
slug: /windows/install/windows-install-and-oobe-notes/
summary: Consolidated setup notes for Windows media creation, local-account workarounds, generic setup helpers, and initial install tasks.
topic: windows/install
type: recipe
tags: [windows, install, oobe, setup, imaging]
aliases: [bypass windows 11 account creation, windows install from cmd, generic product keys]
platforms: [windows]
related:
  - /windows/shortcuts/settings-and-shell-shortcuts/
status: published
updated: 2026-03-21
---

## Goal

Keep the recurring Windows install and first-run tasks in one place so you can set up install media, complete OOBE, and finish the first login without hunting across older notes.

## Prerequisites

- Windows installation media or ISO
- Administrator access during setup
- A clear rollback point before using unsupported workarounds
- For USB media: a USB drive large enough for the ISO (8 GB minimum recommended)
- For custom images: working directory with several GB of free space

## Steps

1. Prepare install media and confirm the target edition or generic key you intend to use.
2. Start setup and use only the workarounds you actually need for that build.
3. Create the local account or sign-in path you want during OOBE.
4. Finish first-run cleanup such as product-key changes or post-install scripts.

## Commands

### Windows 11 Generic Product Keys (All Editions)

These are generic/default keys used during installation to select an edition. They do not activate Windows.

| Edition | Generic Key |
|---|---|
| Windows 11 Home | `YTMG3-N6DKC-DKB77-7M9GH-8HVX7` |
| Windows 11 Home N | `4CPRK-NM3K3-X6XXQ-RXX86-WXCHW` |
| Windows 11 Home Single Language | `BT79Q-G7N6G-PGBYW-4YWX6-6F4BT` |
| Windows 11 Home Country Specific | `N2434-X9D7W-8PF6X-8DV9T-8TYMD` |
| Windows 11 Pro | `VK7JG-NPHTM-C97JM-9MPGT-3V66T` |
| Windows 11 Pro N | `2B87N-8KFHP-DKV6R-Y2C8J-PKCKT` |
| Windows 11 Pro for Workstations | `DXG7C-N36C4-C4HTG-X4T3X-2YV77` |
| Windows 11 Pro for Workstations N | `WYPNQ-8C467-V2W6J-TX4WX-WT2RQ` |
| Windows 11 Pro Education | `8PTT6-RNW4C-6V7J2-C2D3X-MHBPB` |
| Windows 11 Pro Education N | `GJTYN-HDMQY-FRR76-HVGC7-QPF8P` |
| Windows 11 Education | `YNMGQ-8RYV3-4PGQ3-C8XTP-7CFBY` |
| Windows 11 Education N | `84NGF-MHBT6-FXBX8-QWJK7-DRR8H` |
| Windows 11 Enterprise | `XGVPP-NMH47-7TTHJ-W3FW7-8HV2C` |
| Windows 11 Enterprise N | `WGGHN-J84D6-QYCPR-T7PJ7-X766F` |
| Windows 11 Enterprise G N | `FW7NV-4T673-HF4VX-9X4MM-B4H4T` |

### Bypass OOBE Microsoft Account Requirement

#### Method 1: ms-cxh:localonly Command (April 2025+)

Microsoft disabled the `bypassnro` command as of April 2025. The following replacement achieves the same result. Run from the OOBE screen via `Shift + F10`:

```cmd
start ms-cxh:localonly
```

#### Method 2: Batch Script for Local Account Creation (Build 22557+)

When the setup halts at OOBE, open a console with `Shift + F10`. Replace `D:` with the drive letter of your install media (try `D:\user`, `E:\user`, `F:\user` until you find it):

```cmd
@echo off & title WINDOWS 11 22557+ HOME or PRO setup account without internet connection
set /p "name=Enter your account name: "
net user "%name%" /add
net localgroup Administrators "%name%" /add
reg delete "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\OOBE" /va /f
reg delete "HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon" /f /v AutoLogonSID
reg delete "HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon" /f /v DefaultUserName
net user defaultuser0 /delete
shutdown /l
```

#### Method 3: autounattend.xml Template

Create a file named `autounattend.xml` with the following content and copy it to the root of your Windows 11 installation media. This automatically bypasses the Microsoft Account forced login and creates a local user:

```xml
<?xml version="1.0" encoding="utf-8"?>
<unattend xmlns="urn:schemas-microsoft-com:unattend">
<settings pass="specialize">
<component name="Microsoft-Windows-Deployment" processorArchitecture="amd64" language="neutral" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" publicKeyToken="31bf3856ad364e35" versionScope="nonSxS">
<RunSynchronous>
<RunSynchronousCommand wcm:action="add">
<Order>1</Order>
<Path>reg add HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\OOBE /v BypassNRO /t REG_DWORD /d 1 /f</Path>
</RunSynchronousCommand>
</RunSynchronous>
</component>
</settings>
</unattend>
```

### Bypass Windows 11 Hardware Requirements (.reg File)

This registry file bypasses TPM 2.0, Secure Boot, RAM, and storage checks. It also enables the Insider Beta channel. Save as a `.reg` file and import before or during setup:

```reg
Windows Registry Editor Version 5.00

; Enable Insider 'BETA' Channel Requirements
[HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\WindowsSelfHost\Applicability]
"BranchName"="Beta"
"Ring"="External"
"ContentType"="Mainline"

[HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\WindowsSelfHost\UI\Selection]
"UIBranch"="Beta"
"UIRing"="External"
"UIContentType"="Mainline"

; Bypass CPU Ram SecureBoot TPM 2.0 & Storage Checks
[HKEY_LOCAL_MACHINE\SYSTEM\Setup\LabConfig]
"BypassTPMCheck"=dword:00000001
"BypassSecureBootCheck"=dword:00000001
"BypassRAMCheck"=dword:00000001
"BypassStorageCheck"=dword:00000001

[HKEY_LOCAL_MACHINE\SYSTEM\Setup\MoSetup]
"AllowUpgradesWithUnsupportedTPMOrCPU"=dword:00000001
```

### Enable Windows 11 Dev Channel Builds

Save as a `.reg` file and import to switch to the Dev channel:

```reg
Windows Registry Editor Version 5.00

; For Dev Builds
[HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\WindowsSelfHost\Applicability]
"BranchName"="Dev"
"Ring"="External"
"ContentType"="Mainline"

[HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\WindowsSelfHost\UI\Selection]
"UIContentType"="Mainline"
"UIBranch"="Dev"
"UIRing"="External"
```

### Installing Windows From CMD (UEFI and BIOS)

Supports Windows 7, 8, 10, and 11. Be cautious when dual-booting -- back up existing data first.

Open CMD by pressing `Shift + F10` after booting into setup.

#### Creating Partitions -- MBR

```
diskpart
list disk
select disk (number for main disk)
clean
convert mbr
-----------------------
(Creating recovery is optional)
create part primary size 500
format quick label Recovery
assign letter R
set id 27
-----------------------
create part primary
format quick label Windows
assign letter C
active
exit
```

#### Creating Partitions -- UEFI

```
diskpart
list disk
select disk (number for main disk)
clean
convert gpt
create part efi size 512
format fs fat32 quick
assign letter w
create part msr size 16
-----------------------
(Creating recovery is optional)
create part primary size 500
format quick label Recovery
assign letter R
set id de94bba4-06d1-4d40-a16a-bfd50179d6ac
gpt attributes 0x8000000000000001
-----------------------
create part primary
format quick label Windows
assign letter C
exit
```

#### Navigate to install.wim Directory

```
[letter of installation disk]:
cd sources
```

#### List Available SKUs

```cmd
dism /get-wiminfo /wimfile:[Location to install.wim]
```

#### Deploy the WIM File

Copies the content from install.wim to the target disk:

```cmd
dism /apply-image /imagefile:[Location to install.wim] /index:[SKU Number] /applydir:[Drive letter]:\
```

Example:

```cmd
dism /apply-image /imagefile:d:\sources\install.wim /index:6 /applydir:C:\
```

#### Create Recovery Folders (Optional)

```cmd
md R:\Recovery
xcopy /h C:\Windows\System32\Recovery\Winre.wim R:\Recovery
C:\Windows\System32\Reagentc /Setreimage /Path R:\Recovery /Target C:\Windows
```

#### Create Boot Files

MBR only:

```cmd
bootsect /nt60 C: /force /mbr
```

MBR + UEFI:

```cmd
bcdboot C:\Windows
```

Add the `/s [drive letter to UEFI]:` argument if booting via UEFI and the EFI partition is on a different drive.

#### Bypass OOBE Entirely (Optional) -- Part 1

Run before first boot:

```cmd
reg load HKLM\SOFT C:\Windows\System32\config\SOFTWARE
reg load HKLM\SYS C:\Windows\System32\config\SYSTEM
reg add HKLM\SOFT\Microsoft\Windows\CurrentVersion\Policies\System /v VerboseStatus /t REG_DWORD /d 1 /f
reg add HKLM\SOFT\Microsoft\Windows\CurrentVersion\Policies\System /v EnableCursorSuppression /t REG_DWORD /d 0 /f
reg add HKLM\SYS\Setup /v CmdLine /t REG_SZ /d cmd.exe /f
```

Reboot:

```cmd
wpeutil reboot
```

#### Bypass OOBE Entirely (Optional) -- Part 2

Run Windows Deployment Loader, then enable recovery:

```cmd
oobe\windeploy
Reagentc /enable
Reagentc /Info /Target C:\Windows
```

Add user (when the boot status says "Getting Ready"):

```cmd
net user /add (username) (password)
net localgroup users /add (username)
net localgroup administrators /add (username)
```

Clear OOBE status:

```cmd
reg add HKLM\SYSTEM\Setup /v OOBEInProgress /t REG_DWORD /d 0 /f
reg add HKLM\SYSTEM\Setup /v SetupType /t REG_DWORD /d 0 /f
reg add HKLM\SYSTEM\Setup /v SystemSetupInProgress /t REG_DWORD /d 0 /f
exit
```

Disable VerboseStatus and re-enable CursorSuppression after reaching the desktop (run CMD as admin):

```cmd
reg add HKLM\SOFT\Microsoft\Windows\CurrentVersion\Policies\System /v VerboseStatus /t REG_DWORD /d 0 /f
reg add HKLM\SOFT\Microsoft\Windows\CurrentVersion\Policies\System /v EnableCursorSuppression /t REG_DWORD /d 1 /f
```

### Change Product Key After Install

#### Method 1: Command Prompt (Admin)

```cmd
slmgr /ipk XXXXX-XXXXX-XXXXX-XXXXX-XXXXX
slmgr /ato
```

If activation does not trigger automatically, force it with:

```cmd
slmgr.vbs -ato
```

#### Method 2: Settings App (GUI)

1. Press `Win+I` to open Settings.
2. Choose **System**.
3. Under System, click **Activation**.
4. Click **Change** under "Change Product Key".
5. Enter your product key in the pop-up dialog.

### Windows 11 ISO Direct Download

Direct download link pattern for Windows 11 24H2 English x64:

```
https://software.download.prss.microsoft.com/dbazure/Win11_24H2_English_x64.iso
```

> **Note:** The full URL includes time-limited authentication tokens. Use the official [Microsoft Software Download](https://www.microsoft.com/software-download/windows11) page if the direct link has expired.

### Windows 10 Enterprise Bootable USB Guide

#### Download via Media Creation Tool

Launch the Media Creation Tool from PowerShell with enterprise options:

```powershell
# Change language code and architecture as needed (en-US, x64 shown)
C:\path\to\MediaCreationTool_22H2.exe /Eula Accept /Retail /MediaLangCode en-US /MediaArch x64 /MediaEdition Enterprise
```

Select "Create USB" for a stock installer, or "ISO" to create a customized image with preinstalled drivers.

When asked for a key, use the official KMS key for Windows 10 Enterprise: `NPPR9-FWDCX-D2C8J-H872K-2YT43`

#### Mount the ISO and Export install.wim

> Only needed if creating a customized ISO with a CAB driver package.

Mount the ISO and inspect the image contents:

```powershell
dism.exe /Get-WimInfo /WimFile:D:\sources\install.esd
```

Export the desired index to install.wim:

```powershell
dism.exe /Export-Image /SourceImageFile:D:\sources\install.esd /SourceIndex:3 /DestinationImageFile:"C:\BigWork\install.wim" /Compress:max /CheckIntegrity
```

Verify the exported image:

```powershell
Dism /Get-ImageInfo /ImageFile:C:\BigWork\install.wim
```

#### Mount and Customize the Image

```powershell
Dism /Mount-Image /ImageFile:C:\BigWork\install.wim /Index:1 /MountDir:C:\BigWork\Mount
```

Add a CAB driver package:

```powershell
Dism /Image:C:\BigWork\Mount /Add-Package /PackagePath:"C:\BigWork\Packages\driver-pack.CAB"
```

If the CAB fails with error 0x80070002, extract it first and add drivers from the folder:

```powershell
Expand "C:\BigWork\Packages\driver-pack.CAB" -F:* "C:\BigWork\Drivers"
Dism /Image:C:\BigWork\Mount /Add-Driver /Driver:C:\BigWork\Drivers /Recurse
```

Verify installed drivers:

```powershell
Dism /Image:C:\BigWork\Mount /Get-Drivers
```

Save and unmount:

```powershell
Dism /Unmount-Image /MountDir:C:\BigWork\Mount /Commit
```

#### Create the Bootable USB

Format a USB drive as FAT32 and mark it as active. Copy all files from the original ISO **except** `install.esd` to the USB.

If `install.wim` exceeds 4 GB (FAT32 limit), split it:

```powershell
Dism /Split-Image /ImageFile:C:\BigWork\install.wim /SWMFile:E:\sources\install.swm /FileSize:3800
```

Copy the resulting `.swm` files (or the single `install.wim` if under 4 GB) to the USB `\sources\` folder.

### Rufus: Enable Dual UEFI + MBR Mode

Press `Alt+E` while Rufus is running to activate a mode that creates a flash drive bootable in both UEFI and MBR modes. Press `Alt+E` again to disable it.

### Visual Studio 2015 Download URLs

| Edition | Go.microsoft.com Link | Direct ISO Link |
|---|---|---|
| Community | https://go.microsoft.com/fwlink/?LinkId=615448 | https://download.microsoft.com/download/b/e/d/bedddfc4-55f4-4748-90a8-ffe38a40e89f/vs2015.3.com_enu.iso |
| Professional | https://go.microsoft.com/fwlink/?LinkId=615434 | -- |
| Enterprise | https://go.microsoft.com/fwlink/?LinkId=615436 | -- |

Wayback Machine archive of Community ISO:

```
https://web.archive.org/web/20250210121955/https://download.microsoft.com/download/b/e/d/bedddfc4-55f4-4748-90a8-ffe38a40e89f/vs2015.3.com_enu.iso
```

## Verification

- Confirm the expected local account exists and is in the Administrators group.
- Verify Windows activation and edition after applying any generic or replacement key.
- Keep notes about which bypasses were needed for the exact install image you used.
- After custom image creation, verify drivers with `Dism /Image:<mount> /Get-Drivers` before unmounting.
- Test bootable USB media on the target hardware before wiping any existing installation.

## Related

- [`Local Group Policy Editor On Windows Home`](/windows/policy/local-group-policy-editor-on-home/)
- [`DISM, AppX, And System Repair`](/windows/maintenance/dism-appx-and-system-repair/)
