---
title: Core MIDI Reference
slug: /midi/reference/core-midi-reference/
summary: Consolidated MIDI basics, terminology, General MIDI maps, and note-frequency references for quick lookup.
topic: midi/reference
type: reference
tags: [midi, general-midi, notes, percussion, glossary]
aliases: [midi basics, gm patch map, midi note frequency chart, michael p bedesem midiplayer installation, midi-hits order id template]
platforms: [windows, macos, linux]
related:
  - /media/ffmpeg/conversion-and-frame-extraction/
status: published
updated: 2026-03-21
---

## Synopsis

This page combines the legacy MIDI intro notes, glossary material, General MIDI lookups, and note-frequency charts into one place.

### MIDI Basics

MIDI stands for Musical Instrument Digital Interface, a standard method for electronic musical equipment to pass messages to each other. These messages can be as simple as "play middle C until I tell you to stop" or more complex like "adjust the VCA bias on oscillator 6 to match oscillator 1."

Think of MIDI as an easily connected network between multiple devices. In MIDI there are thousands of electronic messages, but three -- Note On, System Exclusive, and Program Change -- are used by 99% of all MIDI applications.

The most basic MIDI function, Note On, tells the instruments what notes to play. In this regard, MIDI is similar to an electronic form of an old-style player piano, where a hole punched in a sheet of paper forced a key down on the piano's keyboard. MIDI performs this task electronically.

The designers of MIDI had a good bit of forethought and made the standard extensible through the use of System Exclusive commands. These commands are sent with an introductory special code that is ignored by all but the desired receiver.

Since synthesizers have the ability to alter the sounds of their musical output, a command called Program Change instructs them to change their program.

MIDI was developed in the early 1980s and proceeded to completely change the musical instrument market and the course of music. Its growth exceeded the wildest dreams of its inventors, and today, MIDI is entrenched to the point that you cannot buy a professional electric instrument without MIDI capabilities.

### How MIDI Works

MIDI specifies 16 separate MIDI Channels. Therefore, with one MIDI cable you can control up to 16 different instruments at once. The concept of MIDI Channels is similar to the idea of television channels. Each television station sends a signal within a particular frequency range. Your television set receives many different ranges (or channels) at once. You then tune your television set to a particular frequency range.

To relate this to MIDI, imagine you have a keyboard that sends out on MIDI Channel 7. You record a part into your sequencer. As you play back the sequence, you decide that you want to have the MIDI information control a synthesizer set to a trumpet sound. You would then set the synthesizer to receive on Channel 7, and the MIDI data from the sequencer would cause the synthesizer to play.

Both the sending device (e.g., the sequencer) and the receiving device (e.g., the synthesizer) must be set to the same MIDI Channel, or no sound will result. Drums are usually set on Channel 16 for base level and Channel 10 for extended level (General MIDI). Many different percussion sounds are on one Program Change Number (usually Number 0). To choose a particular percussion sound, use the Percussion Key Map to see which notes represent which percussion sounds, then enter those notes in the Drum Track.

The notes in the drum track indicate the type of percussion sound, such as Bass Drum, Splash Cymbal, or Maracas -- not the pitch. Entering a middle C (C4) note in the drum track plays a Hi Bongo, while entering an F above middle C (F4) plays a Low Conga, not a higher pitched Hi Bongo.

### MIDI Message Types

There are two MIDI Message types: Channel Messages and System Messages. A Channel Message includes a Channel number within the message. It is received and understood by any device set to that particular Channel, and ignored by any device set to a different Channel. The most basic Channel Message is a Note On Message.

A System Message is meant to be received and understood by all devices that are connected, regardless of their Channel setting. These messages control synchronization between devices, as well as special manufacturer-specific modes of operation.

A Program Change Message causes any devices tuned to the same Channel to change internal settings corresponding to the number sent. On many synthesizers, this causes a change in patch (or instrument sound). MIDI specifies a possible range of 128 Program Change numbers.

### Connecting MIDI Instruments

There are basically three MIDI cables: MIDI IN, MIDI OUT, and MIDI THRU. MIDI IN receives messages from the synth keyboard, MIDI OUT sends messages to the devices connected, and MIDI THRU sends a copy of the MIDI messages to all other devices connected via MIDI.

Example connection chain:

- The MIDI OUT on the computer is connected to the keyboard's MIDI IN.
- The MIDI IN on the computer is connected to the keyboard's MIDI OUT.
- The Keyboard's MIDI THRU is connected to the Synth Module's MIDI IN.
- The Synth Module's MIDI THRU is connected to the Drum Machine's MIDI IN.
- And so on (daisy chain).

### MIDI Standards (GM, GS, XG)

MIDI standards define rules for the exchange of musical data between electronic devices and software to ensure their compatibility. Regarding soundfonts, these standards specify a series of instruments along with the bank and preset numbers to be used for each. Several standards exist:

- **General MIDI (GM)** -- The most well-known, introduced in 1991. Defines 128 instrument patches and a standard percussion map on Channel 10.
- **Roland MT-32** -- Defined in 1987, predates GM. Has its own unique 128-instrument ordering.
- **Roland GS** -- Introduced in 1991 (extended 1994, 1996). Superset of GM with additional bank-select variations. Sound sets: SC-55 (1), SC-88 (2), SC-88 Pro (3).
- **Yamaha XG** -- Introduced in 1994. Superset of GM with additional bank-select variations and extended effects.

## Syntax

```text
MIDI channel: 1-16
General MIDI percussion: channel 10
Middle C: note 60
Note range: 0-127
Velocity range: 0-127 (0 = Note Off)
Program Change range: 0-127
Controller range: 0-127
```

## Parameters/Flags

- `Program Change`: selects an instrument patch (0-127)
- `Channel 10`: General MIDI percussion channel
- `Note Number`: numeric pitch representation from 0 to 127
- `Frequency`: note pitch in hertz
- `Velocity`: how hard a note is struck (0-127; 0 means Note Off)
- `Control Change (CC)`: sends controller values (mod wheel, volume, pan, etc.)
- `Pitch Bend`: 14-bit value; center (no bend) = 0x2000
- `Aftertouch`: pressure applied after a key is already down
- `System Exclusive (SysEx)`: manufacturer-specific data dumps and extensions

## Examples

### MIDI 1.0 Specification Message Summary

Updated 1995 by the MIDI Manufacturers Association.

#### Channel Voice Messages

`nnnn` = 0-15 (MIDI Channel Number 1-16)

| Status Byte | Data Byte(s) | Description |
|---|---|---|
| `1000nnnn` | `0kkkkkkk 0vvvvvvv` | **Note Off.** Sent when a note is released. k = key (note) number, v = velocity. |
| `1001nnnn` | `0kkkkkkk 0vvvvvvv` | **Note On.** Sent when a note is depressed. k = key (note) number, v = velocity. |
| `1010nnnn` | `0kkkkkkk 0vvvvvvv` | **Polyphonic Key Pressure (Aftertouch).** Sent by pressing down on a key after it bottoms out. k = key number, v = pressure value. |
| `1011nnnn` | `0ccccccc 0vvvvvvv` | **Control Change.** Sent when a controller value changes (pedals, levers). Controller numbers 120-127 are reserved as Channel Mode Messages. c = controller number (0-119), v = controller value (0-127). |
| `1100nnnn` | `0ppppppp` | **Program Change.** Sent when the patch number changes. p = new program number. |
| `1101nnnn` | `0vvvvvvv` | **Channel Pressure (Aftertouch).** Sends the single greatest pressure value of all currently depressed keys. v = pressure value. |
| `1110nnnn` | `0lllllll 0mmmmmmm` | **Pitch Wheel Change.** 14-bit value. Center (no pitch change) = 2000H. l = least significant 7 bits, m = most significant 7 bits. |

#### Channel Mode Messages

Same status byte as Control Change (`1011nnnn`) but uses reserved controller numbers 120-127:

| Controller | Value | Function |
|---|---|---|
| 120 | 0 | All Sound Off |
| 121 | 0 | Reset All Controllers |
| 122 | 0 | Local Control Off |
| 122 | 127 | Local Control On |
| 123 | 0 | All Notes Off |
| 124 | 0 | Omni Mode Off (+ All Notes Off) |
| 125 | 0 | Omni Mode On (+ All Notes Off) |
| 126 | M | Mono Mode On (Poly Off); M = number of channels (Omni Off) or 0 (Omni On) |
| 127 | 0 | Poly Mode On (Mono Off) (+ All Notes Off) |

#### System Common Messages

| Status Byte | Data Byte(s) | Description |
|---|---|---|
| `11110000` | `0iiiiiii 0ddddddd ... 11110111` | **System Exclusive.** i = Manufacturer's ID code. If recognized, device listens to data (d). Terminated by End of Exclusive (F7). Also used for Universal Exclusive Messages. |
| `11110010` | `0lllllll 0mmmmmmm` | **Song Position Pointer.** 14-bit register holding number of MIDI beats (1 beat = 6 MIDI clocks) since song start. l = LSB, m = MSB. |
| `11110011` | `0sssssss` | **Song Select.** Specifies which sequence or song is to be played. |
| `11110110` | (none) | **Tune Request.** All analog synthesizers should tune their oscillators. |
| `11110111` | (none) | **End of Exclusive.** Terminates a System Exclusive dump. |

#### System Real-Time Messages

| Status Byte | Description |
|---|---|
| `11111000` | **Timing Clock.** Sent 24 times per quarter note when synchronization is required. |
| `11111010` | **Start.** Start the current sequence playing (followed with Timing Clocks). |
| `11111011` | **Continue.** Continue at the point the sequence was stopped. |
| `11111100` | **Stop.** Stop the current sequence. |
| `11111110` | **Active Sensing.** Optional. Once sent, receiver expects another within 300ms max, or assumes connection terminated. |
| `11111111` | **Reset.** Reset all receivers to power-up status. Use sparingly, preferably under manual control. |

### Expanded Status Bytes Reference

| 1st Byte (Binary) | Hex | Dec | Function | 2nd Byte | 3rd Byte |
|---|---|---|---|---|---|
| `10000000` | 80 | 128 | Chan 1 Note Off | Note Number (0-127) | Note Velocity (0-127) |
| `10000001`-`10001111` | 81-8F | 129-143 | Chan 2-16 Note Off | Note Number | Note Velocity |
| `10010000` | 90 | 144 | Chan 1 Note On | Note Number (0-127) | Note Velocity (0-127) |
| `10010001`-`10011111` | 91-9F | 145-159 | Chan 2-16 Note On | Note Number | Note Velocity |
| `10100000` | A0 | 160 | Chan 1 Polyphonic Aftertouch | Note Number (0-127) | Aftertouch amount (0-127) |
| `10100001`-`10101111` | A1-AF | 161-175 | Chan 2-16 Polyphonic Aftertouch | Note Number | Aftertouch amount |
| `10110000` | B0 | 176 | Chan 1 Control/Mode Change | See Controller Table | See Controller Table |
| `10110001`-`10111111` | B1-BF | 177-191 | Chan 2-16 Control/Mode Change | See Controller Table | See Controller Table |

### MIDI Controller Numbers (CC 0-127)

Updated 1995 by the MIDI Manufacturers Association. Status bytes 176-191 (Control/Mode Change).

#### MSB Controllers (CC 0-31)

| CC# | Hex | Function |
|---|---|---|
| 0 | 00 | Bank Select |
| 1 | 01 | Modulation Wheel |
| 2 | 02 | Breath Control |
| 3 | 03 | Undefined |
| 4 | 04 | Foot Controller |
| 5 | 05 | Portamento Time |
| 6 | 06 | Data Entry |
| 7 | 07 | Channel Volume (formerly Main Volume) |
| 8 | 08 | Balance |
| 9 | 09 | Undefined |
| 10 | 0A | Pan |
| 11 | 0B | Expression Controller |
| 12 | 0C | Effect Control 1 |
| 13 | 0D | Effect Control 2 |
| 14-15 | 0E-0F | Undefined |
| 16 | 10 | General Purpose Controller #1 |
| 17 | 11 | General Purpose Controller #2 |
| 18 | 12 | General Purpose Controller #3 |
| 19 | 13 | General Purpose Controller #4 |
| 20-31 | 14-1F | Undefined |

#### LSB Controllers (CC 32-63)

| CC# | Hex | Function |
|---|---|---|
| 32 | 20 | Bank Select LSB |
| 33 | 21 | Modulation Wheel LSB |
| 34 | 22 | Breath Control LSB |
| 35 | 23 | Undefined LSB |
| 36 | 24 | Foot Controller LSB |
| 37 | 25 | Portamento Time LSB |
| 38 | 26 | Data Entry LSB |
| 39 | 27 | Channel Volume LSB |
| 40 | 28 | Balance LSB |
| 41 | 29 | Undefined LSB |
| 42 | 2A | Pan LSB |
| 43 | 2B | Expression Controller LSB |
| 44 | 2C | Effect Control 1 LSB |
| 45 | 2D | Effect Control 2 LSB |
| 46-47 | 2E-2F | Undefined LSB |
| 48 | 30 | General Purpose Controller #1 LSB |
| 49 | 31 | General Purpose Controller #2 LSB |
| 50 | 32 | General Purpose Controller #3 LSB |
| 51 | 33 | General Purpose Controller #4 LSB |
| 52-63 | 34-3F | Undefined LSB |

#### Switch and Mode Controllers (CC 64-127)

| CC# | Hex | Function | Value |
|---|---|---|---|
| 64 | 40 | Damper Pedal (Sustain) | 0-63 off, 64-127 on |
| 65 | 41 | Portamento On/Off | 0-63 off, 64-127 on |
| 66 | 42 | Sostenuto | 0-63 off, 64-127 on |
| 67 | 43 | Soft Pedal | 0-63 off, 64-127 on |
| 68 | 44 | Legato Footswitch | 0-63 off, 64-127 on |
| 69 | 45 | Hold 2 | 0-63 off, 64-127 on |
| 70 | 46 | Sound Controller 1 (Sound Variation) | 0-127 |
| 71 | 47 | Sound Controller 2 (Timbre/Harmonic Content) | 0-127 |
| 72 | 48 | Sound Controller 3 (Release Time) | 0-127 |
| 73 | 49 | Sound Controller 4 (Attack Time) | 0-127 |
| 74 | 4A | Sound Controller 5 (Brightness) | 0-127 |
| 75-79 | 4B-4F | Sound Controllers 6-10 | 0-127 |
| 80-83 | 50-53 | General Purpose Controllers 5-8 | 0-127 |
| 84 | 54 | Portamento Control | 0-127 |
| 85-90 | 55-5A | Undefined | -- |
| 91 | 5B | Effects 1 Depth (Reverb Send Level) | 0-127 |
| 92 | 5C | Effects 2 Depth (Tremolo Depth) | 0-127 |
| 93 | 5D | Effects 3 Depth (Chorus Send Level) | 0-127 |
| 94 | 5E | Effects 4 Depth (Celeste/Detune Depth) | 0-127 |
| 95 | 5F | Effects 5 Depth (Phaser Depth) | 0-127 |
| 96 | 60 | Data Increment | -- |
| 97 | 61 | Data Decrement | -- |
| 98 | 62 | Non-Registered Parameter Number LSB | 0-127 |
| 99 | 63 | Non-Registered Parameter Number MSB | 0-127 |
| 100 | 64 | Registered Parameter Number LSB | 0-127 |
| 101 | 65 | Registered Parameter Number MSB | 0-127 |
| 102-119 | 66-77 | Undefined | -- |
| 120 | 78 | All Sound Off | 0 |
| 121 | 79 | Reset All Controllers | 0 |
| 122 | 7A | Local Control On/Off | 0 off, 127 on |
| 123 | 7B | All Notes Off | 0 |
| 124 | 7C | Omni Mode Off | 0 |
| 125 | 7D | Omni Mode On | 0 |
| 126 | 7E | Mono Mode On (Poly Off) | M = # channels |
| 127 | 7F | Poly Mode On (Mono Off) | 0 |

### MIDI Velocity / dB / Dynamics Mapping

MIDI velocity (V) values are linear. dB calculations are logarithmic: `dB = 40 * log(V / 127)`. Musical dynamics are average values.

| Velocity | Dynamic | dB | dB (%) | Diff (dB) |
|---|---|---|---|---|
| 127 | fff | 0.0 | 100 | 0 |
| 112 | ff | -2.2 | 86 | -2.2 |
| 96 | f | -4.9 | 71 | -2.7 |
| 80 | mf | -8.0 | 57 | -3.1 |
| 72 | m (theoretical) | -10.0 | 50 | -- |
| 64 | mp | -11.9 | 44 | -3.9 |
| 48 | p | -16.9 | 31 | -5.0 |
| 32 | pp | -23.9 | 19 | -7.0 |
| 16 | ppp | -36.0 | 8 | -12.1 |
| 0 | off | -57.6 | 0 | -21.6 |

Notes:

- Velocity 0 means Note Off, so the usable dynamic range is 1-127.
- The theoretical "mezzo" (m) at velocity 72 = -10 dB = 50%. Band-in-a-Box uses 72 as default velocity.
- Velocity ranges per dynamic: 0 = off, 1-16 = ppp, 17-32 = pp, 33-48 = p, 49-63 = mp, 64-79 = mf, 80-95 = f, 96-111 = ff, 112-127 = fff.
- To get real 0 dB: MIDI master volume 127, expression 127; set your audio mixer's master volume accordingly.

### General MIDI Channel Map

The standard MIDI patch assignments for authoring MIDI files for use with Windows are based on the MIDI Manufacturers Association (MMA) General MIDI Mode specification.

> **Numbering note:** The Prog# column below uses 1-based numbering (as displayed by most sequencers and modules). The actual MIDI Program Change byte sent is one less (e.g., Prog# 120 sends value 119). The Win32 / zero-based equivalent is shown in the second column.

### General MIDI Instrument Patch Map (All 128 Patches)

#### Piano (Prog# 1-8)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 1 | 0 | Acoustic Grand Piano |
| 2 | 1 | Bright Acoustic Piano |
| 3 | 2 | Electric Grand Piano |
| 4 | 3 | Honky-Tonk Piano |
| 5 | 4 | Electric Piano 1 |
| 6 | 5 | Electric Piano 2 |
| 7 | 6 | Harpsichord |
| 8 | 7 | Clavinet |

#### Chromatic Percussion (Prog# 9-16)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 9 | 8 | Celesta |
| 10 | 9 | Glockenspiel |
| 11 | 10 | Music Box |
| 12 | 11 | Vibraphone |
| 13 | 12 | Marimba |
| 14 | 13 | Xylophone |
| 15 | 14 | Tubular Bells |
| 16 | 15 | Dulcimer |

#### Organ (Prog# 17-24)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 17 | 16 | Drawbar Organ |
| 18 | 17 | Percussive Organ |
| 19 | 18 | Rock Organ |
| 20 | 19 | Church Organ |
| 21 | 20 | Reed Organ |
| 22 | 21 | Accordion |
| 23 | 22 | Harmonica |
| 24 | 23 | Tango Accordion |

#### Guitar (Prog# 25-32)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 25 | 24 | Acoustic Guitar (nylon) |
| 26 | 25 | Acoustic Guitar (steel) |
| 27 | 26 | Electric Guitar (jazz) |
| 28 | 27 | Electric Guitar (clean) |
| 29 | 28 | Electric Guitar (muted) |
| 30 | 29 | Overdriven Guitar |
| 31 | 30 | Distortion Guitar |
| 32 | 31 | Guitar Harmonics |

#### Bass (Prog# 33-40)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 33 | 32 | Acoustic Bass |
| 34 | 33 | Electric Bass (finger) |
| 35 | 34 | Electric Bass (pick) |
| 36 | 35 | Fretless Bass |
| 37 | 36 | Slap Bass 1 |
| 38 | 37 | Slap Bass 2 |
| 39 | 38 | Synth Bass 1 |
| 40 | 39 | Synth Bass 2 |

#### Solo Strings (Prog# 41-48)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 41 | 40 | Violin |
| 42 | 41 | Viola |
| 43 | 42 | Cello |
| 44 | 43 | Contrabass |
| 45 | 44 | Tremolo Strings |
| 46 | 45 | Pizzicato Strings |
| 47 | 46 | Orchestral Harp |
| 48 | 47 | Timpani |

#### Ensemble (Prog# 49-56)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 49 | 48 | String Ensemble 1 |
| 50 | 49 | String Ensemble 2 |
| 51 | 50 | Synth Strings 1 |
| 52 | 51 | Synth Strings 2 |
| 53 | 52 | Choir Aahs |
| 54 | 53 | Voice Oohs |
| 55 | 54 | Synth Voice |
| 56 | 55 | Orchestra Hit |

#### Brass (Prog# 57-64)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 57 | 56 | Trumpet |
| 58 | 57 | Trombone |
| 59 | 58 | Tuba |
| 60 | 59 | Muted Trumpet |
| 61 | 60 | French Horn |
| 62 | 61 | Brass Section |
| 63 | 62 | Synth Brass 1 |
| 64 | 63 | Synth Brass 2 |

#### Reed (Prog# 65-72)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 65 | 64 | Soprano Sax |
| 66 | 65 | Alto Sax |
| 67 | 66 | Tenor Sax |
| 68 | 67 | Baritone Sax |
| 69 | 68 | Oboe |
| 70 | 69 | English Horn |
| 71 | 70 | Bassoon |
| 72 | 71 | Clarinet |

#### Pipe (Prog# 73-80)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 73 | 72 | Piccolo |
| 74 | 73 | Flute |
| 75 | 74 | Recorder |
| 76 | 75 | Pan Flute |
| 77 | 76 | Blown Bottle |
| 78 | 77 | Shakuhachi |
| 79 | 78 | Whistle |
| 80 | 79 | Ocarina |

#### Synth Lead (Prog# 81-88)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 81 | 80 | Lead 1 (square) |
| 82 | 81 | Lead 2 (sawtooth) |
| 83 | 82 | Lead 3 (calliope) |
| 84 | 83 | Lead 4 (chiff) |
| 85 | 84 | Lead 5 (charang) |
| 86 | 85 | Lead 6 (voice) |
| 87 | 86 | Lead 7 (fifths) |
| 88 | 87 | Lead 8 (bass + lead) |

#### Synth Pad (Prog# 89-96)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 89 | 88 | Pad 1 (new age) |
| 90 | 89 | Pad 2 (warm) |
| 91 | 90 | Pad 3 (polysynth) |
| 92 | 91 | Pad 4 (choir) |
| 93 | 92 | Pad 5 (bowed) |
| 94 | 93 | Pad 6 (metallic) |
| 95 | 94 | Pad 7 (halo) |
| 96 | 95 | Pad 8 (sweep) |

#### Synth Effects (Prog# 97-104)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 97 | 96 | FX 1 (rain) |
| 98 | 97 | FX 2 (soundtrack) |
| 99 | 98 | FX 3 (crystal) |
| 100 | 99 | FX 4 (atmosphere) |
| 101 | 100 | FX 5 (brightness) |
| 102 | 101 | FX 6 (goblins) |
| 103 | 102 | FX 7 (echoes) |
| 104 | 103 | FX 8 (sci-fi) |

#### Ethnic (Prog# 105-112)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 105 | 104 | Sitar |
| 106 | 105 | Banjo |
| 107 | 106 | Shamisen |
| 108 | 107 | Koto |
| 109 | 108 | Kalimba |
| 110 | 109 | Bagpipe |
| 111 | 110 | Fiddle |
| 112 | 111 | Shanai |

#### Percussive (Prog# 113-120)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 113 | 112 | Tinkle Bell |
| 114 | 113 | Agogo |
| 115 | 114 | Steel Drums |
| 116 | 115 | Woodblock |
| 117 | 116 | Taiko Drum |
| 118 | 117 | Melodic Tom |
| 119 | 118 | Synth Drum |
| 120 | 119 | Reverse Cymbal |

#### Sound Effects (Prog# 121-128)

| Prog# | PC (0-based) | Instrument |
|---|---|---|
| 121 | 120 | Guitar Fret Noise |
| 122 | 121 | Breath Noise |
| 123 | 122 | Seashore |
| 124 | 123 | Bird Tweet |
| 125 | 124 | Telephone Ring |
| 126 | 125 | Helicopter |
| 127 | 126 | Applause |
| 128 | 127 | Gunshot |

### General MIDI Percussion Key Map (Channel 10)

All 47 GM percussion instruments, mapped by MIDI note number. Note/Octave column shows the corresponding piano key.

| Note# | Drum Sound | Note/Octave |
|---|---|---|
| 35 | Acoustic Bass Drum | B0 |
| 36 | Bass Drum 1 | C1 |
| 37 | Side Stick | C#1 |
| 38 | Acoustic Snare | D1 |
| 39 | Hand Clap | D#1 |
| 40 | Electric Snare | E1 |
| 41 | Low Floor Tom | F1 |
| 42 | Closed Hi-Hat | F#1 |
| 43 | High Floor Tom | G1 |
| 44 | Pedal Hi-Hat | G#1 |
| 45 | Low Tom | A1 |
| 46 | Open Hi-Hat | A#1 |
| 47 | Low-Mid Tom | B1 |
| 48 | Hi-Mid Tom | C2 |
| 49 | Crash Cymbal 1 | C#2 |
| 50 | High Tom | D2 |
| 51 | Ride Cymbal 1 | D#2 |
| 52 | Chinese Cymbal | E2 |
| 53 | Ride Bell | F2 |
| 54 | Tambourine | F#2 |
| 55 | Splash Cymbal | G2 |
| 56 | Cowbell | G#2 |
| 57 | Crash Cymbal 2 | A2 |
| 58 | Vibraslap | A#2 |
| 59 | Ride Cymbal 2 | B2 |
| 60 | Hi Bongo | C3 |
| 61 | Low Bongo | C#3 |
| 62 | Mute Hi Conga | D3 |
| 63 | Open Hi Conga | D#3 |
| 64 | Low Conga | E3 |
| 65 | High Timbale | F3 |
| 66 | Low Timbale | F#3 |
| 67 | High Agogo | G3 |
| 68 | Low Agogo | G#3 |
| 69 | Cabasa | A3 |
| 70 | Maracas | A#3 |
| 71 | Short Whistle | B3 |
| 72 | Long Whistle | C4 |
| 73 | Short Guiro | C#4 |
| 74 | Long Guiro | D4 |
| 75 | Claves | D#4 |
| 76 | Hi Wood Block | E4 |
| 77 | Low Wood Block | F4 |
| 78 | Mute Cuica | F#4 |
| 79 | Open Cuica | G4 |
| 80 | Mute Triangle | G#4 |
| 81 | Open Triangle | A4 |

**Percussion behavior notes:**

- A note-on with note 42 (Closed Hi-Hat) should cut off any Open Hi-Hat or Pedal Hi-Hat that may be sustaining, and vice versa. Only one of these three hi-hat sounds can be sounding at any given time.
- Similarly: Short Whistle cuts off Long Whistle. Short Guiro cuts off Long Guiro. Mute Triangle cuts off Open Triangle. Mute Cuica cuts off Open Cuica.
- Normally, all drum sounds have a fixed duration regardless of Note-Off timing. Exceptions may be Long Whistle and Long Guiro, which may use the Note-On to Note-Off duration.
- If a drum is still sounding when another Note-On for the same drum is received, typically another voice stacks another instance of that sound.

### MIDI Note Numbers by Octave

| Octave | C | C# | D | D# | E | F | F# | G | G# | A | A# | B |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| -1 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
| 0 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 |
| 1 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 |
| 2 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 |
| 3 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 |
| 4 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 |
| 5 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 |
| 6 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 |
| 7 | 96 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 |
| 8 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 |
| 9 | 120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 | -- | -- | -- | -- |

Middle C = note 60 (C4). A440 = note 69 (A4).

### MIDI Note Number to Frequency Conversion Chart

Frequency is in Hertz (Hz). Tuning based on A4 = 440 Hz.

| Note | MIDI# | Frequency (Hz) | | Note | MIDI# | Frequency (Hz) | | Note | MIDI# | Frequency (Hz) |
|---|---|---|---|---|---|---|---|---|---|---|
| C | 0 | 8.1758 | | C | 12 | 16.3516 | | C | 24 | 32.7032 |
| Db | 1 | 8.6620 | | Db | 13 | 17.3239 | | Db | 25 | 34.6478 |
| D | 2 | 9.1770 | | D | 14 | 18.3540 | | D | 26 | 36.7081 |
| Eb | 3 | 9.7227 | | Eb | 15 | 19.4454 | | Eb | 27 | 38.8909 |
| E | 4 | 10.3009 | | E | 16 | 20.6017 | | E | 28 | 41.2034 |
| F | 5 | 10.9134 | | F | 17 | 21.8268 | | F | 29 | 43.6535 |
| Gb | 6 | 11.5623 | | Gb | 18 | 23.1247 | | Gb | 30 | 46.2493 |
| G | 7 | 12.2499 | | G | 19 | 24.4997 | | G | 31 | 48.9994 |
| Ab | 8 | 12.9783 | | Ab | 20 | 25.9565 | | Ab | 32 | 51.9131 |
| A | 9 | 13.7500 | | A | 21 | 27.5000 | | A | 33 | 55.0000 |
| Bb | 10 | 14.5676 | | Bb | 22 | 29.1352 | | Bb | 34 | 58.2705 |
| B | 11 | 15.4339 | | B | 23 | 30.8677 | | B | 35 | 61.7354 |

| Note | MIDI# | Frequency (Hz) | | Note | MIDI# | Frequency (Hz) | | Note | MIDI# | Frequency (Hz) |
|---|---|---|---|---|---|---|---|---|---|---|
| C | 36 | 65.4064 | | C | 48 | 130.8128 | | C | 60 | 261.6256 |
| Db | 37 | 69.2957 | | Db | 49 | 138.5913 | | Db | 61 | 277.1826 |
| D | 38 | 73.4162 | | D | 50 | 146.8324 | | D | 62 | 293.6648 |
| Eb | 39 | 77.7817 | | Eb | 51 | 155.5635 | | Eb | 63 | 311.1270 |
| E | 40 | 82.4069 | | E | 52 | 164.8138 | | E | 64 | 329.6276 |
| F | 41 | 87.3071 | | F | 53 | 174.6141 | | F | 65 | 349.2282 |
| Gb | 42 | 92.4986 | | Gb | 54 | 184.9972 | | Gb | 66 | 369.9944 |
| G | 43 | 97.9989 | | G | 55 | 195.9977 | | G | 67 | 391.9954 |
| Ab | 44 | 103.8262 | | Ab | 56 | 207.6523 | | Ab | 68 | 415.3047 |
| A | 45 | 110.0000 | | A | 57 | 220.0000 | | A | 69 | 440.0000 |
| Bb | 46 | 116.5409 | | Bb | 58 | 233.0819 | | Bb | 70 | 466.1638 |
| B | 47 | 123.4708 | | B | 59 | 246.9417 | | B | 71 | 493.8833 |

| Note | MIDI# | Frequency (Hz) | | Note | MIDI# | Frequency (Hz) | | Note | MIDI# | Frequency (Hz) |
|---|---|---|---|---|---|---|---|---|---|---|
| C | 72 | 523.2511 | | C | 84 | 1046.5023 | | C | 96 | 2093.0045 |
| Db | 73 | 554.3653 | | Db | 85 | 1108.7305 | | Db | 97 | 2217.4610 |
| D | 74 | 587.3295 | | D | 86 | 1174.6591 | | D | 98 | 2349.3181 |
| Eb | 75 | 622.2540 | | Eb | 87 | 1244.5079 | | Eb | 99 | 2489.0159 |
| E | 76 | 659.2551 | | E | 88 | 1318.5102 | | E | 100 | 2637.0205 |
| F | 77 | 698.4565 | | F | 89 | 1396.9129 | | F | 101 | 2793.8259 |
| Gb | 78 | 739.9888 | | Gb | 90 | 1479.9777 | | Gb | 102 | 2959.9554 |
| G | 79 | 783.9909 | | G | 91 | 1567.9817 | | G | 103 | 3135.9635 |
| Ab | 80 | 830.6094 | | Ab | 92 | 1661.2188 | | Ab | 104 | 3322.4376 |
| A | 81 | 880.0000 | | A | 93 | 1760.0000 | | A | 105 | 3520.0000 |
| Bb | 82 | 932.3275 | | Bb | 94 | 1864.6550 | | Bb | 106 | 3729.3101 |
| B | 83 | 987.7666 | | B | 95 | 1975.5332 | | B | 107 | 3951.0664 |

| Note | MIDI# | Frequency (Hz) | | Note | MIDI# | Frequency (Hz) |
|---|---|---|---|---|---|---|
| C | 108 | 4186.0090 | | C | 120 | 8372.0181 |
| Db | 109 | 4434.9221 | | Db | 121 | 8869.8442 |
| D | 110 | 4698.6363 | | D | 122 | 9397.2726 |
| Eb | 111 | 4978.0317 | | Eb | 123 | 9956.0635 |
| E | 112 | 5274.0409 | | E | 124 | 10548.0818 |
| F | 113 | 5587.6517 | | F | 125 | 11175.3034 |
| Gb | 114 | 5919.9108 | | Gb | 126 | 11839.8215 |
| G | 115 | 6271.9270 | | G | 127 | 12543.8540 |
| Ab | 116 | 6644.8752 | | | | |
| A | 117 | 7040.0000 | | | | |
| Bb | 118 | 7458.6202 | | | | |
| B | 119 | 7902.1328 | | | | |

**Frequency calculation formula (BASIC):**

```basic
DIM MIDI(127)
A = 440
FOR x = 0 TO 127
  MIDI(x) = (A / 32) * (2 ^ ((x - 9) / 12))
NEXT x
```

Simplified for A=440 tuning:

```basic
DIM MIDI(127)
FOR x = 0 TO 127
  MIDI(x) = 8.1758 * 2 ^ (x / 12)
NEXT x
```

### Note Frequency Quick-Reference (Rounded)

Octave rows, frequency in Hz (rounded values for quick lookup):

| Octave | C | C# | D | Eb | E | F | F# | G | G# | A | Bb | B |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | 16.35 | 17.32 | 18.35 | 19.45 | 20.60 | 21.83 | 23.12 | 24.50 | 25.96 | 27.50 | 29.14 | 30.87 |
| 1 | 32.70 | 34.65 | 36.71 | 38.89 | 41.20 | 43.65 | 46.25 | 49.00 | 51.91 | 55.00 | 58.27 | 61.74 |
| 2 | 65.41 | 69.30 | 73.42 | 77.78 | 82.41 | 87.31 | 92.50 | 98.00 | 103.8 | 110.0 | 116.5 | 123.5 |
| 3 | 130.8 | 138.6 | 146.8 | 155.6 | 164.8 | 174.6 | 185.0 | 196.0 | 207.7 | 220.0 | 233.1 | 246.9 |
| 4 | 261.6 | 277.2 | 293.7 | 311.1 | 329.6 | 349.2 | 370.0 | 392.0 | 415.3 | 440.0 | 466.2 | 493.9 |
| 5 | 523.3 | 554.4 | 587.3 | 622.3 | 659.3 | 698.5 | 740.0 | 784.0 | 830.6 | 880.0 | 932.3 | 987.8 |
| 6 | 1047 | 1109 | 1175 | 1245 | 1319 | 1397 | 1480 | 1568 | 1661 | 1760 | 1865 | 1976 |
| 7 | 2093 | 2217 | 2349 | 2489 | 2637 | 2794 | 2960 | 3136 | 3322 | 3520 | 3729 | 3951 |
| 8 | 4186 | 4435 | 4699 | 4978 | 5274 | 5588 | 5920 | 6272 | 6645 | 7040 | 7459 | 7902 |

### Roland MT-32 Patch Map

The MT-32 standard (1987) predates General MIDI and has its own unique instrument ordering:

| Preset | Instrument | | Preset | Instrument |
|---|---|---|---|---|
| 0 | Acoustic Piano 1 | | 32 | Fantasy |
| 1 | Acoustic Piano 2 | | 33 | Harmo Pan |
| 2 | Acoustic Piano 3 | | 34 | Chorale |
| 3 | Electronic Piano 1 | | 35 | Glasses |
| 4 | Electronic Piano 2 | | 36 | Soundtrack |
| 5 | Electronic Piano 3 | | 37 | Atmosphere |
| 6 | Electronic Piano 4 | | 38 | Warm Bell |
| 7 | Honkytonk | | 39 | Funny Vox |
| 8 | Electronic Organ 1 | | 40 | Echo Bell |
| 9 | Electronic Organ 2 | | 41 | Ice Rain |
| 10 | Electronic Organ 3 | | 42 | Oboe 2001 |
| 11 | Electronic Organ 4 | | 43 | Echo Pan |
| 12 | Pipe Organ 1 | | 44 | Doctor Solo |
| 13 | Pipe Organ 2 | | 45 | School Daze |
| 14 | Pipe Organ 3 | | 46 | Bellsinger |
| 15 | Accordion | | 47 | Square Wave |
| 16 | Harpsichord 1 | | 48 | String Section 1 |
| 17 | Harpsichord 2 | | 49 | String Section 2 |
| 18 | Harpsichord 3 | | 50 | String Section 3 |
| 19 | Clavinet 1 | | 51 | Pizzicato |
| 20 | Clavinet 2 | | 52 | Violin 1 |
| 21 | Clavinet 3 | | 53 | Violin 2 |
| 22 | Celesta 1 | | 54 | Cello 1 |
| 23 | Celesta 2 | | 55 | Cello 2 |
| 24 | Synth Brass 1 | | 56 | Contrabass |
| 25 | Synth Brass 2 | | 57 | Harp 1 |
| 26 | Synth Brass 3 | | 58 | Harp 2 |
| 27 | Synth Brass 4 | | 59 | Guitar 1 |
| 28 | Synth Bass 1 | | 60 | Guitar 2 |
| 29 | Synth Bass 2 | | 61 | Electric Gtr 1 |
| 30 | Synth Bass 3 | | 62 | Electric Gtr 2 |
| 31 | Synth Bass 4 | | 63 | Sitar |

| Preset | Instrument | | Preset | Instrument |
|---|---|---|---|---|
| 64 | Acoustic Bass 1 | | 96 | Brass Section 2 |
| 65 | Acoustic Bass 2 | | 97 | Vibe 1 |
| 66 | Electric Bass 1 | | 98 | Vibe 2 |
| 67 | Electric Bass 2 | | 99 | Synth Mallet |
| 68 | Slap Bass 1 | | 100 | Windbell |
| 69 | Slap Bass 2 | | 101 | Glock |
| 70 | Fretless 1 | | 102 | Tube Bell |
| 71 | Fretless 2 | | 103 | Xylophone |
| 72 | Flute 1 | | 104 | Marimba |
| 73 | Flute 2 | | 105 | Koto |
| 74 | Piccolo 1 | | 106 | Sho |
| 75 | Piccolo 2 | | 107 | Shakuhachi |
| 76 | Recorder | | 108 | Whistle 1 |
| 77 | Pan Pipes | | 109 | Whistle 2 |
| 78 | Sax 1 | | 110 | Bottleblow |
| 79 | Sax 2 | | 111 | Breathpipe |
| 80 | Sax 3 | | 112 | Timpani |
| 81 | Sax 4 | | 113 | Melodic Tom |
| 82 | Clarinet 1 | | 114 | Deep Snare |
| 83 | Clarinet 2 | | 115 | Elec Perc 1 |
| 84 | Oboe | | 116 | Elec Perc 2 |
| 85 | English Horn | | 117 | Taiko |
| 86 | Bassoon | | 118 | Taiko Rim |
| 87 | Harmonica | | 119 | Cymbal |
| 88 | Trumpet 1 | | 120 | Castanets |
| 89 | Trumpet 2 | | 121 | Triangle |
| 90 | Trombone 1 | | 122 | Orchestral Hit |
| 91 | Trombone 2 | | 123 | Telephone |
| 92 | French Horn 1 | | 124 | Bird Tweet |
| 93 | French Horn 2 | | 125 | One Note Jam |
| 94 | Tuba | | 126 | Water Bell |
| 95 | Brass Section 1 | | 127 | Jungle Tune |

### MIDI Glossary

| Term | Definition |
|---|---|
| ADSR | Attack, Decay, Sustain, Release -- the four phases of a sound envelope. |
| Aftertouch | A MIDI message generated by further pressure on a key that is already depressed -- often vibrato or other sound modulation. |
| Aliasing / Foldover | Distortion caused in digital synthesis when frequencies higher than 1/2 the sample rate are created. They fold over, generating undesirable lower frequencies. |
| Alphanumeric | Any member of the set of all numbers, letters, and other characters used by the computer. |
| Amplifier | A device which controls the volume or loudness of a signal. |
| Amplitude | The distance between the rest position and the farthest point to which a vibrating object moves. As amplitude increases, so does perceived volume. |
| Amplitude Modulation (AM) | A variance of the amplitude of one signal (the carrier) in accordance with the frequency and amplitude of a second signal (the program). |
| Analog-to-Digital Converter (ADC) | A device that converts an analog signal into a series of digital values, such as a sampler. |
| Analog Recording | Using voltages to represent acoustical vibrations, normally recorded as magnetic fluctuations on tape. |
| Attack | The initial phase of a sound. |
| Attenuator | A device that controls the strength of some parameter of a signal. See also: Pot. |
| Audio-Frequency Range | The range of frequencies perceived by humans: roughly 16-20 Hz to 20 kHz. |
| Channel | One track of a tape recorder; one of 16 lines of MIDI data transmission. |
| Chorus Effect | Combining the main signal with several delayed versions to replicate a fuller sound. |
| Click Track | A click sound used as an aid in synchronizing material to be recorded. |
| Controller | A synthesizer module or function which controls some aspect of another module; a keyboard is a controller. |
| Daisy Chain | Connecting two or more synthesizers or devices, enabling one (the master) to control the others (slaves). |
| DAT Recorder | Digital Audio on Tape -- a recorder which encodes acoustic audio signals digitally. |
| Decay | A decrease in signal amplitude. |
| Decibel (dB) | A standard unit of loudness; 1/10th of a Bel. Normal conversation = 60 dB; threshold of pain = 130 dB. |
| Default | A standard or start-up condition. |
| Digital-to-Analog Converter (DAC) | Converts digital information into analog voltages. |
| Digital Delay | A device that creates effects by varying the number, amplitude, and timing of echoes. |
| Digital Recording | Using numbers (digital) to represent acoustical vibrations measured at equal intervals of time. |
| Direct Box | A device enabling, for example, an electric guitar to be connected directly to a mixing console by lowering impedance. |
| Disc | Storage medium for music or video information. |
| Disk | Computer storage medium, usually 3.5". |
| Editing | Altering previously-recorded data. |
| Effects | Echo, Chorus, Digital Delay, Reverb and other modifications produced by an effects processor. |
| Envelope | The amplitude history of a sonic event; a multi-segmented voltage curve. See also: ADSR. |
| Fader | A potentiometer (pot) or slider that varies the value of some parameter. |
| Frequency | The number of cycles per unit time of a periodic waveform, measured in Hertz (Hz). |
| Frequency Modulation (FM) | A variation in the frequency of one signal caused by another. FM produces many more sidebands than AM. |
| General MIDI | An addition to the MIDI specification ensuring sequences sound relatively the same across instruments. Includes 128 instrument sounds grouped into families. |
| Harmonic | A sine wave component of a complex waveform. Also called partial. |
| Hertz (Hz) | Unit of measurement of frequency equal to one cycle per second. |
| Impedance | Resistance to AC current, measured in ohms. |
| Intensity | The amount of power in a sound, directly related to amplitude. Measured in decibels. |
| Interface | A device or point where two or more instruments or procedures meet and communicate. |
| Loop | A method of recording or playback whereby specified measures repeat. |
| MIDI | Musical Instrument Digital Interface. |
| MIDI IN | Port which receives MIDI information from another source. |
| MIDI OUT | Port which transmits MIDI information to another source. |
| MIDI THRU | Port which retransmits received MIDI information to another source; used in daisy chaining. |
| Modulation Wheel / Joystick | A manual controller normally used to regulate vibrato amount. |
| Monophonic | One sound at a time. Monophonic synthesizers allow only one active key. |
| MP3 | MPEG-1 Audio Layer 3. A compression technique reducing sound files to about 1/10th original size by removing sounds beyond human hearing range. |
| Mute | A switch or control that turns off audio from a specified track. |
| Nyquist Theorem | The sampling rate must be at least twice the highest frequency to accurately represent sound without distortion. |
| Overdub / Sound on Sound | Adding material to a track after initial recording. |
| Overwrite | Erasing previously-recorded material and substituting new material. |
| Pan | The stereo field from left to right. |
| Parameter | A variable; some aspect or characteristic that may be used to control the flow of music. |
| Patch | The voice-name/location (bank, instrument number) of a particular synthesizer sound. |
| Phantom Power | Power from a mixer obtained through the cable connecting a direct box. |
| Pitch | A musical term for a note or tone. Pitch is what is perceived of the frequency. |
| Pitch Bend | Manual alteration of pitch through a pitch bend device (wheel or joystick). |
| Polyphonic | Many sounds. Keyboards controlling more than one signal path at a time. |
| Potentiometer (Pot) | A variable resistor used to control amplitude and other signal characteristics. See also: Attenuator. |
| PPQ / PPQN | Pulses Per Quarter Note. A division of timing resolution. |
| Quantization | Production of discrete values; in rhythm, error correction for attack/release points. |
| Sample | A numerical value representing a voltage. More samples per second = more accurate reproduction. |
| Sequencer | A device or software for recording and playback of MIDI data. |
| SMPTE Time Code | Standard code (Society of Motion Picture & Television Engineers) for synchronizing sight and sound. |
| Split Point | The point at which a keyboard is divided into different timbral regions. |
| Step Recording | Sequencing material one event at a time, with duration specified rather than performed. |
| Synthesizer | A set of electronic modules for generation and modification of sound. |
| Touch Sensitivity | Ability of a keyboard to produce/respond to velocity information based on key strike force. |
| Track | A channel on a tape recorder or sequencer; one of a set of concentric circles on a floppy disk. |
| Tweaking | Experimenting with a controller; adjusting parameters. |
| Wave Form | Describing an electrical signal by showing how amplitude varies over time. |
| Widget | A knob, slider, meter, etc. |

### Michael P. Bedesem MidiPlayer Installation

1. Uninstall any existing versions via Control Panel
2. Download the full package (v7.1.6), unzip, and run `Setup.exe` as Administrator — accept defaults and do not replace newer files with older ones
3. Download the update package (v8.3.1), unzip, and copy files to `C:\Program Files\MidiPlayer` — allow overwriting older files

### MIDI-Hits Download URL Template

```text
https://www.midi-hits.com/dl.php?OrderId=181368&catNo=<cat-number>
```

Replace `<cat-number>` with the catalog number from your order.

## Related

- [`FFmpeg Conversion And Frame Extraction`](/media/ffmpeg/conversion-and-frame-extraction/)
