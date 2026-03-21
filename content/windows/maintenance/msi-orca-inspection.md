---
title: MSI Inspection With Orca
slug: /windows/maintenance/msi-orca-inspection/
summary: Tips for examining and editing MSI installer packages with Orca, identifying public properties, and building silent install commands with msiexec.
topic: windows/maintenance
type: reference
tags: [windows, msi, orca, msiexec, installer, deployment]
aliases: [tips for examining msi files with orca, msiexec silent install properties]
platforms: [windows]
related:
  - /windows/install/winpe-setup-and-components/
status: published
updated: 2026-03-21
---

## Synopsis

Reference for inspecting MSI installer databases with Orca (part of the Windows SDK) and building silent install commands using public properties discovered inside the package.

## Syntax

```text
msiexec /i <installer.msi> /q /qn PROPERTY_NAME=VALUE
msiexec /i <installer.msi> TRANSFORMS=transform.mst /qn
```

## Parameters/Flags

- `/i`: install mode
- `/q` or `/qn`: quiet/silent install (no UI)
- `PROPERTY_NAME=VALUE`: set a public MSI property from the command line
- `TRANSFORMS=`: apply an MST transform file that overrides default property values
- `/l*v log.txt`: verbose logging (useful for debugging property values)

## Examples

### Getting Started with Orca

1. Open the `.msi` file in Orca
2. In the left pane, navigate to the **Property** table
3. All properties written in **ALL CAPITALS** are public — these can be set from the command line

### Identify What a Property Controls

Find an MSI with multiple checkboxes or text fields in its GUI installer:

1. Open the MSI in Orca and note the uppercase properties (e.g., `ALLUSERS`, `INSTALLDIR`)
2. Run the installer **without** `/qn` but **with** properties set on the command line:

```text
msiexec /i installer.msi ALLUSERS=1
```

3. Verify the GUI reflects your command-line values (e.g., the "Install for all users" checkbox is pre-checked)

### Silent Install Example

```text
msiexec /i installer.msi ALLUSERS=1 /qn
```

### Using Transform Files

For complex installers with many options, create a transform (`.mst`) file in Orca instead of passing dozens of properties:

```text
msiexec /i installer.msi TRANSFORMS=custom.mst /qn
```

### Common Pitfalls

- Do **not** have the MSI open in Orca while running the installer — the install will fail with an unhelpful error (file lock conflict)
- Use verbose logging (`/l*v install.log`) to identify which properties the installer reads and what values it expects

## Related

- [`WinPE Setup And Components`](/windows/install/winpe-setup-and-components/)
