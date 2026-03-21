---
title: GIMP Script-Fu Examples
slug: /developer-tools/reference/gimp-script-fu-examples/
summary: Ten GIMP Script-Fu (Scheme) batch processing scripts for resizing, watermarking, color correction, format conversion, borders, text overlays, artistic effects, contact sheets, metadata, and layer blending.
topic: developer-tools/reference
type: reference
tags: [gimp, script-fu, scheme, batch-processing, image-editing]
aliases: [gimp 10 scripting examples with script-fu, gimp batch processing]
platforms: [windows, linux, macos]
related:
  - /developer-tools/reference/document-and-image-tools/
status: published
updated: 2026-03-21
---

## Synopsis

Ten GIMP Script-Fu (Scheme) batch processing scripts demonstrating core image automation concepts. Each script solves a real-world problem and builds in complexity. Scripts are placed in `~/.gimp-2.10/scripts/` (Linux/Mac) or `%APPDATA%\GIMP\2.10\scripts\` (Windows) and run from Filters > Script-Fu > Console.

## Syntax

```scheme
(define (function-name input-dir output-dir ...params)
  (let* ((filelist (cadr (file-glob (string-append input-dir "/*.{jpg,jpeg,png}")))))
    (while (not (null? filelist))
      ...
      (gimp-image-delete image)
      (set! filelist (cdr filelist)))))
```

## Parameters/Flags

- `RUN-NONINTERACTIVE`: suppress all dialogs during batch execution
- `file-glob`: pattern-based file listing (supports `{jpg,jpeg,png}` syntax)
- `gimp-image-delete`: must be called after each image to prevent memory leaks
- `gimp-image-flatten`: merge all layers into one (required before saving to flat formats like JPEG)

## Examples

### 1. Smart Resize with Aspect Ratio Preservation

Resize hundreds of images while maintaining aspect ratio:

```scheme
(define (batch-smart-resize input-dir output-dir max-width max-height quality)
  (let* ((filelist (cadr (file-glob (string-append input-dir "/*.{jpg,jpeg,png,tiff}")))))
    (while (not (null? filelist))
      (let* ((filename (car filelist))
             (image (car (gimp-file-load RUN-NONINTERACTIVE filename filename)))
             (drawable (car (gimp-image-get-active-layer image)))
             (width (car (gimp-drawable-width drawable)))
             (height (car (gimp-drawable-height drawable)))
             (ratio (min (/ max-width width) (/ max-height height)))
             (new-width (* width ratio))
             (new-height (* height ratio))
             (output-file (string-append output-dir "/"
                          (substring filename (string-rindex filename #\/)
                                   (string-length filename)))))

        (gimp-image-scale image new-width new-height)
        (gimp-file-save RUN-NONINTERACTIVE image drawable
                       output-file output-file)
        (gimp-image-delete image)
        (set! filelist (cdr filelist))))))

; Usage: (batch-smart-resize "/input/path" "/output/path" 800 600 85)
```

### 2. Watermark Applier with Position Control

Add copyright watermarks with consistent positioning (supports `bottom-right`, `bottom-left`, `center`):

```scheme
(define (batch-watermark input-dir output-dir watermark-file opacity position)
  (let* ((filelist (cadr (file-glob (string-append input-dir "/*.{jpg,jpeg,png}"))))
         (watermark-img (car (gimp-file-load RUN-NONINTERACTIVE watermark-file watermark-file)))
         (watermark-layer (car (gimp-image-get-active-layer watermark-img))))

    (while (not (null? filelist))
      (let* ((filename (car filelist))
             (image (car (gimp-file-load RUN-NONINTERACTIVE filename filename)))
             (img-width (car (gimp-image-width image)))
             (img-height (car (gimp-image-height image)))
             (wm-width (car (gimp-drawable-width watermark-layer)))
             (wm-height (car (gimp-drawable-height watermark-layer)))
             (new-layer (car (gimp-layer-new-from-drawable watermark-layer image)))
             (x-pos (cond ((string=? position "bottom-right") (- img-width wm-width 20))
                         ((string=? position "bottom-left") 20)
                         ((string=? position "center") (/ (- img-width wm-width) 2))
                         (else 20)))
             (y-pos (cond ((string-ci=? position "bottom-right") (- img-height wm-height 20))
                         ((string-ci=? position "bottom-left") (- img-height wm-height 20))
                         ((string=? position "center") (/ (- img-height wm-height) 2))
                         (else 20))))

        (gimp-image-insert-layer image new-layer 0 0)
        (gimp-layer-set-offsets new-layer x-pos y-pos)
        (gimp-layer-set-opacity new-layer opacity)
        (gimp-image-merge-down image new-layer EXPAND-AS-NECESSARY)

        (let ((output-file (string-append output-dir "/"
                           (car (reverse (string-split filename #\/))))))
          (gimp-file-save RUN-NONINTERACTIVE image
                         (car (gimp-image-get-active-layer image))
                         output-file output-file))

        (gimp-image-delete image)
        (set! filelist (cdr filelist))))

    (gimp-image-delete watermark-img)))

; Usage: (batch-watermark "/photos" "/watermarked" "/logo.png" 50 "bottom-right")
```

### 3. Auto Color Correction Pipeline

Fix exposure and color issues in scanned documents or photos with optional contrast and color normalization:

```scheme
(define (batch-auto-correct input-dir output-dir enhance-contrast normalize-colors)
  (let* ((filelist (cadr (file-glob (string-append input-dir "/*.{jpg,jpeg,png,tiff}")))))
    (while (not (null? filelist))
      (let* ((filename (car filelist))
             (image (car (gimp-file-load RUN-NONINTERACTIVE filename filename)))
             (drawable (car (gimp-image-get-active-layer image))))

        (gimp-levels-stretch drawable)

        (if enhance-contrast
            (gimp-brightness-contrast drawable 0 15))

        (if normalize-colors
            (begin
              (gimp-color-balance drawable TRANSFER-MIDTONES TRUE 0 0 0)
              (gimp-hue-saturation drawable HUE-RANGE-ALL 0 0 10)))

        (plug-in-unsharp-mask RUN-NONINTERACTIVE image drawable 1.0 1.0 0)

        (let ((output-file (string-append output-dir "/corrected_"
                           (car (reverse (string-split filename #\/))))))
          (gimp-file-save RUN-NONINTERACTIVE image drawable
                         output-file output-file))

        (gimp-image-delete image)
        (set! filelist (cdr filelist))))))

; Usage: (batch-auto-correct "/scanned" "/corrected" TRUE TRUE)
```

### 4. Format Converter with Quality Control

Convert between formats (JPEG, PNG, WebP) with format-specific quality options:

```scheme
(define (batch-convert input-dir output-dir target-format quality progressive)
  (let* ((filelist (cadr (file-glob (string-append input-dir "/*.*")))))
    (while (not (null? filelist))
      (let* ((filename (car filelist))
             (image (car (gimp-file-load RUN-NONINTERACTIVE filename filename)))
             (drawable (car (gimp-image-get-active-layer image)))
             (base-name (substring filename 0 (string-rindex filename #\.)))
             (output-file (string-append output-dir "/"
                          (car (reverse (string-split base-name #\/)))
                          "." target-format)))

        (if (not (= (car (gimp-image-base-type image)) RGB))
            (gimp-image-convert-rgb image))

        (if (or (string=? target-format "jpg")
                (string=? target-format "jpeg"))
            (set! drawable (car (gimp-image-flatten image))))

        (cond
          ((or (string=? target-format "jpg") (string=? target-format "jpeg"))
           (file-jpeg-save RUN-NONINTERACTIVE image drawable
                          output-file output-file
                          quality 0 TRUE progressive "" 0 TRUE 0 0))
          ((string=? target-format "png")
           (file-png-save RUN-NONINTERACTIVE image drawable
                         output-file output-file
                         FALSE 9 FALSE FALSE FALSE FALSE FALSE))
          ((string=? target-format "webp")
           (file-webp-save RUN-NONINTERACTIVE image drawable
                          output-file output-file
                          0 quality 0 0 0 0 0 0 0 0 0 0 0)))

        (gimp-image-delete image)
        (set! filelist (cdr filelist))))))

; Usage: (batch-convert "/raw" "/web" "jpg" 85 TRUE)
```

### 5. Border and Frame Generator

Add consistent borders with optional drop shadows:

```scheme
(define (batch-add-border input-dir output-dir border-width border-color shadow)
  (let* ((filelist (cadr (file-glob (string-append input-dir "/*.{jpg,jpeg,png}")))))
    (while (not (null? filelist))
      (let* ((filename (car filelist))
             (image (car (gimp-file-load RUN-NONINTERACTIVE filename filename)))
             (drawable (car (gimp-image-get-active-layer image)))
             (width (car (gimp-image-width image)))
             (height (car (gimp-image-height image)))
             (new-width (+ width (* border-width 2)))
             (new-height (+ height (* border-width 2))))

        (gimp-image-resize image new-width new-height border-width border-width)
        (gimp-context-set-background border-color)
        (gimp-drawable-fill (car (gimp-image-get-active-layer image)) BACKGROUND-FILL)
        (gimp-layer-resize drawable new-width new-height border-width border-width)

        (if shadow
            (begin
              (script-fu-drop-shadow image drawable 5 5 15
                                   '(0 0 0) 80 FALSE)
              (set! drawable (car (gimp-image-flatten image)))))

        (let ((output-file (string-append output-dir "/framed_"
                           (car (reverse (string-split filename #\/))))))
          (gimp-file-save RUN-NONINTERACTIVE image drawable
                         output-file output-file))

        (gimp-image-delete image)
        (set! filelist (cdr filelist))))))

; Usage: (batch-add-border "/art" "/framed" 50 '(255 255 255) TRUE)
```

### 6. Text Overlay System

Add captions, dates, or filenames to photos automatically (supports `bottom-center`, `top-right`, `bottom-left`). Use `"FILENAME"` as text to insert the source filename:

```scheme
(define (batch-text-overlay input-dir output-dir text font-size font-name color position)
  (let* ((filelist (cadr (file-glob (string-append input-dir "/*.{jpg,jpeg,png}")))))
    (while (not (null? filelist))
      (let* ((filename (car filelist))
             (image (car (gimp-file-load RUN-NONINTERACTIVE filename filename)))
             (width (car (gimp-image-width image)))
             (height (car (gimp-image-height image)))
             (base-name (car (reverse (string-split filename #\/))))
             (display-text (if (string=? text "FILENAME")
                              (substring base-name 0 (string-rindex base-name #\.))
                              text))
             (text-layer (car (gimp-text-fontname image -1 0 0 display-text
                                                 0 TRUE font-size PIXELS font-name))))

        (let* ((text-width (car (gimp-drawable-width text-layer)))
               (text-height (car (gimp-drawable-height text-layer)))
               (x-pos (cond ((string=? position "bottom-center")
                            (/ (- width text-width) 2))
                           ((string=? position "top-right")
                            (- width text-width 20))
                           ((string=? position "bottom-left") 20)
                           (else 20)))
               (y-pos (cond ((string-ci=? position "bottom-center")
                            (- height text-height 20))
                           ((string=? position "top-right") 20)
                           ((string-ci=? position "bottom-left")
                            (- height text-height 20))
                           (else 20))))

          (gimp-layer-set-offsets text-layer x-pos y-pos)
          (gimp-text-layer-set-color text-layer color)

          (let ((flattened (car (gimp-image-flatten image)))
                (output-file (string-append output-dir "/captioned_" base-name)))
            (gimp-file-save RUN-NONINTERACTIVE image flattened
                           output-file output-file)))

        (gimp-image-delete image)
        (set! filelist (cdr filelist))))))

; Usage: (batch-text-overlay "/photos" "/captioned" "FILENAME" 24 "Arial Bold" '(255 255 255) "bottom-center")
```

### 7. Multi-Filter Artistic Effects Pipeline

Apply consistent artistic treatments (`vintage`, `oil-painting`, `pencil-sketch`, `pop-art`):

```scheme
(define (batch-artistic-effect input-dir output-dir effect-type intensity)
  (let* ((filelist (cadr (file-glob (string-append input-dir "/*.{jpg,jpeg,png}")))))
    (while (not (null? filelist))
      (let* ((filename (car filelist))
             (image (car (gimp-file-load RUN-NONINTERACTIVE filename filename)))
             (drawable (car (gimp-image-get-active-layer image))))

        (cond
          ((string=? effect-type "vintage")
           (begin
             (gimp-desaturate drawable 1)
             (gimp-color-balance drawable TRANSFER-SHADOWS FALSE 20 -10 -20)
             (gimp-color-balance drawable TRANSFER-HIGHLIGHTS FALSE 10 0 -15)
             (let ((vignette-layer (car (gimp-layer-copy drawable FALSE))))
               (gimp-image-insert-layer image vignette-layer 0 0)
               (plug-in-vignette RUN-NONINTERACTIVE image vignette-layer
                               TRUE 1.5 1.0 0.7 0.0))))

          ((string=? effect-type "oil-painting")
           (plug-in-oilify RUN-NONINTERACTIVE image drawable
                          (* intensity 2) (* intensity 1)))

          ((string=? effect-type "pencil-sketch")
           (begin
             (gimp-desaturate drawable 1)
             (let ((sketch-layer (car (gimp-layer-copy drawable FALSE))))
               (gimp-image-insert-layer image sketch-layer 0 0)
               (gimp-invert sketch-layer)
               (plug-in-gauss RUN-NONINTERACTIVE image sketch-layer
                             intensity intensity TRUE)
               (gimp-layer-set-mode sketch-layer DODGE-MODE))))

          ((string=? effect-type "pop-art")
           (begin
             (gimp-posterize drawable 6)
             (gimp-hue-saturation drawable HUE-RANGE-ALL 0 0 30)
             (plug-in-edge RUN-NONINTERACTIVE image drawable
                          2.0 1 0))))

        (let ((final-layer (car (gimp-image-flatten image)))
              (output-file (string-append output-dir "/" effect-type "_"
                           (car (reverse (string-split filename #\/))))))
          (gimp-file-save RUN-NONINTERACTIVE image final-layer
                         output-file output-file))

        (gimp-image-delete image)
        (set! filelist (cdr filelist))))))

; Usage: (batch-artistic-effect "/photos" "/artistic" "vintage" 5)
```

### 8. Contact Sheet Generator

Create photo galleries or proof sheets for client review:

```scheme
(define (batch-contact-sheet input-dir output-file cols rows spacing thumb-size)
  (let* ((filelist (cadr (file-glob (string-append input-dir "/*.{jpg,jpeg,png}"))))
         (sheet-width (+ (* cols thumb-size) (* (+ cols 1) spacing)))
         (sheet-height (+ (* rows thumb-size) (* (+ rows 1) spacing)))
         (contact-sheet (car (gimp-image-new sheet-width sheet-height RGB)))
         (bg-layer (car (gimp-layer-new contact-sheet sheet-width sheet-height
                                       RGB-IMAGE "Background" 100 NORMAL-MODE)))
         (current-col 0)
         (current-row 0))

    (gimp-image-insert-layer contact-sheet bg-layer 0 0)
    (gimp-context-set-background '(255 255 255))
    (gimp-drawable-fill bg-layer BACKGROUND-FILL)

    (while (and (not (null? filelist)) (< current-row rows))
      (let* ((filename (car filelist))
             (thumb-img (car (gimp-file-load RUN-NONINTERACTIVE filename filename)))
             (thumb-layer (car (gimp-image-get-active-layer thumb-img)))
             (thumb-width (car (gimp-drawable-width thumb-layer)))
             (thumb-height (car (gimp-drawable-height thumb-layer)))
             (scale-factor (min (/ thumb-size thumb-width) (/ thumb-size thumb-height)))
             (new-width (* thumb-width scale-factor))
             (new-height (* thumb-height scale-factor))
             (x-pos (+ spacing (* current-col (+ thumb-size spacing))
                      (/ (- thumb-size new-width) 2)))
             (y-pos (+ spacing (* current-row (+ thumb-size spacing))
                      (/ (- thumb-size new-height) 2))))

        (gimp-image-scale thumb-img new-width new-height)

        (let ((copied-layer (car (gimp-layer-new-from-drawable
                                 (car (gimp-image-get-active-layer thumb-img))
                                 contact-sheet))))
          (gimp-image-insert-layer contact-sheet copied-layer 0 0)
          (gimp-layer-set-offsets copied-layer x-pos y-pos))

        (gimp-image-delete thumb-img)
        (set! current-col (+ current-col 1))

        (if (>= current-col cols)
            (begin
              (set! current-col 0)
              (set! current-row (+ current-row 1))))

        (set! filelist (cdr filelist))))

    (let ((final-layer (car (gimp-image-flatten contact-sheet))))
      (gimp-file-save RUN-NONINTERACTIVE contact-sheet final-layer
                     output-file output-file))

    (gimp-image-delete contact-sheet)))

; Usage: (batch-contact-sheet "/photos" "/contact_sheet.jpg" 4 3 10 200)
```

### 9. Metadata Embedder and Renamer

Organize photo libraries with consistent naming, date stamps, and XMP metadata:

```scheme
(define (batch-organize-photos input-dir output-dir prefix add-date add-size)
  (let* ((filelist (cadr (file-glob (string-append input-dir "/*.{jpg,jpeg}"))))
         (counter 1))
    (while (not (null? filelist))
      (let* ((filename (car filelist))
             (image (car (gimp-file-load RUN-NONINTERACTIVE filename filename)))
             (drawable (car (gimp-image-get-active-layer image)))
             (width (car (gimp-image-width image)))
             (height (car (gimp-image-height image)))
             (new-name (string-append prefix "_"
                       (if add-date
                           (string-append (strftime "%Y%m%d" (localtime (current-time))) "_")
                           "")
                       (if add-size
                           (string-append (number->string width) "x"
                                        (number->string height) "_")
                           "")
                       (format #f "~3,'0d" counter) ".jpg")))

        (gimp-image-set-metadata image "Xmp.dc.creator" "Batch Processed")
        (gimp-image-set-metadata image "Xmp.dc.rights" "Processed with GIMP")
        (gimp-image-set-metadata image "Xmp.dc.description"
                               (string-append "Original: "
                                            (car (reverse (string-split filename #\/)))))

        (let ((output-file (string-append output-dir "/" new-name)))
          (file-jpeg-save RUN-NONINTERACTIVE image drawable
                         output-file output-file 90 0 TRUE FALSE "" 0 TRUE 0 0))

        (gimp-image-delete image)
        (set! counter (+ counter 1))
        (set! filelist (cdr filelist))))))

; Usage: (batch-organize-photos "/unsorted" "/organized" "vacation" TRUE TRUE)
```

### 10. Advanced Layer Blending Processor

Create complex composite effects using multiple processing layers with blend modes (`vivid`, `soft`, `overlay`):

```scheme
(define (batch-advanced-blend input-dir output-dir blend-mode strength unsharp)
  (let* ((filelist (cadr (file-glob (string-append input-dir "/*.{jpg,jpeg,png}")))))
    (while (not (null? filelist))
      (let* ((filename (car filelist))
             (image (car (gimp-file-load RUN-NONINTERACTIVE filename filename)))
             (base-layer (car (gimp-image-get-active-layer image))))

        (let* ((contrast-layer (car (gimp-layer-copy base-layer FALSE)))
               (color-layer (car (gimp-layer-copy base-layer FALSE)))
               (detail-layer (car (gimp-layer-copy base-layer FALSE))))

          (gimp-image-insert-layer image contrast-layer 0 0)
          (gimp-image-insert-layer image color-layer 0 0)
          (gimp-image-insert-layer image detail-layer 0 0)

          (gimp-curves-spline contrast-layer HISTOGRAM-VALUE 8
                            #(0 0 64 70 192 185 255 255))
          (gimp-layer-set-mode contrast-layer OVERLAY-MODE)
          (gimp-layer-set-opacity contrast-layer (/ strength 2))

          (gimp-hue-saturation color-layer HUE-RANGE-ALL 0 0 15)
          (gimp-color-balance color-layer TRANSFER-MIDTONES TRUE 0 -5 5)
          (gimp-layer-set-mode color-layer
                              (cond ((string=? blend-mode "vivid") VIVID-LIGHT-MODE)
                                   ((string=? blend-mode "soft") SOFT-LIGHT-MODE)
                                   ((string=? blend-mode "overlay") OVERLAY-MODE)
                                   (else NORMAL-MODE)))
          (gimp-layer-set-opacity color-layer strength)

          (if unsharp
              (begin
                (plug-in-unsharp-mask RUN-NONINTERACTIVE image detail-layer
                                     1.5 1.0 0)
                (gimp-layer-set-mode detail-layer OVERLAY-MODE)
                (gimp-layer-set-opacity detail-layer 30)))

          (let ((mask (car (gimp-layer-create-mask detail-layer ADD-WHITE-MASK))))
            (gimp-layer-add-mask detail-layer mask)
            (plug-in-gauss RUN-NONINTERACTIVE image mask 2.0 2.0 TRUE))

          (let ((final-layer (car (gimp-image-flatten image)))
                (output-file (string-append output-dir "/enhanced_"
                             (car (reverse (string-split filename #\/))))))
            (gimp-file-save RUN-NONINTERACTIVE image final-layer
                           output-file output-file)))

        (gimp-image-delete image)
        (set! filelist (cdr filelist))))))

; Usage: (batch-advanced-blend "/photos" "/enhanced" "vivid" 40 TRUE)
```

### Setup and Performance Tips

- **Script location**: `~/.gimp-2.10/scripts/` (Linux/Mac) or `%APPDATA%\GIMP\2.10\scripts\` (Windows)
- **Refresh**: Filters > Script-Fu > Refresh Scripts (or restart GIMP)
- **Run**: Filters > Script-Fu > Console, then call the function with parameters
- **Memory**: always call `gimp-image-delete` after processing each image
- **Testing**: test on a small batch before running on thousands of files
- **Performance**: use `RUN-NONINTERACTIVE` to suppress all dialogs

## Related

- [`Document And Image Tools`](/developer-tools/reference/document-and-image-tools/)
