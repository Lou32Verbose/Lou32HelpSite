---
title: TikTok Download Template
slug: /cli-tools/gallery-dl/tiktok-download-template/
summary: Optimized gallery-dl command for downloading TikTok profiles with rate limiting, throttling, and archive tracking.
topic: cli-tools/gallery-dl
type: template
tags: [gallery-dl, tiktok, download, rate-limiting]
aliases: [gallery-dl tiktok optimized command]
platforms: [windows, linux, macos]
related:
  - /cli-tools/yt-dlp/media-download-templates/
status: published
updated: 2026-03-21
---

## Use Case

Download an entire TikTok user's profile with optimized rate limiting to avoid 429 errors and archive tracking to skip previously downloaded content.

## Template

```text
gallery-dl --sleep-extractor 10-20 --sleep-request 1.5-3.0 --sleep-429 900 --limit-rate 800k-1500k --download-archive "<archive-path>" -d "<output-dir>" "<tiktok-url>"
```

## Variables

- `--sleep-extractor 10-20`: wait 10-20 seconds between page extractions
- `--sleep-request 1.5-3.0`: wait 1.5-3.0 seconds between HTTP requests
- `--sleep-429 900`: wait 900 seconds (15 minutes) when receiving a 429 rate limit response
- `--limit-rate 800k-1500k`: throttle download speed to 800KB-1.5MB/s
- `--download-archive`: path to text file tracking downloaded items (skips duplicates on re-run)
- `-d`: output directory for downloaded files
- `<tiktok-url>`: TikTok profile URL (e.g., `https://www.tiktok.com/@username`)

## Examples

```text
gallery-dl --sleep-extractor 10-20 --sleep-request 1.5-3.0 --sleep-429 900 --limit-rate 800k-1500k --download-archive "C:\Downloads\tiktok-archive.txt" -d "C:\Downloads\TikTok" "https://www.tiktok.com/@exampleuser"
```

## Related

- [`yt-dlp Media Download Templates`](/cli-tools/yt-dlp/media-download-templates/)
