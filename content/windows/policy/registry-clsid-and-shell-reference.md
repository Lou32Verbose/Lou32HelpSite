---
title: Registry CLSID And Shell Reference
slug: /windows/policy/registry-clsid-and-shell-reference/
summary: Reference for Windows shell CLSIDs, shortcut registry keys, component registration, and Take Ownership shell commands.
topic: windows/policy
type: reference
tags: [windows, registry, clsid, shell, regsvr32, takeown]
aliases: [clsid ifontcache, clsid shell locations win8, windows help clsid, show lnk extension, regsvr32 register ocx, take ownership powershell, get all software clsid]
platforms: [windows]
related:
  - /windows/policy/windows-11-feature-control/
  - /windows/shortcuts/settings-and-shell-shortcuts/
status: published
updated: 2026-03-21
---

## Synopsis

Lookup reference for Windows shell location CLSIDs, shortcut-related registry values, component registration with `regsvr32`, and PowerShell commands for registry inspection and ownership.

## Syntax

```text
shell:::{CLSID-GUID}
regsvr32 /i:install "filename.ocx"
```

## Parameters/Flags

- `shell:::` prefix: opens a shell folder by CLSID from Explorer or Run dialog
- `regsvr32 /i:install`: registers an OCX/ActiveX control with install mode
- `NeverShowExt`: REG_SZ value that hides file extensions for a file type

## Examples

### Known CLSIDs

| Component | CLSID |
|-----------|-------|
| IFontCache | `{B0D17FC2-7BC4-11d1-BDFA-00C04FA31009}` |
| Windows Help ActiveX Control (Hhctrl.ocx) | `{52A2AAAE-085D-4187-97EA-8C30DB990436}` |
| Shortcut (.lnk) handler | `{00021401-0000-0000-C000-000000000046}` |

### Shell Location CLSIDs

Open any of these from the Run dialog or Explorer address bar using `shell:::{GUID}`:

| Location | Command |
|----------|---------|
| Action Center | `shell:::{BB64F8A7-BEE7-4E1A-AB8D-7D8273F7FDB6}` |
| Add Network Place | `shell:::{D4480A50-BA28-11d1-8E75-00C04FA31A86}` |
| Administrative Tools | `shell:::{D20EA4E1-3957-11d2-A40B-0C5020524153}` |
| All Control Panel Items | `shell:::{21EC2020-3AEA-1069-A2DD-08002B30309D}` |
| All Settings | `shell:::{F90C627B-7280-45DB-BC26-CCE7BDD620A4}` |
| All Tasks (God Mode) | `shell:::{ED7BA470-8E54-465E-825C-99712043E01C}` |
| Applications | `shell:::{4234d49b-0245-4df3-b780-3893943456e1}` |
| AutoPlay | `shell:::{9C60DE1E-E5FC-40f4-A487-460851A8D915}` |
| Biometric Devices | `shell:::{0142e4d0-fb7a-11dc-ba4a-000ffe7ab428}` |
| BitLocker Drive Encryption | `shell:::{D9EF8727-CAC2-4e60-809E-86F80A666C91}` |
| Bluetooth Devices | `shell:::{28803F59-3A75-4058-995F-4EE5503B023C}` |
| Computer | `shell:::{20D04FE0-3AEA-1069-A2D8-08002B30309D}` |
| Control Panel | `shell:::{26EE0668-A00A-44D7-9371-BEB064C98683}` |
| Credential Manager | `shell:::{1206F5F1-0569-412C-8FEC-3204630DFB70}` |
| Default Programs | `shell:::{17cd9488-1228-4b2f-88ce-4298e93e0966}` |
| Devices and Printers | `shell:::{A8A91A66-3A7D-4424-8D24-04E180695C7A}` |
| Display | `shell:::{C555438B-3C23-4769-A71F-B6D3D9B6053A}` |
| Ease of Access Center | `shell:::{D555645E-D4F8-4c29-A827-D93C859C4F2A}` |
| Favorites | `shell:::{323CA680-C24D-4099-B94D-446DD2D7249E}` |
| File History | `shell:::{F6B6E965-E9B2-444B-9286-10C9152EDBC5}` |
| Folder Options | `shell:::{6DFD7C5C-2451-11d3-A299-00C04F8EF6AF}` |
| Font Settings | `shell:::{93412589-74D4-4E4E-AD0E-E0CB621440FD}` |
| Homegroup | `shell:::{6785BFAC-9D2D-4be5-B7E2-59937E8FB80A}` |
| Installed Updates | `shell:::{d450a8a1-9568-45c7-9c0e-b4f9fb4537bd}` |
| Language | `shell:::{BF782CC9-5A52-4A17-806C-2A894FFEEAC5}` |
| Libraries | `shell:::{031E4825-7B94-4dc3-B131-E946B44C8DD5}` |
| My Documents | `shell:::{450D8FBA-AD25-11D0-98A8-0800361B1103}` |
| Network | `shell:::{208D2C60-3AEA-1069-A2D7-08002B30309D}` |
| Network and Sharing Center | `shell:::{8E908FC9-BECC-40f6-915B-F4CA0E70D03D}` |
| Network Connections | `shell:::{7007ACC7-3202-11D1-AAD2-00805FC1270E}` |
| Notification Area Icons | `shell:::{05d7b0f4-2121-4eff-bf6b-ed3f69b894d9}` |
| Performance Info and Tools | `shell:::{78F3955E-3B90-4184-BD14-5397C15F1EFC}` |
| Personalization | `shell:::{ED834ED6-4B5A-4bfe-8F11-A626DCB6A921}` |
| Power Options | `shell:::{025A5937-A6BE-4686-A844-36FE4BEC8B6D}` |
| Printers | `shell:::{2227A280-3AEA-1069-A2DE-08002B30309D}` |
| Programs and Features | `shell:::{7b81be6a-ce2b-4676-a29e-eb907a5126c5}` |
| Recent Places | `shell:::{22877a6d-37a1-461a-91b0-dbda5aaebc99}` |
| Recovery | `shell:::{9FE63AFD-59CF-4419-9775-ABCC3849F861}` |
| Recycle Bin | `shell:::{645FF040-5081-101B-9F08-00AA002F954E}` |
| Removable Storage Devices | `shell:::{a6482830-08eb-41e2-84c1-73920c2badb9}` |
| Run | `shell:::{2559a1f3-21d7-11d4-bdaf-00c04f60b9f0}` |
| Show Desktop | `shell:::{3080F90D-D7AD-11D9-BD98-0000947B0257}` |
| Storage Spaces | `shell:::{F942C606-0914-47AB-BE56-1321B8035096}` |
| Switch Between Windows | `shell:::{3080F90E-D7AD-11D9-BD98-0000947B0257}` |
| Sync Center | `shell:::{9C73F5E5-7AE7-4E32-A8E8-8D23B85255BF}` |
| System | `shell:::{BB06C0E4-D293-4f75-8A90-CB05B6477EEE}` |
| Taskbar | `shell:::{0DF44EAA-FF21-4412-828E-260A8728E7F1}` |
| Troubleshooting | `shell:::{C58C4893-3BE0-4B45-ABB5-A63E4B8C8651}` |
| User Accounts | `shell:::{60632754-c523-4b62-b45c-4172da012619}` |
| Windows Defender | `shell:::{D8559EB9-20C0-410E-BEDA-7ED416AECC2A}` |
| Windows Features | `shell:::{67718415-c450-4f3c-bf8a-b487642dc39b}` |
| Windows Firewall | `shell:::{4026492F-2F69-46B8-B9BF-5654FC07E423}` |
| Windows Mobility Center | `shell:::{5ea4f148-308c-46d7-98a9-49041b1dd468}` |
| Windows Update | `shell:::{36eef7db-88ad-4e81-ad49-0e313f0c35f8}` |

### Shortcut (.lnk) Registry Reference

The `NeverShowExt` value at `HKEY_CLASSES_ROOT\Lnkfile` controls whether `.lnk` extensions are displayed:

- **Present (REG_SZ):** extensions are hidden (default behavior)
- **Deleted:** the system shows `.lnk` extension on shortcuts

To restore default behavior, re-add `NeverShowExt` as a REG_SZ entry.

### Register an OCX/ActiveX File

```text
regsvr32 /i:install "msflxgrd.ocx"
```

### Take Ownership Shell Command

PowerShell command used by the "Take Ownership" context menu entry (registered at `HKCR\Directory\shell\TakeOwnership`):

```powershell
powershell.exe -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \`"%1\`" /r /d Y /skipsl && icacls \`"%1\`" /grant *S-1-3-4:F /t /c /l & pause' -Verb runAs"
```

### Get All Software CLSIDs from Registry

```powershell
Get-ItemProperty -Path "HKLM:\SOFTWARE\Classes\CLSID\*" |
  Select-Object PSChildName, '(default)' |
  Format-Table -Auto *
```

## Related

- [`Windows 11 Feature Control`](/windows/policy/windows-11-feature-control/)
- [`Settings And Shell Shortcuts`](/windows/shortcuts/settings-and-shell-shortcuts/)
