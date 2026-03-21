---
title: PowerShell File And Text Recipes
slug: /powershell/filesystem/file-and-text-recipes/
summary: Repeatable PowerShell recipes for renaming files, transforming text, cleaning lists, extracting URLs, hex conversion, directory listing, and environment variable inspection.
topic: powershell/filesystem
type: recipe
tags: [powershell, files, text, csv, paths, hex, urls, msc, environment]
aliases: [convert tab delimited csv, delete files from list, rename and change extensions, hex to ascii, get all urls, list msc files, write dir list, path environment variable]
platforms: [windows, powershell]
related:
  - /powershell/querying/system-inspection-patterns/
status: published
updated: 2026-03-21
---

## Goal

Handle small, repeatable file and text tasks directly in PowerShell without switching to a separate scripting language.

## Prerequisites

- PowerShell session with access to the target files
- A backup or scratch directory when bulk-renaming or deleting

## Steps

1. Identify a narrow file set before changing anything.
2. Preview the transformed output with `Select-Object` or `-WhatIf` when possible.
3. Apply the rename, export, or delete operation.
4. Verify the resulting files and output paths.

## Commands

### Rename Files By Changing Extension

Change files with one extension to another in the current directory:

```powershell
Get-ChildItem *.txt | Rename-Item -NewName { $_.BaseName + ".log" }
```

Using a foreach loop (e.g., `.jpg` to `.jpeg`):

```powershell
foreach ($file in gci *.jpg) { mv $file "$($file.basename).jpeg" }
```

### Convert Tab-Delimited Text To CSV

```powershell
Import-Csv .\input.tsv -Delimiter "`t" |
  Export-Csv .\output.csv -NoTypeInformation
```

With explicit encoding options (for PowerShell Core, use `utf8BOM`; for Windows PowerShell, use `UTF8` and add `-NoTypeInformation`):

```powershell
Import-Csv -Path yourfile.csv -Delimiter "`t" -Encoding unicode | Export-Csv -Path yourfileoutput.csv -Delimiter ',' -Encoding utf8BOM
```

The delimiter character can be changed to any value (e.g., `'|'` for pipe-delimited output).

### Delete Files Listed In A Text File

```powershell
Get-Content .\files-to-delete.txt |
  ForEach-Object { Remove-Item $_ -Force }
```

Alternate form:

```powershell
Get-Content .\filesToDelete.txt | ForEach-Object {Remove-Item $_}
```

### Show Current Session PATH One Entry Per Line

```powershell
$env:PATH -split ';'
```

Sorted alphabetically:

```powershell
$env:PATH -split ';' | Sort-Object
```

### Write Directory Listing To Text File

```powershell
$directory = "C:\path\to\your\directory"
$outputFile = "C:\path\to\output\file.txt"

# List all items (files and subdirectories)
Get-ChildItem -Path $directory | Out-File -FilePath $outputFile

# List only files (excluding subdirectories)
Get-ChildItem -Path $directory -File | Out-File -FilePath $outputFile

# Include subdirectories recursively
Get-ChildItem -Path $directory -Recurse -File | Out-File -FilePath $outputFile
```

### Grab All URLs From A Web Page

```powershell
$URI = 'www.insert.yoururl.com'
(Invoke-WebRequest -URI $URI).links.href | Select-Object -Unique | ForEach-Object {if ($_ -match '(^/|^#)') {$URI + $_} else {$_}}
```

### List All MMC (.msc) Files In System32

```powershell
Get-ChildItem -Path C:\Windows\system32\* -Include *.msc | Sort-Object -Property Extension | Select-Object -Property Name | Format-Wide -Column 1
```

### Convert Hexadecimal To ASCII

Convert a number to hexadecimal:

```powershell
[convert]::tostring(12345,16)
# Output: 3039
```

Convert hexadecimal back to decimal:

```powershell
[convert]::toint16("3039",16)
# Output: 12345
```

Find the ASCII value of a character, and convert back:

```powershell
[BYTE][CHAR]'a'
# Output: 97

[CHAR][BYTE]97
# Output: a
```

Decode a space-separated hex string to readable ASCII text:

```powershell
$HEXDATA="50 6f 73 74 61 6c 41 63 63 65 73 73 31 09 55 62 65 72 4c 33 33 74"

# Split, convert each pair to decimal
$HEXDATA.Split(" ") | FOREACH { [CONVERT]::toint16($_,16) }

# Full conversion: hex to ASCII characters written inline
$HEXDATA.Split(" ") | FOREACH { WRITE-HOST -object ([CHAR][BYTE]([CONVERT]::toint16($_,16))) -nonewline }
```

### Hex Dump A Binary File

Simple hex dump using Get-Content:

```powershell
Get-Content "C:\Windows\notepad.exe" -Encoding Byte `
  -ReadCount 16 | ForEach-Object {
  $output = ""
  foreach ( $byte in $_ ) {
    $output += "{0:X2} " -f $byte
  }
  $output
}
```

High-performance hex dump using stream reader for large files:

```powershell
$bufferSize = 65536
$stream = [System.IO.File]::OpenRead(
  "C:\Windows\notepad.exe")
while ( $stream.Position -lt $stream.Length ) {
  $buffer = new-object Byte[] $bufferSize
  $bytesRead = $stream.Read($buffer, 0, $bufferSize)
  for ( $line = 0; $line -lt [Math]::Floor($bytesRead /
  16); $line++ ) {
    $slice = $buffer[($line * 16)..(($line * 16) + 15)]
    (("{0:X2} {1:X2} {2:X2} {3:X2} {4:X2} {5:X2} ") +
    ("{6:X2} {7:X2} {8:X2} {9:X2} {10:X2} {11:X2} ") +
    ("{12:X2} {13:X2} {14:X2} {15:X2} ")) -f $slice
  }
  if ( $bytesRead % 16 -ne 0 ) {
    $slice = $buffer[($line * 16)..($bytesRead - 1)]
    $output = ""
    foreach ( $byte in $slice ) {
      $output += "{0:X2} " -f $byte
    }
    $output
  }
}
$stream.Close()
```

### Pass Execution Policy To pwsh.dll

```powershell
C:\Program Files\PowerShell\7-preview\pwsh.dll -ExecutionPolicy Bypass -NoProfile
```

## Verification

- Confirm renamed files have the expected extensions.
- Open the generated CSV and check that columns were preserved.
- Review the list of removed or changed files before continuing to the next batch.
- Verify hex conversion output matches expected ASCII text.
- Check that URL extraction results include both relative and absolute links.

## Related

- [`PowerShell System Inspection Patterns`](/powershell/querying/system-inspection-patterns/)
- [`PowerShell Console And Profile Customization`](/powershell/profiles/console-and-profile-customization/)
