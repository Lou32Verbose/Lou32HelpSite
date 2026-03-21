---
title: OneDrive Network Drive Mapping
slug: /windows/shortcuts/onedrive-network-drive-mapping/
summary: Steps to map a OneDrive account as a network drive in Windows Explorer using the WebDAV URL and your CID.
topic: windows/shortcuts
type: recipe
tags: [windows, onedrive, network-drive, explorer, webdav]
aliases: [map onedrive as network location in explorer]
platforms: [windows]
related:
  - /windows/shortcuts/ms-settings-and-shell-commands/
status: published
updated: 2026-03-21
---

## Goal

Map your OneDrive storage as a network drive letter in Windows Explorer so it appears alongside local drives.

## Prerequisites

- A Microsoft account with OneDrive access
- Ability to sign in to OneDrive.com in a browser

## Steps

1. **Get your OneDrive CID**: sign in to OneDrive.com and copy the CID from the address bar — the long hex string after `https://d.docs.live.net/`
2. Open **File Explorer** and navigate to **This PC**
3. Click **Computer** tab > **Map network drive**
4. In the **Folder** field enter:

```text
https://d.docs.live.net/<YOUR_CID>
```

5. Check **Connect using different credentials**
6. Click **Finish** and enter your Microsoft account credentials when prompted

## Commands

```text
\\https://d.docs.live.net/<YOUR_CID>
```

This is entered in the Map Network Drive dialog, not at a command prompt.

## Verification

- The mapped drive should appear under **This PC** with a drive letter
- Browse into it to confirm your OneDrive files are accessible

## Related

- [`ms-settings And Shell Commands`](/windows/shortcuts/ms-settings-and-shell-commands/)
