---
title: Disable Windows Recall
slug: /windows/policy/disable-windows-recall/
summary: Registry-based steps for turning off Windows Recall policy values for the current user or the whole machine.
topic: windows/policy
type: recipe
tags: [windows, recall, registry, policy]
aliases: [disable windows recall registry method, disableaidataanalysis]
platforms: [windows]
related:
  - /windows/policy/local-group-policy-editor-on-home/
status: published
updated: 2026-03-20
---

## Goal

Disable Windows Recall by setting the policy registry value that controls data analysis behavior.

## Prerequisites

- Local administrator access for machine-wide policy changes
- `regedit.exe` or a PowerShell session with registry write access

## Steps

1. Open Registry Editor.
2. Navigate to the current-user or local-machine `WindowsAI` policy key.
3. Create the key if it does not already exist.
4. Create or update `DisableAIDataAnalysis` as a `DWORD (32-bit)` value set to `1`.
5. Restart Windows or sign out and back in.

## Commands

Current user:

```powershell
New-Item -Path 'HKCU:\Software\Policies\Microsoft\Windows\WindowsAI' -Force | Out-Null
New-ItemProperty -Path 'HKCU:\Software\Policies\Microsoft\Windows\WindowsAI' -Name DisableAIDataAnalysis -Value 1 -PropertyType DWord -Force | Out-Null
```

Local machine:

```powershell
New-Item -Path 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsAI' -Force | Out-Null
New-ItemProperty -Path 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsAI' -Name DisableAIDataAnalysis -Value 1 -PropertyType DWord -Force | Out-Null
```

## Verification

- Confirm the value exists in both the intended hive and path.
- Restart Windows and review the Recall setting or policy behavior afterward.
- Export the key before changing it if you need an easy rollback point.

## Related

- [`Local Group Policy Editor On Windows Home`](/windows/policy/local-group-policy-editor-on-home/)
