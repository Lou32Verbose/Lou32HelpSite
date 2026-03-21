---
title: OOBE Bypass Methods
slug: /windows/install/oobe-bypass-methods/
summary: Workarounds for bypassing Windows 11 online account requirements and creating dual-mode bootable USB drives with Rufus.
topic: windows/install
type: recipe
tags: [windows, oobe, bypass, rufus, local-account]
aliases: [bypass nro workaround, ms-cxh localonly, rufus uefi mbr mode]
platforms: [windows]
related:
  - /windows/install/windows-install-and-oobe-notes/
  - /windows/policy/product-key-and-activation/
status: published
updated: 2026-03-21
---

## Goal

Bypass the Windows 11 OOBE online account requirement to create a local account, and configure Rufus to create USB drives that work in both UEFI and MBR boot modes.

## Prerequisites

- Access to the Windows 11 OOBE screen (during initial setup or after reset)
- Rufus (for the bootable USB method)

## Steps

1. Reach the OOBE screen where Microsoft account sign-in is required.
2. Use the alternate bypass command to switch to local account creation.
3. For bootable USB creation, enable Rufus dual-mode before writing the image.

## Commands

### Bypass Online Account Requirement (April 2025+)

Microsoft disabled the `BYPASSNRO` command as of April 2025. Use this alternate command instead:

Open a command prompt during OOBE (press Shift+F10) and run:

```text
start ms-cxh:localonly
```

This launches the local account creation flow directly.

### Rufus UEFI and MBR Dual Mode

While Rufus is running, press **Alt+E** to activate a mode that creates a flash drive bootable in both UEFI and legacy BIOS (MBR) modes.

The mode stays active until you press **Alt+E** again to disable it.

## Verification

- After running the bypass command, the OOBE should present a local account creation screen.
- For Rufus, verify the mode is active by checking the status bar or partition scheme options.

## Related

- [`Windows Install And OOBE Notes`](/windows/install/windows-install-and-oobe-notes/)
- [`Product Key And Activation Reference`](/windows/policy/product-key-and-activation/)
