---
title: WinPE Setup And Components
slug: /windows/install/winpe-setup-and-components/
summary: Complete guide to building a custom WinPE bootable USB with optional components, GUI support, and BitLocker recovery for Windows 11.
topic: windows/install
type: recipe
tags: [windows, winpe, dism, adk, bitlocker, deployment, recovery]
aliases: [winpe optional components, winpe command list, winpe customizable image guide, winpe usb bootable, winpe gui support, dart recovery image creation script]
platforms: [windows]
related:
  - /windows/install/windows-install-and-oobe-notes/
  - /windows/maintenance/dism-appx-and-system-repair/
status: published
updated: 2026-03-21
---

## Goal

Build a customized WinPE bootable USB from the Windows ADK with all relevant optional components, a GUI file explorer, and BitLocker support for Windows 11 emergency recovery and OS deployment.

## Prerequisites

- **Windows ADK for Windows 11** (latest version, currently 10.1.26100.2454 as of December 2024)
- **Windows PE Add-on for ADK** (matching version, installed after the ADK core tools)
- Development PC running Windows 10 or 11 with administrator privileges
- USB flash drive (at least 1 GB) -- will be formatted during creation
- Internet connection for downloading ADK installers and portable tools

Install the ADK Deployment Tools first, then the WinPE add-on. During ADK setup, only the *Deployment Tools* feature is required.

## Steps

1. Create the WinPE working directory with `copype`.
2. Mount the WinPE image with DISM.
3. Add optional component packages in dependency order.
4. Optionally add drivers, portable GUI tools, and startup configuration.
5. Unmount and commit the image.
6. Create the bootable USB with `MakeWinPEMedia`.

## Commands

### Create WinPE Working Directory

Open the **Deployment and Imaging Tools Environment** as administrator:

```text
copype.cmd amd64 C:\WinPE_amd64
```

This creates `C:\WinPE_amd64` with `Media\Sources\boot.wim`, a `Mount` folder, and supporting files. Use `arm64` instead for ARM-based PCs.

### Mount the WinPE Image

```text
Dism /Mount-Image /ImageFile:"C:\WinPE_amd64\Media\Sources\boot.wim" /Index:1 /MountDir:"C:\WinPE_amd64\Mount"
```

### Optional Components Reference

All `.cab` files are located under:

```text
C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\
```

Language packs are in the `en-us\` subfolder. Add the neutral cab first, then the language cab.

**Dependency order for PowerShell support:** WMI -> NetFX -> Scripting -> PowerShell

| Component | Purpose | Dependencies |
|-----------|---------|-------------|
| WinPE-WMI | WMI providers (required by most others) | None |
| WinPE-NetFX | .NET Framework 4.5 subset | WMI |
| WinPE-Scripting | Windows Script Host (VBScript/JScript) | None |
| WinPE-PowerShell | PowerShell cmdlets and scripting | WMI, NetFX, Scripting |
| WinPE-DismCmdlets | DISM PowerShell module | WMI, NetFX, Scripting, PowerShell |
| WinPE-StorageWMI | Storage management cmdlets | WMI, NetFX, Scripting, PowerShell |
| WinPE-SecureBootCmdlets | Secure Boot variable management | WMI, NetFX, Scripting, PowerShell |
| WinPE-SecureStartup | BitLocker/TPM support and unlock wizard | WMI |
| WinPE-EnhancedStorage | Encrypted/self-encrypting drive support | None |
| WinPE-HTA | HTML Application hosting (simple GUI) | Scripting |
| WinPE-FMAPI | File recovery API for deleted files on NTFS | None |
| WinPE-MDAC | ODBC/OLE DB database access | None |
| WinPE-Setup | Core Windows Setup files | None |
| WinPE-Setup-Client | Windows client installer (Win 10/11) | Setup |
| WinPE-Setup-Server | Windows Server installer | Setup |
| WinPE-LegacySetup | Legacy setup support | None |
| WinPE-Dot3Svc | IEEE 802.1X wired authentication | None |
| WinPE-PPPoE | PPP over Ethernet | None |
| WinPE-RNDIS | USB tethering/networking | None |
| WinPE-WDS-Tools | Windows Deployment Services tools | None |
| WinPE-PlatformId | Device Platform Identifier cmdlet | WMI, SecureStartup |
| WinPE-Fonts-Legacy | Additional legacy fonts | None |
| WinPE-WinReCfg | Windows Recovery configuration | None |
| WinPE-PmemCmdlets | Persistent memory cmdlets | None |
| WinPE-HSP-Driver | Microsoft Pluton security processor | None |
| WinPE-x64-Support | x64 emulation on ARM64 WinPE | ARM64 only |

> **Not available as separate ADK packages:** WinPE-Rejuv, WinPE-SRT, WinPE-WiFi-Package (these are WinRE-only).

### Core Component Commands (Minimal Set)

```text
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\WinPE-WMI.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\en-us\WinPE-WMI_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\WinPE-NetFX.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\en-us\WinPE-NetFX_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\WinPE-Scripting.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\en-us\WinPE-Scripting_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\WinPE-PowerShell.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\en-us\WinPE-PowerShell_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\WinPE-StorageWMI.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\en-us\WinPE-StorageWMI_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\WinPE-DismCmdlets.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\en-us\WinPE-DismCmdlets_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\WinPE-HTA.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\en-us\WinPE-HTA_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\WinPE-EnhancedStorage.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"...WinPE_OCs\en-us\WinPE-EnhancedStorage_en-us.cab"
```

### Full Component Template (Copy-Paste Ready)

All components including Setup, networking, security, and extras:

```text
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-WMI.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-WMI_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-NetFX.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-NetFX_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-Scripting.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-Scripting_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-PowerShell.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-PowerShell_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-SecureStartup.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-SecureStartup_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-PlatformId.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-SecureBootCmdlets.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-StorageWMI.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-StorageWMI_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-DismCmdlets.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-DismCmdlets_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-Setup.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-Setup_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-Setup-Client.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-Setup-Client_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-LegacySetup.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-LegacySetup_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-HTA.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-HTA_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-EnhancedStorage.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-EnhancedStorage_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-FMAPI.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-MDAC.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-MDAC_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-Dot3Svc.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-Dot3Svc_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-PmemCmdlets.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-PmemCmdlets_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-Fonts-Legacy.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-WDS-Tools.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-WDS-Tools_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-WinReCfg.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-WinReCfg_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\WinPE-PPPoE.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\WinPE-PPPoE_en-us.cab"
Dism /Add-Package /Image:"C:\WinPE\mount" /PackagePath:"C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs\en-us\lp.cab"
```

### Adding a GUI File Explorer

Copy a portable 64-bit file manager (Explorer++, FreeCommander, or Double Commander) into the mounted image:

```text
mkdir "C:\WinPE_amd64\Mount\Util\ExplorerPlusPlus"
copy "C:\Downloads\Explorer++.exe" "C:\WinPE_amd64\Mount\Util\ExplorerPlusPlus\"
```

To auto-launch on boot, create `C:\WinPE_amd64\Mount\Windows\System32\Winpeshl.ini`:

```ini
[LaunchApp]
AppPath = %SYSTEMDRIVE%\Util\ExplorerPlusPlus\Explorer++.exe
```

Alternatively, launch manually from the WinPE command prompt: `X:\Util\ExplorerPlusPlus\Explorer++.exe`

### Configure Startup Script

Edit `C:\WinPE_amd64\Mount\Windows\System32\startnet.cmd` to add commands that run on boot (after `wpeinit`).

### Verify Installed Packages

```text
Dism /Get-Packages /Image:"C:\WinPE_amd64\Mount"
```

### Unmount and Commit

```text
Dism /Unmount-Image /MountDir:"C:\WinPE_amd64\Mount" /Commit
```

### Create Bootable USB

```text
MakeWinPEMedia.cmd /UFD C:\WinPE_amd64 G:
```

Replace `G:` with your USB drive letter. Add `/F` to skip the format confirmation.

### BitLocker Unlock in WinPE

Once booted into WinPE, unlock BitLocker-encrypted drives:

```text
manage-bde -unlock C: -RecoveryPassword 123456-789012-345678-901234-567890-123456-789012-345678
```

Or with a `.bek` recovery key file:

```text
manage-bde -unlock C: -RecoveryKey "E:\RecoveryKey.bek"
```

## Verification

- After unmounting, verify the WinPE image has all packages with `Dism /Get-Packages`.
- Boot from the USB on a test system and confirm the command prompt (or GUI shell) appears.
- Verify PowerShell is available by typing `powershell` at the prompt.
- Test BitLocker unlock if applicable.
- Check that the USB directory contains `Boot`, `EFI`, and `Sources\boot.wim`.

### DaRT Recovery Image Creation Script

Auto-generated PowerShell script from the Microsoft DaRT Recovery Image Wizard. Creates a bootable DaRT 10 WIM and ISO with all diagnostic tools enabled.

```powershell
$ErrorActionPreference = "Stop"
Import-Module "Dism"
Import-Module "Microsoft.Dart"

$WinMediaPath = "D:\"
$DestinationWimPath = "C:\Users\Administrator\Desktop\DaRT10\x64\boot.wim"
$DestinationIsoPath = "C:\Users\Administrator\Desktop\DaRT10\x64\DaRT10.iso"

$WimParentPath = (Split-Path -Path "$DestinationWimPath" -Parent)
$IsoParentPath = (Split-Path -Path "$DestinationIsoPath" -Parent)
$TempMountPath = "$([System.IO.Path]::GetTempPath())\DaRT8Mount_$(Get-Random)"

New-Item -Path $WimParentPath -Type Directory -Force
New-Item -Path $IsoParentPath -Type Directory -Force
New-Item -Path $TempMountPath -Type Directory -Force

Copy-Item "$WinMediaPath\sources\boot.wim" $DestinationWimPath -Force
Set-ItemProperty $DestinationWimPath -Name IsReadOnly -Value $false
Mount-WindowsImage -ImagePath $DestinationWimPath -Path $TempMountPath -Index 2

# Add WinPE packages (EnhancedStorage, WMI, WinReCfg, FMAPI, FontSupport,
# Scripting, NetFx, PowerShell, DismCmdlets + language packs)
$ADKPath = "C:\Program Files (x86)\Windows Kits\10\Assessment and Deployment Kit\Windows Preinstallation Environment\amd64\WinPE_OCs"
@(
  "WinPE-EnhancedStorage.cab", "WinPE-WMI.cab", "WinPE-WinReCfg.cab",
  "WinPE-FMAPI.cab", "WinPE-FontSupport-WinRE.cab", "WinPE-Scripting.cab",
  "en-us\WinPE-EnhancedStorage_en-us.cab", "en-us\WinPE-Scripting_en-us.cab",
  "en-us\WinPE-WMI_en-us.cab", "en-us\WinPE-WinReCfg_en-us.cab",
  "WinPE-NetFx.cab", "en-us\WinPE-NetFx_en-us.cab",
  "WinPE-PowerShell.cab", "en-us\WinPE-PowerShell_en-us.cab",
  "WinPE-DismCmdlets.cab", "en-us\WinPE-DismCmdlets_en-us.cab"
) | ForEach-Object {
  Add-WindowsPackage -Path $TempMountPath -PackagePath "$ADKPath\$_"
}

# Add all DaRT tools
$config = New-DartConfiguration -AddComputerManagement -AddCrashAnalyzer `
  -AddDiskCommander -AddDiskWipe -AddExplorer -AddFileRestore `
  -AddFileSearch -AddHotfixUninstall -AddLocksmith -AddRegistryEditor `
  -AddSfcScan -AddSolutionWizard -AddTcpConfig
$config | Set-DartImage -Path $TempMountPath

Dismount-WindowsImage -Path $TempMountPath -Save
Export-DartImage -IsoPath $DestinationIsoPath -WimPath $DestinationWimPath

# Burn to DVD or USB (uncomment as needed):
# Copy-DartImage -IsoPath $DestinationIsoPath -Drive "G:" -Type DVD
# Copy-DartImage -IsoPath $DestinationIsoPath -Drive "G:" -Type USB

Remove-Item $TempMountPath -Force -Recurse
```

## Related

- [`Windows Install And OOBE Notes`](/windows/install/windows-install-and-oobe-notes/)
- [`DISM, AppX, And System Repair`](/windows/maintenance/dism-appx-and-system-repair/)
