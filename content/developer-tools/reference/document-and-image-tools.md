---
title: Document And Image Tools
slug: /developer-tools/reference/document-and-image-tools/
summary: Command references for Pandoc document conversion, ImageMagick favicon generation, Python HTTP server, NuGet source setup, and Visual Studio 2015 download URLs.
topic: developer-tools/reference
type: reference
tags: [pandoc, imagemagick, python, nuget, visual-studio, favicon]
aliases: [pandoc docx to markdown, imagemagick favicon, http server python, nuget add source, visual studio 2015 downloads]
platforms: [windows, linux, macos]
related:
  - /developer-tools/reference/ascii-and-color-reference/
status: published
updated: 2026-03-21
---

## Synopsis

Quick reference commands for document conversion, image processing, local web servers, and .NET package management.

## Syntax

```bash
pandoc -t markdown_strict --extract-media='./attachments/$myfilename' $myfilename.docx -o $myfilename.md
magick "source.png" -background transparent -define icon:auto-resize=16,32,48,64,128,256 "favicon.ico"
python -m http.server 80
dotnet nuget add source <url> -n <name>
```

## Parameters/Flags

- `-t markdown_strict`: Pandoc output format for clean markdown
- `--extract-media`: extract embedded images to a directory
- `-define icon:auto-resize`: ImageMagick multi-size ICO generation
- `-type TrueColorAlpha`: preserve transparency in favicon
- `-m http.server`: Python 3 built-in HTTP server module

## Examples

### Pandoc: DOCX to Markdown

```bash
myfilename="example"
pandoc \
  -t markdown_strict \
  --extract-media="./attachments/$myfilename" \
  "$myfilename.docx" \
  -o "$myfilename.md"
```

### ImageMagick: Generate Multi-Size Favicon

Creates a high-quality, multi-size, transparent favicon from a source PNG:

```text
magick "source_image.png" -background transparent -define icon:auto-resize=16,32,48,64,128,256 -type TrueColorAlpha "favicon.ico"
```

### Python HTTP Server

**Python 3:**

```bash
python -m http.server 80
```

**Python 2:**

```bash
python -m SimpleHTTPServer 80
```

Run from the directory you want to serve, then visit `http://<your-ip>`. For remote access, add a reverse proxy for HTTPS and basic authentication.

### NuGet: Add Package Source

```text
dotnet nuget add source https://api.nuget.org/v3/index.json -n nuget.org
```

### Visual Studio 2015 Download URLs

| Edition | URL |
|---------|-----|
| Community | `https://go.microsoft.com/fwlink/?LinkId=615448` |
| Professional | `https://go.microsoft.com/fwlink/?LinkId=615434` |
| Enterprise | `https://go.microsoft.com/fwlink/?LinkId=615436` |

Direct ISO download (Community):

```text
https://download.microsoft.com/download/b/e/d/bedddfc4-55f4-4748-90a8-ffe38a40e89f/vs2015.3.com_enu.iso
```

## Related

- [`ASCII And Color Reference`](/developer-tools/reference/ascii-and-color-reference/)
