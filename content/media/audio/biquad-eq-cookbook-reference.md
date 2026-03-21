---
title: Biquad EQ Cookbook Reference
slug: /media/audio/biquad-eq-cookbook-reference/
summary: Reference for the common RBJ biquad filter formulas used in digital EQ implementations.
topic: media/audio
type: reference
tags: [audio, dsp, eq, filters, biquad]
aliases: [audio eq cookbook, rbj biquad]
platforms: [windows, linux, macos]
related:
  - /media/ffmpeg/conversion-and-frame-extraction/
status: published
updated: 2026-03-20
---

## Synopsis

This draft collects the recurring RBJ biquad equations that are useful when implementing low-pass, high-pass, shelving, and peaking filters.

## Syntax

```text
H(z) = (b0 + b1*z^-1 + b2*z^-2) / (a0 + a1*z^-1 + a2*z^-2)
```

## Parameters/Flags

- `Fs`: sample rate
- `f0`: center or cutoff frequency
- `Q`: quality factor
- `dBgain`: gain value for peaking and shelving filters

## Examples

Common intermediate values:

```text
A = 10^(dBgain/40)
w0 = 2*pi*f0/Fs
alpha = sin(w0)/(2*Q)
```

Low-pass filter coefficients:

```text
b0 = (1 - cos(w0))/2
b1 = 1 - cos(w0)
b2 = (1 - cos(w0))/2
a0 = 1 + alpha
a1 = -2*cos(w0)
a2 = 1 - alpha
```

## Related

- [`FFmpeg Conversion And Frame Extraction`](/media/ffmpeg/conversion-and-frame-extraction/)
