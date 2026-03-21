---
title: ImgBB API Reference
slug: /developer-tools/reference/imgbb-api-reference/
summary: Quick reference for the ImgBB image hosting API v1 covering upload endpoint, parameters, and JSON response format.
topic: developer-tools/reference
type: reference
tags: [api, imgbb, image-hosting, upload, curl]
aliases: [imagebb api v1 overview]
platforms: [windows, linux, macos]
related:
  - /developer-tools/reference/document-and-image-tools/
status: published
updated: 2026-03-21
---

## Synopsis

The ImgBB API v1 provides a simple image upload endpoint that returns hosted URLs in JSON format. Supports binary files, base64 data, and image URLs up to 32 MB.

## Syntax

```text
POST https://api.imgbb.com/1/upload
```

## Parameters/Flags

| Parameter | Required | Description |
|-----------|----------|-------------|
| `key` | Yes | Your API key |
| `image` | Yes | Binary file, base64 data, or image URL (up to 32 MB) |
| `name` | No | Filename (auto-detected for multipart/form-data uploads) |
| `expiration` | No | Auto-delete after N seconds (range: 60–15552000) |

## Examples

### Upload via curl

```bash
curl --location --request POST \
  "https://api.imgbb.com/1/upload?expiration=600&key=YOUR_API_KEY" \
  --form "image=R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7"
```

### JSON Response Structure

```json
{
  "data": {
    "id": "2ndCYJK",
    "title": "c1f64245afb2",
    "url_viewer": "https://ibb.co/2ndCYJK",
    "url": "https://i.ibb.co/w04Prt6/c1f64245afb2.gif",
    "display_url": "https://i.ibb.co/98W13PY/c1f64245afb2.gif",
    "width": "1",
    "height": "1",
    "size": "42",
    "time": "1552042565",
    "expiration": "0",
    "image": {
      "filename": "c1f64245afb2.gif",
      "name": "c1f64245afb2",
      "mime": "image/gif",
      "extension": "gif",
      "url": "https://i.ibb.co/w04Prt6/c1f64245afb2.gif"
    },
    "thumb": { "...": "..." },
    "medium": { "...": "..." },
    "delete_url": "https://ibb.co/2ndCYJK/670a7e48ddcb85ac340c717a41047e5c"
  },
  "success": true,
  "status": 200
}
```

Key response fields: `data.url` (direct link), `data.display_url` (display-optimized), `data.delete_url` (deletion link), `data.thumb.url` (thumbnail).

> Always use POST when uploading local files. URL encoding may alter base64 data when using GET.

## Related

- [`Document And Image Tools`](/developer-tools/reference/document-and-image-tools/)
