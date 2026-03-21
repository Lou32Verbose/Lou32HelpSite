---
title: Device Driver And Hardware ID Reference
slug: /windows/troubleshooting/device-driver-and-hardware-id/
summary: How to find hardware and device IDs in Device Manager, and how to manage Windows system attributes and desktop.ini files.
topic: windows/troubleshooting
type: reference
tags: [windows, device-manager, hardware-id, attributes, desktop-ini]
aliases: [find vendor hardware device id, system attributes desktop ini]
platforms: [windows]
related:
  - /windows/troubleshooting/lenovo-tablet-mode-and-sensor-fixes/
status: published
updated: 2026-03-21
---

## Synopsis

Reference for finding hardware and vendor IDs in Device Manager and managing Windows file system attributes and `desktop.ini` files.

## Syntax

```text
<BUS>\<VENDOR_ID>&<DEVICE_ID>&<ADDITIONAL_IDS>
ATTRIB [+|-][S|H|R] <filename>
```

## Parameters/Flags

- `VEN_XXXX`: four-character vendor ID (e.g., `VEN_10EC` = Realtek)
- `DEV_XXXX`: four-character device ID
- `+S`: set System attribute
- `+H`: set Hidden attribute
- `-R`: clear Read-only attribute
- `ATTRIB`: Windows command to view and set file attributes

## Examples

### Finding a Hardware ID in Device Manager

1. Open Device Manager (`devmgmt.msc` from Run dialog).
2. Right-click the device and select **Properties**.
3. Select the **Details** tab.
4. Choose **Hardware Ids** from the dropdown list.

Multiple hardware IDs may appear. The top listing is the most specific; lower entries are more generic. A driver can match any of these identifiers.

For example, a Realtek audio device shows `VEN_10EC` -- the four characters after `VEN_` identify the vendor.

Hardware IDs are available for devices with installed drivers, unknown devices, and unrecognized devices.

### System Attributes and desktop.ini

Windows requires the System attribute on `desktop.ini` for the file's settings (custom icons, localized names) to take effect.

**Check current attributes:**

```text
ATTRIB desktop.ini
```

**Set System and Hidden attributes (required for desktop.ini to work):**

```text
ATTRIB +S +H -R desktop.ini
```

**Remove attributes for editing:**

```text
ATTRIB -S -H -R desktop.ini
```

After editing the `desktop.ini` file with a text editor, restore the attributes:

```text
ATTRIB +S +H desktop.ini
```

> **Note:** Windows will not allow setting the System attribute without also setting Hidden. If Read-only is set, clear it first before changing other attributes.

The important lines in `desktop.ini` are typically `LocalizedResourceName=` and `IconResource=`. If a `desktop.ini` stops working after being copied, these values may have been stripped -- edit and restore them manually.

## Related

- [`Lenovo Tablet Mode And Sensor Fixes`](/windows/troubleshooting/lenovo-tablet-mode-and-sensor-fixes/)
