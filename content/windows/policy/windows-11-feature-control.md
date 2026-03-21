---
title: Windows 11 Feature Control
slug: /windows/policy/windows-11-feature-control/
summary: Registry and command-line methods for disabling unwanted Windows features, telemetry, and auto-installed apps.
topic: windows/policy
type: recipe
tags: [windows, registry, policy, cortana, outlook, telemetry, uac]
aliases: [block new outlook, disable cortana, disable auto installing apps, compattelrunner disable, classic alt tab, export start menu layout, hyper-v check, disable uac for app]
platforms: [windows]
related:
  - /windows/policy/disable-windows-recall/
  - /windows/policy/local-group-policy-editor-on-home/
  - /windows/maintenance/dism-appx-and-system-repair/
status: published
updated: 2026-03-21
---

## Goal

Disable or remove unwanted Windows 11 features, telemetry components, and auto-installed apps using registry edits, scheduled-task changes, and PowerShell commands.

## Prerequisites

- Elevated Command Prompt or PowerShell session for most operations
- `regedit.exe` access for manual registry edits
- PsExec (Sysinternals) only needed for TrustedInstaller-level task changes

## Steps

1. Identify the feature or component to disable.
2. Apply the appropriate registry value or scheduled-task change from the commands below.
3. Restart Windows or sign out and back in for changes to take effect.
4. Verify the feature is disabled.

## Commands

### Block New Outlook (Windows 11)

Windows 11 23H2+ pre-installs the new Outlook app. Remove it and prevent reinstallation:

```powershell
Remove-AppxProvisionedPackage -AllUsers -Online -PackageName (Get-AppxPackage Microsoft.OutlookForWindows).PackageFullName
```

Additionally, remove the Windows orchestrator registry value if on a build before the March 2024 non-security preview:

```text
reg delete "HKLM\SOFTWARE\Microsoft\WindowsUpdate\Orchestrator\UScheduler_Oobe\OutlookUpdate" /f
```

### Block New Outlook (Windows 10)

Prevent installation before it happens by adding a blocker value:

```text
reg add "HKLM\SOFTWARE\Microsoft\WindowsUpdate\Orchestrator\UScheduler_Oobe" /v BlockedOobeUpdaters /t REG_SZ /d "[\"MS_Outlook\"]" /f
```

To remove after it has already been installed:

```powershell
Remove-AppxProvisionedPackage -AllUsers -Online -PackageName (Get-AppxPackage Microsoft.OutlookForWindows).PackageFullName
```

For user-initiated installs (toggle install), use `Remove-AppxPackage` instead:

```powershell
Remove-AppxPackage -AllUsers -Package (Get-AppxPackage Microsoft.OutlookForWindows).PackageFullName
```

### Disable Cortana (All Users)

```powershell
Get-AppxPackage -allusers Microsoft.549981C3F5F10 | Remove-AppxPackage
```

### Disable Automatic App Installation

Open `regedit.exe` and navigate to:

```text
HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager
```

Create or set the DWORD value `SilentInstalledAppsEnabled` to `0`.

### Disable CompatTelRunner (Microsoft Compatibility Appraiser)

**Method 1 -- Disable the scheduled task:**

```text
schtasks /change /disable /tn "\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser"
```

**Method 2 -- Registry telemetry block:**

```text
reg add "HKLM\Software\Policies\Microsoft\Windows\DataCollection" /v "AllowTelemetry" /t REG_DWORD /d 0 /f
```

**Method 3 -- Full disable (all related tasks and Setting Sync):**

Run in an elevated Command Prompt. If access is denied, prefix with `psexec -S cmd` to run as TrustedInstaller:

```text
reg add HKLM\Software\Policies\Microsoft\Windows\DataCollection /v "AllowTelemetry" /t REG_DWORD /d 0 /f
reg add HKLM\SOFTWARE\Policies\Microsoft\Windows\SettingSync /v "DisableSettingSync" /t REG_DWORD /d 2 /f
reg add HKLM\SOFTWARE\Policies\Microsoft\Windows\SettingSync /v "DisableSettingSyncUserOverride" /t REG_DWORD /d 1 /f
schtasks /change /disable /tn "\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser"
schtasks /change /disable /tn "\Microsoft\Windows\Application Experience\PcaPatchDbTask"
schtasks /change /disable /tn "\Microsoft\Windows\Application Experience\ProgramDataUpdater"
schtasks /change /disable /tn "\Microsoft\Windows\Application Experience\StartupAppTask"
schtasks /change /disable /tn "\Microsoft\Windows\SettingSync\BackgroundUploadTask"
schtasks /change /disable /tn "\Microsoft\Windows\SettingSync\NetworkStateChangeTask"
```

**Method 4 -- Image File Execution Options (last resort):**

```text
reg add "HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\CompatTelRunner.exe" /v Debugger /t REG_SZ /d "%windir%\System32\taskkill.exe" /f
```

### Restore Classic Alt+Tab Interface

Navigate in `regedit.exe` to:

```text
HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer
```

Add a DWORD value `AltTabSettings` set to `1`. Restart Explorer or reboot.

### Export Start Menu Layout to JSON

Windows 11 uses a JSON file for Start Menu configuration (Windows 10 uses XML):

```powershell
Export-StartLayout -Path "C:\LouTemp32\layoutmodification.json"
```

The output file must be named `layoutmodification.json`.

### Check Hyper-V Enablement Status

Run in an elevated Command Prompt (not PowerShell):

```text
bcdedit.exe /enum {current} | find "hypervisorlaunchtype"
```

### Disable UAC Prompt for a Specific App

Use Task Scheduler to bypass UAC for a frequently used application:

1. Open Task Scheduler and right-click **Task Scheduler Library** to create a new folder.
2. Select the new folder and click **Create Task** (not Create Basic Task).
3. Name the task descriptively. Enable **Run with highest privileges** and select your OS under **Configure for**.
4. Under the **Actions** tab, set Action to **Start a program** and browse to the app's `.exe` file.
5. On laptops, under the **Conditions** tab, deselect **Start the task only if the computer is on AC power**.
6. Create a desktop shortcut with target:

```text
C:\Windows\System32\schtasks.exe /RUN /TN "Folder_Name\Task_Name"
```

> **Tip:** Replace spaces with underscores in both the folder and task names to avoid issues.

## Verification

- Confirm registry values exist using `reg query` or `regedit.exe`.
- Restart Windows and check that the disabled feature no longer appears or runs.
- For CompatTelRunner, open Task Scheduler and confirm the tasks show as Disabled.
- For Outlook removal, run `Get-AppxPackage Microsoft.OutlookForWindows` and confirm no results.

## Related

- [`Disable Windows Recall`](/windows/policy/disable-windows-recall/)
- [`Local Group Policy Editor On Windows Home`](/windows/policy/local-group-policy-editor-on-home/)
- [`DISM, AppX, And System Repair`](/windows/maintenance/dism-appx-and-system-repair/)
