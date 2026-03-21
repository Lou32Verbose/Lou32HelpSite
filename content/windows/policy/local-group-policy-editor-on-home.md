---
title: Local Group Policy Editor On Windows Home
slug: /windows/policy/local-group-policy-editor-on-home/
summary: DISM-based steps for enabling the Local Group Policy Editor package set on Windows Home editions.
topic: windows/policy
type: recipe
tags: [windows, group-policy, dism, home]
aliases: [local group policy on win11 home, gpedit on home]
platforms: [windows]
related:
  - /windows/maintenance/dism-appx-and-system-repair/
status: published
updated: 2026-03-20
---

## Goal

Enable the Group Policy client package set on Windows Home by adding the required servicing packages with DISM.

## Prerequisites

- Administrative PowerShell session
- Windows Home installation with the package files still present under `%SystemRoot%\servicing\Packages`

## Steps

1. Collect the `ClientExtensions` package names.
2. Collect the `ClientTools` package names.
3. Feed both package lists into `dism /add-package`.
4. Reboot or launch `gpedit.msc` after the install completes.

## Commands

```powershell
$pkg = Get-ChildItem $env:SystemRoot\servicing\Packages\Microsoft-Windows-GroupPolicy-ClientExtensions-Package~3*.mum
$pkg += Get-ChildItem $env:SystemRoot\servicing\Packages\Microsoft-Windows-GroupPolicy-ClientTools-Package~3*.mum

foreach ($p in $pkg) {
    dism /online /norestart /add-package:"$p"
}
```

## Verification

- Launch `gpedit.msc` and confirm the editor opens.
- Review the DISM output for package-install failures.
- Reboot if the packages install but the editor is still unavailable.

## Related

- [`DISM, AppX, And System Repair`](/windows/maintenance/dism-appx-and-system-repair/)
