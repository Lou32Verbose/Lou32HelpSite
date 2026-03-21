---
title: Product Key And Activation Reference
slug: /windows/policy/product-key-and-activation/
summary: Methods for changing Windows product keys, generic keys for all Windows 11 editions, and registry bypass for hardware requirements.
topic: windows/policy
type: reference
tags: [windows, product-key, activation, slmgr, registry, bypass]
aliases: [change product key win11, generic product keys all win11 editions, windows 2000 retail key, bypass requirements reg file]
platforms: [windows]
related:
  - /windows/install/windows-install-and-oobe-notes/
status: published
updated: 2026-03-21
---

## Synopsis

Reference for Windows product key management: changing keys via command line or Settings, generic keys for clean installs, and registry entries to bypass Windows 11 hardware requirements.

## Syntax

```text
slmgr /ipk <product-key>
slmgr.vbs -ato
```

## Parameters/Flags

- `/ipk`: install product key
- `-ato`: force activation attempt
- `BypassTPMCheck`, `BypassSecureBootCheck`, `BypassRAMCheck`, `BypassStorageCheck`: DWORD values under `HKLM\SYSTEM\Setup\LabConfig` that skip hardware checks

## Examples

### Change Product Key via Command Prompt

Open an elevated Command Prompt and run:

```text
slmgr /ipk XXXXX-XXXXX-XXXXX-XXXXX-XXXXX
```

If Windows does not activate automatically, force it:

```text
slmgr.vbs -ato
```

### Change Product Key via Settings App

1. Press Win+I to open Settings.
2. Navigate to **System > Activation**.
3. Click **Change** under "Change Product Key".
4. Enter the new product key in the dialog.

### Generic Product Keys (Windows 11)

These keys allow installation but do not activate Windows. Useful for clean installs and testing:

| Edition | Key |
|---------|-----|
| Home | `YTMG3-N6DKC-DKB77-7M9GH-8HVX7` |
| Home N | `4CPRK-NM3K3-X6XXQ-RXX86-WXCHW` |
| Home Single Language | `BT79Q-G7N6G-PGBYW-4YWX6-6F4BT` |
| Home Country Specific | `N2434-X9D7W-8PF6X-8DV9T-8TYMD` |
| Pro | `VK7JG-NPHTM-C97JM-9MPGT-3V66T` |
| Pro N | `2B87N-8KFHP-DKV6R-Y2C8J-PKCKT` |
| Pro for Workstations | `DXG7C-N36C4-C4HTG-X4T3X-2YV77` |
| Pro for Workstations N | `WYPNQ-8C467-V2W6J-TX4WX-WT2RQ` |
| Pro Education | `8PTT6-RNW4C-6V7J2-C2D3X-MHBPB` |
| Pro Education N | `GJTYN-HDMQY-FRR76-HVGC7-QPF8P` |
| Education | `YNMGQ-8RYV3-4PGQ3-C8XTP-7CFBY` |
| Education N | `84NGF-MHBT6-FXBX8-QWJK7-DRR8H` |
| Enterprise | `XGVPP-NMH47-7TTHJ-W3FW7-8HV2C` |
| Enterprise N | `WGGHN-J84D6-QYCPR-T7PJ7-X766F` |
| Enterprise G N | `FW7NV-4T673-HF4VX-9X4MM-B4H4T` |

### Legacy Key Reference

| Edition | Key |
|---------|-----|
| Windows 2000 Professional (Retail) | `RM233-2PRQQ-FR4RH-JP89H-46QYB` |

### Bypass Windows 11 Hardware Requirements (Registry)

Save as a `.reg` file and import before or during installation to bypass CPU, RAM, TPM, Secure Boot, and storage checks:

```text
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

## Related

- [`Windows Install And OOBE Notes`](/windows/install/windows-install-and-oobe-notes/)
- [`Windows 11 Feature Control`](/windows/policy/windows-11-feature-control/)
