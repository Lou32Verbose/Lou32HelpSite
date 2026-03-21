---
title: yt-dlp Media Download Templates
slug: /cli-tools/yt-dlp/media-download-templates/
summary: Reusable `yt-dlp` templates for format selection, output naming, audio extraction, playlist and album downloads, quality control, and `gallery-dl` usage.
topic: cli-tools/yt-dlp
type: template
tags: [yt-dlp, downloads, audio, video, playlists, gallery-dl, format-selection]
aliases: [yt-dlp output templates, yt-dlp mp3 best quality, yt-dlp format selection]
platforms: [windows, linux, macos]
related:
  - /cli-tools/wget/recursive-download/
status: published
updated: 2026-03-21
---

## Use Case

Keep a library of trusted `yt-dlp` templates for format selection, audio extraction, predictable filenames, playlist downloads, album ripping, and quality control without rebuilding the command each time. Also includes a `gallery-dl` template for TikTok archival.

## Template

```bash
yt-dlp -o "%(title)s.%(ext)s" URL
```

## Variables

- `%(title)s`: source title
- `%(ext)s`: detected output extension
- `%(playlist)s`: playlist name
- `%(playlist_title)s`: playlist title (alternative key)
- `%(playlist_index)s`: item number within the playlist
- `%(upload_date>%Y)s`: upload year (date formatting syntax)
- `%(uploader)s`: channel or uploader name
- `%(format_id)s`: internal format identifier
- `%(series)s`: series name
- `%(season_number)s`: season number
- `%(season)s`: season name
- `%(episode_number)s`: episode number
- `%(episode)s`: episode name
- `%(chapter_number)s`: chapter number
- `%(chapter)s`: chapter name

## Examples

### Format Selection

Download and merge the best video-only format and the best audio-only format, or download the best combined format if video-only format is not available:

```bash
yt-dlp -f "bv+ba/b"
```

Download best format that contains video, and if it doesn't already have an audio stream, merge it with best audio-only format:

```bash
yt-dlp -f "bv*+ba/b"
```

Same as above (default behavior):

```bash
yt-dlp
```

Download the best video-only format and the best audio-only format without merging them (use an output template since bestvideo and bestaudio will have the same file name by default):

```bash
yt-dlp -f "bv,ba" -o "%(title)s.f%(format_id)s.%(ext)s"
```

Download and merge the best format that has a video stream, and all audio-only formats into one file:

```bash
yt-dlp -f "bv*+mergeall[vcodec=none]" --audio-multistreams
```

Download and merge the best format that has a video stream, and the best 2 audio-only formats into one file:

```bash
yt-dlp -f "bv*+ba+ba.2" --audio-multistreams
```

Download the worst video available (old method):

```bash
yt-dlp -f "wv*+wa/w"
```

Download the best video available but with the smallest resolution:

```bash
yt-dlp -S "+res"
```

Download the smallest video available:

```bash
yt-dlp -S "+size,+br"
```

Download the best mp4 video available, or the best video if no mp4 available:

```bash
yt-dlp -f "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4] / bv*+ba/b"
```

Download the best video with the best extension (for video, mp4 > mov > webm > flv; for audio, m4a > aac > mp3):

```bash
yt-dlp -S "ext"
```

Download the best video available but no better than 480p, or the worst video if there is no video under 480p:

```bash
yt-dlp -f "bv*[height<=480]+ba/b[height<=480] / wv*+ba/w"
```

Download the best video available with the largest height but no better than 480p, or the best video with the smallest resolution if there is no video under 480p:

```bash
yt-dlp -S "height:480"
```

Download the best video available with the largest resolution but no better than 480p (works correctly for vertical videos as well since resolution uses the smallest dimension):

```bash
yt-dlp -S "res:480"
```

Download the best video (that also has audio) but no bigger than 50 MB, or the worst video if there is no video under 50 MB:

```bash
yt-dlp -f "b[filesize<50M] / w"
```

Download the largest video (that also has audio) but no bigger than 50 MB, or the smallest video if there is no video under 50 MB:

```bash
yt-dlp -f "b" -S "filesize:50M"
```

Download the best video (that also has audio) that is closest in size to 50 MB:

```bash
yt-dlp -f "b" -S "filesize~50M"
```

Download best video available via direct link over HTTP/HTTPS protocol, or the best video available via any protocol if there is no such video:

```bash
yt-dlp -f "(bv*+ba/b)[protocol^=http][protocol!*=dash] / (bv*+ba/b)"
```

Download best video available via the best protocol (https/ftps > http/ftp > m3u8_native > m3u8 > http_dash_segments):

```bash
yt-dlp -S "proto"
```

Download the best video with either h264 or h265 codec, or the best video if there is no such video:

```bash
yt-dlp -f "(bv*[vcodec~='^((he|a)vc|h26[45])']+ba) / (bv*+ba/b)"
```

Download the best video with best codec no better than h264, or the best video with worst codec if there is no such video:

```bash
yt-dlp -S "codec:h264"
```

Download the best video with worst codec no worse than h264, or the best video with best codec if there is no such video:

```bash
yt-dlp -S "+codec:h264"
```

Download the best video no better than 720p preferring framerate greater than 30, or the worst video (still preferring framerate greater than 30) if there is no such video:

```bash
yt-dlp -f "((bv*[fps>30]/bv*)[height<=720]/(wv*[fps>30]/wv*)) + ba / (b[fps>30]/b)[height<=720]/(w[fps>30]/w)"
```

Download the video with the largest resolution no better than 720p, preferring larger framerate for formats with the same resolution:

```bash
yt-dlp -S "res:720,fps"
```

Download the video with smallest resolution no worse than 480p, preferring better codec and then larger total bitrate for the same resolution:

```bash
yt-dlp -S "+res:480,codec,br"
```

### Output Templates

Literal name with correct extension:

```bash
yt-dlp --print filename -o "test video.%(ext)s" BaW_jenozKc
```

Use the video title as filename:

```bash
yt-dlp --print filename -o "%(title)s.%(ext)s" BaW_jenozKc
```

Restricted filename (safe characters only):

```bash
yt-dlp --print filename -o "%(title)s.%(ext)s" BaW_jenozKc --restrict-filenames
```

Download YouTube playlist videos in separate directory indexed by video order:

```bash
yt-dlp -o "%(playlist)s/%(playlist_index)s - %(title)s.%(ext)s" "https://www.youtube.com/playlist?list=PLwiyx1dc3P2JR9N8gQaQN_BCvlSlap7re"
```

Download YouTube playlist videos in separate directories according to their uploaded year:

```bash
yt-dlp -o "%(upload_date>%Y)s/%(title)s.%(ext)s" "https://www.youtube.com/playlist?list=PLwiyx1dc3P2JR9N8gQaQN_BCvlSlap7re"
```

Prefix playlist index with separator, but only if it is available:

```bash
yt-dlp -o "%(playlist_index&{} - |)s%(title)s.%(ext)s" BaW_jenozKc "https://www.youtube.com/user/TheLinuxFoundation/playlists"
```

Download all playlists of YouTube channel/user keeping each playlist in separate directory:

```bash
yt-dlp -o "%(uploader)s/%(playlist)s/%(playlist_index)s - %(title)s.%(ext)s" "https://www.youtube.com/user/TheLinuxFoundation/playlists"
```

Download Udemy course keeping each chapter in separate directory:

```bash
yt-dlp -u user -p password -P "~/MyVideos" -o "%(playlist)s/%(chapter_number)s - %(chapter)s/%(title)s.%(ext)s" "https://www.udemy.com/java-tutorial"
```

Download entire series season keeping each series and each season in separate directory:

```bash
yt-dlp -P "C:/MyVideos" -o "%(series)s/%(season_number)s - %(season)s/%(episode_number)s - %(episode)s.%(ext)s" "https://videomore.ru/kino_v_detalayah/5_sezon/367617"
```

Download video to one path and subtitles to a dedicated subfolder, with temp files in a separate directory:

```bash
yt-dlp -P "C:/MyVideos" -P "temp:tmp" -P "subtitle:subs" -o "%(uploader)s/%(title)s.%(ext)s" BaW_jenozKc --write-subs
```

Download video and subtitles into nested uploader subdirectories:

```bash
yt-dlp -P "C:/MyVideos" -o "%(uploader)s/%(title)s.%(ext)s" -o "subtitle:%(uploader)s/subs/%(title)s.%(ext)s" BaW_jenozKc --write-subs
```

Stream the video being downloaded to stdout:

```bash
yt-dlp -o - BaW_jenozKc
```

### Audio Extraction

Download best audio and extract to highest quality (VBR 0):

```bash
yt-dlp -f bestaudio --extract-audio --audio-quality 0 -o "%(title)s.%(ext)s" URL
```

Download a YouTube video as MP3 with modified date set to download date, single video only (no playlist), saved to Downloads folder:

```bash
yt-dlp -o "~/Downloads/%(title)s.%(ext)s" --no-mtime --no-playlist --extract-audio --audio-format mp3 --format "bestvideo[height=1080]+bestaudio/best[height<=1080]/best" --merge-output-format mp4 <INSERT-URL-HERE>
```

- `--no-mtime` sets the file modified date to the download date instead of the video upload date
- `-o "~/Downloads/%(title)s.%(ext)s"` saves to the Downloads folder using the video title as the filename

### Playlist Downloads

Download entire YouTube Music playlist as MP3s (one-liner):

```bash
yt-dlp --ignore-errors --yes-playlist --format bestaudio --extract-audio --audio-format mp3 --audio-quality 0 --output "%(playlist_index)02d - %(title)s.%(ext)s" "<insert-url>"
```

Keep playlist videos in a per-playlist folder:

```bash
yt-dlp -o "%(playlist)s/%(playlist_index)s - %(title)s.%(ext)s" PLAYLIST_URL
```

### Album Downloads

Download entire album from YouTube Music as MP3, organized in an album folder:

```bash
yt-dlp -x --audio-format mp3 --audio-quality 0 -o "%(playlist_title)s/%(playlist_index)s - %(title)s.%(ext)s" <insert-ytmusic-album-url>
```

- `-x` extracts audio
- `--audio-format mp3` converts to MP3
- `--audio-quality 0` uses highest available quality
- Output is organized as `AlbumName/01 - SongTitle.mp3`

### Quality Selection

Download video in highest quality up to 1080p, preferring h264 codec, merged to MKV:

```bash
yt-dlp -f "bv*+ba/b" -S "res:1080,fps,vcodec:avc" --merge-output-format mkv -o "%(title)s.%(ext)s" --paths <desired-download-dir> <insert-video-url>
```

Restrict filenames to safer characters:

```bash
yt-dlp --restrict-filenames -o "%(title)s.%(ext)s" URL
```

### gallery-dl

Download TikTok profile with rate limiting, retry on 429, and download archive to avoid re-downloading:

```bash
gallery-dl --sleep-extractor 10-20 --sleep-request 1.5-3.0 --sleep-429 900 --limit-rate 800k-1500k --download-archive "C:\your\path\here\tiktok-archive.txt" -d "C:\your\path\here" "https://www.tiktok.com/@<insert-username>"
```

- `--sleep-extractor 10-20` waits 10-20 seconds between page extractions
- `--sleep-request 1.5-3.0` waits 1.5-3 seconds between HTTP requests
- `--sleep-429 900` waits 15 minutes if rate-limited (HTTP 429)
- `--limit-rate 800k-1500k` throttles download speed
- `--download-archive` tracks already-downloaded files to avoid duplicates

## Related

- [`Wget Recursive Download Reference`](/cli-tools/wget/recursive-download/)
