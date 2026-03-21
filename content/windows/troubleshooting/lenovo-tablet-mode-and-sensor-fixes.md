---
title: Lenovo Tablet Mode And Sensor Fixes
slug: /windows/troubleshooting/lenovo-tablet-mode-and-sensor-fixes/
summary: Troubleshooting notes for Lenovo convertibility, tablet mode, and sensor-monitoring issues that affect shell behavior or display state.
topic: windows/troubleshooting
type: troubleshooting
tags: [windows, lenovo, tablet-mode, sensors, registry]
aliases: [lenovo flex tablet mode fix, disable convertibility control, lenovo ideapad brightness registry fix, disable sensor monitoring service tablet mode, disable win11 tablet mode convertibility reg cmd]
platforms: [windows]
related:
  - /windows/display/display-diagnostics/
status: published
updated: 2026-03-21
---

## Symptoms

- Explorer behaves as if the machine is permanently in tablet mode.
- Rotation or slate-mode state is wrong after boot or resume.
- Convertible-specific sensor services keep reasserting the wrong shell mode.

## Cause

These issues usually come from vendor convertibility services, stale registry values, or a sensor-monitoring service that reports the wrong chassis state.

## Resolution

1. Check whether Lenovo or vendor sensor services are running and whether the problem returns when they are disabled.
2. Inspect the registry values that store convertibility or slate-mode state.
3. Set the shell state back to the expected desktop mode.
4. Reboot and re-test before making additional changes.

Representative registry targets from the legacy notes:

```text
HKEY_LOCAL_MACHINE\SOFTWARE\Lenovo\ImController\Plugins\LenovoModeService
HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\AutoRotation
```

### Brightness Slider Not Working (Lenovo IdeaPad)

If the brightness slider has no effect, set both power-policy brightness values to `0` in the display driver class key:

```text
HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\0000
```

| Value | Data |
|-------|------|
| `DCUserPreferencePolicy` | `0` |
| `ACUserPreferencePolicy` | `0` |

- `DC` = on battery, `AC` = plugged in
- Reboot after making the change

## Verification

- Restart Explorer or reboot and confirm the desktop shell remains stable.
- Fold and unfold the device only if it is safe to do so on that hardware.
- Re-enable services one at a time if you need to isolate the exact trigger.

## Related

- [`Display Diagnostics`](/windows/display/display-diagnostics/)
