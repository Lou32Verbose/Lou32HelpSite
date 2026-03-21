---
title: FFmpeg Conversion And Frame Extraction
slug: /media/ffmpeg/conversion-and-frame-extraction/
summary: Reusable FFmpeg templates for compressing MP4 files, extracting frames, batch-converting audio, context menu integration, and related ImageMagick commands.
topic: media/ffmpeg
type: template
tags: [ffmpeg, media, conversion, video, audio, imagemagick, powershell, batch]
aliases: [ffmpeg templates, grab still image from video, wav to mp3, ffmpeg cheat sheet]
platforms: [windows, linux, macos]
related:
  - /midi/reference/core-midi-reference/
status: published
updated: 2026-03-21
---

## Use Case

Keep a comprehensive set of FFmpeg command templates for the conversions that come up repeatedly: basic format conversion, remuxing, trimming, audio extraction, video compression, frame grabs, batch transcoding, subtitle burning, concatenation, and context-menu integration. Also includes an ImageMagick favicon generator.

## Template

```text
ffmpeg -i input.ext [options] output.ext
```

## Variables

- `-crf`: Constant Rate Factor -- visual quality target (range 0-51, default 23, visually lossless ~18)
- `-preset`: speed versus compression tradeoff (`ultrafast` to `veryslow`)
- `-ss`: seek position (before `-i` for fast seek, after for accurate seek)
- `-t` / `-to`: duration or end time of a clip
- `-frames:v 1`: write exactly one video frame
- `-c copy`: stream-copy without re-encoding (fast, lossless)
- `-q:v`: output quality for JPEG (1-31, lower is better; 2-5 is a good range)
- `-ab` / `-b:a`: audio bitrate
- `-ar`: audio sample rate
- `-ac`: audio channel count
- `-map`: stream selection
- `-vf`: video filter chain

## Examples

### Basic Conversion

Convert container format (re-encodes to defaults of target format):

```bash
ffmpeg -i in.mp4 out.avi
```

### Remux

Remux MKV to MP4 without re-encoding (stream copy):

```bash
ffmpeg -i in.mkv -c:v copy -c:a copy out.mp4
```

### High-Quality Encoding

Use CRF for constant-quality encoding. Lower CRF = higher quality. Visually lossless is around `-crf 18`. See [H.264 Encoding Guide](https://trac.ffmpeg.org/wiki/Encode/H.264).

```bash
ffmpeg -i in.mp4 -preset slower -crf 18 out.mp4
```

### Trimming

Without re-encoding (fast, stream copy):

```bash
ffmpeg -ss [start] -i in.mp4 -t [duration] -c copy out.mp4
```

- `-ss` specifies start time, e.g. `00:01:23.000` or `83` (seconds)
- `-t` specifies duration; `-to` specifies end time
- `-c copy` copies video, audio, and subtitle streams without re-encoding

With re-encoding (frame-accurate):

```bash
ffmpeg -ss [start] -i in.mp4 -t [duration] -c:v libx264 -c:a aac -strict experimental -b:a 128k out.mp4
```

### Audio Extraction

WAV rematrix (stereo, verbose logging):

```bash
ffmpeg -i input.wav -c:v libx264 -c:a copy -rematrix_maxval 1.0 -ac 2 -report -loglevel verbose output.wav
```

### Mux Video and Audio from Different Sources

Copy video from `in0.mp4` and audio from `in1.mp4`:

```bash
ffmpeg -i in0.mp4 -i in1.mp4 -c copy -map 0:0 -map 1:1 -shortest out.mp4
```

- `-shortest` matches output duration to the shortest input stream
- `-c copy` avoids re-encoding

### Frame Extraction

Grab a single still frame as JPEG (place `-ss` before `-i` for speed):

```bash
ffmpeg -ss 01:23:45 -i video.mp4 -frames:v 1 -q:v 2 output.jpg
```

For JPEG output, `-q:v` controls quality (1-31, lower = better; 2-5 is recommended).

Grab a single frame as PNG:

```bash
ffmpeg -ss 01:23:45 -i video.mp4 -c:v png -frames:v 1 image.png
```

Extract all frames between specific time ranges:

```bash
ffmpeg -i in.mp4 -vf select='between(t,1,5)+between(t,11,15)' -vsync 0 out%d.png
```

Extract one frame per second:

```bash
ffmpeg -i in.mp4 -vf fps=1 -vsync 0 out%d.png
```

Extract all frames:

```bash
ffmpeg -i input.mp4 thumb%04d.jpg -hide_banner
```

Extract one frame each second:

```bash
ffmpeg -i input.mp4 -vf fps=1 thumb%04d.jpg -hide_banner
```

Extract exactly one frame at a given timestamp:

```bash
ffmpeg -i input.mp4 -ss 00:00:10.000 -vframes 1 thumb.jpg
```

### Batch Conversion

Recursively convert all WAV (or FLAC) files to 320k MP3, preserving metadata:

```bash
find . -iname '*.wav' -exec bash -c 'D=$(dirname "{}"); B=$(basename "{}"); mkdir -p "$D/mp3"; ffmpeg -i "{}" -ab 320k -map_metadata 0 -id3v2_version 3 -acodec libmp3lame "$D/mp3/${B%.*}.mp3"' \;
```

### Context Menu Integration

Add a "Convert WAV to MP3" option to the Windows right-click context menu. Settings: 44100 Hz sample rate, 256k bitrate, stereo:

```text
cmd /q /c for %%I in ("%1") do ffmpeg -i %%I -acodec libmp3lame -vn -ar 44100 -ac 2 -ab 256k "%%~nI.mp3"
```

To install, create a registry key under `HKEY_CLASSES_ROOT\SystemFileAssociations\.wav\shell\ConvertToMP3\command` with the above as the default value (adjust the path to `ffmpeg.exe` as needed).

### Compression

Compress an MP4 with H.264 video and AAC audio:

```text
ffmpeg -i "C:\path\to\video.mp4" -c:v libx264 -crf 23 -preset medium -c:a aac -b:a 128k "%USERPROFILE%\Downloads\compressedvid.mp4"
```

### Concat

Concatenate multiple files using the concat demuxer. First create a text file listing inputs:

```text
file 'in1.mp4'
file 'in2.mp4'
file 'in3.mp4'
file 'in4.mp4'
```

Then run:

```bash
ffmpeg -f concat -i list.txt -c copy out.mp4
```

### Delay Audio/Video

Delay video by 3.84 seconds:

```bash
ffmpeg -i in.mp4 -itsoffset 3.84 -i in.mp4 -map 1:v -map 0:a -vcodec copy -acodec copy out.mp4
```

Delay audio by 3.84 seconds:

```bash
ffmpeg -i in.mp4 -itsoffset 3.84 -i in.mp4 -map 0:v -map 1:a -vcodec copy -acodec copy out.mp4
```

### Subtitles

Convert subtitles to ASS format:

```bash
ffmpeg -i sub.srt sub.ass
```

Burn subtitles into video (requires `--enable-libass`):

```bash
ffmpeg -i in.mp4 -vf ass=sub.ass out.mp4
```

### Rotate Video

Rotate 90 degrees clockwise:

```bash
ffmpeg -i in.mov -vf "transpose=1" out.mov
```

Transpose values:

```text
0 = 90 CounterClockwise and Vertical Flip (default)
1 = 90 Clockwise
2 = 90 CounterClockwise
3 = 90 Clockwise and Vertical Flip
```

For 180 degrees: `-vf "transpose=2,transpose=2"`

### Mute Audio Segments

Replace the first 90 seconds of audio with silence:

```bash
ffmpeg -i in.mp4 -vcodec copy -af "volume=enable='lte(t,90)':volume=0" out.mp4
```

Replace audio between 1:20 and 1:30 with silence:

```bash
ffmpeg -i in.mp4 -vcodec copy -af "volume=enable='between(t,80,90)':volume=0" out.mp4
```

### Deinterlace

```bash
ffmpeg -i in.mp4 -vf yadif out.mp4
```

### Download Transport Stream (HLS/m3u8)

1. Locate the playlist file (Chrome > F12 > Network > Filter: `m3u8`)
2. Download and concatenate:

```bash
ffmpeg -i "path_to_playlist.m3u8" -c copy -bsf:a aac_adtstoasc out.mp4
```

If you get a protocol whitelist error:

```bash
ffmpeg -protocol_whitelist "file,http,https,tcp,tls" -i "path_to_playlist.m3u8" -c copy -bsf:a aac_adtstoasc out.mp4
```

### Create Video Slideshow from Images

`-r` sets the image framerate (inverse display time per image); `-vf fps=25` sets the output framerate:

```bash
ffmpeg -r 1/5 -i img%03d.png -c:v libx264 -vf fps=25 -pix_fmt yuv420p out.mp4
```

### Display Frame Number Overlay

```bash
ffmpeg -i in.mov -vf "drawtext=fontfile=arial.ttf: text=%{n}: x=(w-tw)/2: y=h-(2*lh): fontcolor=white: box=1: boxcolor=0x00000099: fontsize=72" -y out.mov
```

### Change Metadata Title

```bash
ffmpeg -i in.mp4 -map_metadata -1 -metadata title="My Title" -c:v copy -c:a copy out.mp4
```

### Bash Frame Extraction Script

Full script to extract every frame from a video as maximum-quality PNG images:

```bash
#!/bin/bash

# FFmpeg Frame Extraction Script
# Extracts every single frame from a video as high-quality PNG images

# Check if input file is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <input_video_file> [output_directory]"
    echo "Example: $0 video.mp4 frames/"
    exit 1
fi

INPUT_VIDEO="$1"
OUTPUT_DIR="${2:-frames}"  # Default to 'frames' directory if not specified

# Check if input file exists
if [ ! -f "$INPUT_VIDEO" ]; then
    echo "Error: Input file '$INPUT_VIDEO' does not exist."
    exit 1
fi

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

echo "Extracting frames from: $INPUT_VIDEO"
echo "Output directory: $OUTPUT_DIR"
echo "Starting extraction..."

# FFmpeg command to extract all frames
# -vf: video filter (scale ensures we maintain original resolution)
# -pix_fmt: pixel format (rgba for maximum quality with transparency support)
# -compression_level: PNG compression (0 = no compression, fastest, largest files)
# -pred: PNG prediction method (mixed gives best compression without quality loss)
ffmpeg -i "$INPUT_VIDEO" \
       -vf "scale=iw:ih" \
       -pix_fmt rgba \
       -compression_level 0 \
       -pred mixed \
       "$OUTPUT_DIR/frame_%08d.png"

# Check if extraction was successful
if [ $? -eq 0 ]; then
    FRAME_COUNT=$(ls -1 "$OUTPUT_DIR"/*.png 2>/dev/null | wc -l)
    echo "Success! Extracted $FRAME_COUNT frames to $OUTPUT_DIR/"

    # Display file size information
    TOTAL_SIZE=$(du -sh "$OUTPUT_DIR" | cut -f1)
    echo "Total size of extracted frames: $TOTAL_SIZE"
else
    echo "Error: Frame extraction failed."
    exit 1
fi
```

### PowerShell Frame Extraction Script

Full PowerShell script to extract every frame from a video as high-quality PNG images. Requires `ffmpeg.exe` and `ffprobe.exe` in PATH.

```powershell
# FFmpeg Frame Extraction PowerShell Script
# Extracts every single frame from a video as high-quality PNG images
# Requires ffmpeg.exe to be installed and accessible in PATH

param(
    [Parameter(Mandatory=$true, Position=0)]
    [string]$InputVideo,

    [Parameter(Mandatory=$false, Position=1)]
    [string]$OutputDirectory = "frames"
)

# Function to check if ffmpeg is available in the system PATH
function Test-FFmpegAvailable {
    try {
        $null = Get-Command ffmpeg -ErrorAction Stop
        return $true
    }
    catch {
        return $false
    }
}

# Function to format file sizes in human-readable format
function Format-FileSize {
    param([long]$Size)

    $units = @("B", "KB", "MB", "GB", "TB")
    $index = 0
    $sizeFloat = [double]$Size

    while ($sizeFloat -ge 1024 -and $index -lt $units.Length - 1) {
        $sizeFloat /= 1024
        $index++
    }

    return "{0:N2} {1}" -f $sizeFloat, $units[$index]
}

# Display usage information if no parameters provided
if (-not $InputVideo) {
    Write-Host "Usage: .\extract-frames.ps1 <input_video_file> [output_directory]" -ForegroundColor Yellow
    Write-Host "Example: .\extract-frames.ps1 video.mp4 frames" -ForegroundColor Yellow
    exit 1
}

# Check if ffmpeg is installed and accessible
Write-Host "Checking for ffmpeg installation..." -ForegroundColor Cyan
if (-not (Test-FFmpegAvailable)) {
    Write-Host "Error: ffmpeg is not found in your system PATH." -ForegroundColor Red
    Write-Host "Please install ffmpeg and ensure it's accessible from PowerShell." -ForegroundColor Red
    Write-Host "Download from: https://ffmpeg.org/download.html" -ForegroundColor Yellow
    exit 1
}
Write-Host "ffmpeg found successfully." -ForegroundColor Green

# Validate input file exists
if (-not (Test-Path $InputVideo)) {
    Write-Host "Error: Input file '$InputVideo' does not exist." -ForegroundColor Red
    exit 1
}

# Get absolute paths to avoid any relative path issues
$InputVideoPath = Resolve-Path $InputVideo
$OutputDirectoryPath = Join-Path (Get-Location) $OutputDirectory

# Create output directory if it doesn't exist
if (-not (Test-Path $OutputDirectoryPath)) {
    Write-Host "Creating output directory: $OutputDirectoryPath" -ForegroundColor Cyan
    New-Item -ItemType Directory -Path $OutputDirectoryPath -Force | Out-Null
}

Write-Host ""
Write-Host "=== Frame Extraction Configuration ===" -ForegroundColor Green
Write-Host "Input Video: $InputVideoPath" -ForegroundColor White
Write-Host "Output Directory: $OutputDirectoryPath" -ForegroundColor White

# Get video information before processing
Write-Host ""
Write-Host "Analyzing video properties..." -ForegroundColor Cyan
try {
    # Use ffprobe to get video information (frame count, duration, fps)
    $videoInfo = & ffprobe -v quiet -print_format json -show_format -show_streams "$InputVideoPath" | ConvertFrom-Json
    $videoStream = $videoInfo.streams | Where-Object { $_.codec_type -eq "video" } | Select-Object -First 1

    if ($videoStream) {
        $duration = [math]::Round([double]$videoStream.duration, 2)
        $fps = if ($videoStream.r_frame_rate -match "(\d+)/(\d+)") {
            [math]::Round([double]$matches[1] / [double]$matches[2], 2)
        } else { "Unknown" }
        $resolution = "$($videoStream.width)x$($videoStream.height)"
        $estimatedFrames = if ($fps -ne "Unknown") { [math]::Ceiling($duration * $fps) } else { "Unknown" }

        Write-Host "Video Duration: $duration seconds" -ForegroundColor White
        Write-Host "Frame Rate: $fps fps" -ForegroundColor White
        Write-Host "Resolution: $resolution" -ForegroundColor White
        Write-Host "Estimated Frame Count: $estimatedFrames" -ForegroundColor White
    }
}
catch {
    Write-Host "Could not analyze video properties, but proceeding with extraction..." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Starting frame extraction..." -ForegroundColor Green
Write-Host "Note: This process may take several minutes depending on video length and resolution." -ForegroundColor Yellow

# Build the output path pattern for ffmpeg
$outputPattern = Join-Path $OutputDirectoryPath "frame_%08d.png"

# Execute ffmpeg with maximum quality settings
# -i: specifies input file
# -vf "scale=iw:ih": maintains original resolution without any scaling artifacts
# -pix_fmt rgba: uses RGBA pixel format for maximum color depth and alpha support
# -compression_level 0: disables PNG compression for absolute maximum quality
# -pred mixed: uses mixed prediction method for optimal lossless encoding
try {
    $ffmpegArgs = @(
        "-i", "`"$InputVideoPath`""
        "-vf", "scale=iw:ih"
        "-pix_fmt", "rgba"
        "-compression_level", "0"
        "-pred", "mixed"
        "-y"  # Overwrite output files without asking
        "`"$outputPattern`""
    )

    # Start ffmpeg process and capture output
    $process = Start-Process -FilePath "ffmpeg" -ArgumentList $ffmpegArgs -NoNewWindow -Wait -PassThru

    if ($process.ExitCode -eq 0) {
        Write-Host ""
        Write-Host "Frame extraction completed successfully!" -ForegroundColor Green

        # Count extracted frames and calculate total size
        $extractedFrames = Get-ChildItem "$OutputDirectoryPath\*.png" -ErrorAction SilentlyContinue
        $frameCount = $extractedFrames.Count

        if ($frameCount -gt 0) {
            $totalSize = ($extractedFrames | Measure-Object -Property Length -Sum).Sum
            $formattedSize = Format-FileSize $totalSize

            Write-Host ""
            Write-Host "=== Extraction Results ===" -ForegroundColor Green
            Write-Host "Frames Extracted: $frameCount" -ForegroundColor White
            Write-Host "Total Size: $formattedSize" -ForegroundColor White
            Write-Host "Average Size per Frame: $(Format-FileSize ($totalSize / $frameCount))" -ForegroundColor White
            Write-Host "Output Location: $OutputDirectoryPath" -ForegroundColor White
        }
        else {
            Write-Host "Warning: No PNG files found in output directory." -ForegroundColor Yellow
        }
    }
    else {
        Write-Host "Error: ffmpeg process failed with exit code $($process.ExitCode)" -ForegroundColor Red
        exit 1
    }
}
catch {
    Write-Host "Error executing ffmpeg: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Frame extraction process complete." -ForegroundColor Cyan
```

### ImageMagick Favicon

Generate a high-quality, multi-size, transparent favicon from a source PNG using ImageMagick:

```bash
magick "source_image.png" -background transparent -define icon:auto-resize=16,32,48,64,128,256 -type TrueColorAlpha "favicon.ico"
```

This produces an ICO file containing all standard favicon sizes (16, 32, 48, 64, 128, 256) with transparency preserved.

## Related

- [`Core MIDI Reference`](/midi/reference/core-midi-reference/)
- [FFmpeg Official Documentation](https://www.ffmpeg.org/ffmpeg.html)
- [FFmpeg Download](https://www.ffmpeg.org/download.html)
- [H.264 Encoding Guide](https://trac.ffmpeg.org/wiki/Encode/H.264)
- [ffmpeg.lav.io](https://ffmpeg.lav.io/) -- interactive FFmpeg command composer
