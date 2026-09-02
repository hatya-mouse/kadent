<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/hatya-mouse/kadent/main/assets/logo/kadent_logo_white.png">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/hatya-mouse/kadent/main/assets/logo/kadent_logo_black.png">
  <img src="https://raw.githubusercontent.com/hatya-mouse/kadent/main/assets/logo/kadent_logo_white_on_black.png" width="320px" alt="Description">
</picture>

**DAW with a KASL language to create custom synths & effects**

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/hatya-mouse/kadent/release.yml?style=for-the-badge)
![GitHub Release](https://img.shields.io/github/v/release/hatya-mouse/kadent?style=for-the-badge)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/hatya-mouse/kadent/total?style=for-the-badge)
![GitHub License](https://img.shields.io/github/license/hatya-mouse/kadent?style=for-the-badge)
![Deps.rs Repository Dependencies](https://img.shields.io/deps-rs/repo/github/hatya-mouse/kadent?style=for-the-badge)

---

</div>

Kadent is a DAW (Digital Audio Workstation) software. It supports building synthesizers and effects using built-in KASL language.

## Features

- Build your own synthesizers and effects using KASL language!
- Live performance with MIDI controller
- Project save & load
- Node graph attached to every track

## KASL Language

KASL is a domain-specific language designed for audio processing. It uses Cranelift as a backend for faster JIT (Just-in-time) compilation.

### Input, Output, and State

KASL has three unique types of variables — `input`, `output` and `state`.

`input` variable is a variable whose value is passed from the DAW that executes the program.
`output` variable is a variable to pass the value to the DAW.
`state` variable is a variable that is preserved over samples.

### Limitations

KASL is designed for audio processing where the program needs to finish processing in a short period.

Dynamic-size array (resizable array) needs reallocation when an element is added to it. The DAW cannot predict how long it will take to reallocate the memory, potentially leading to audio glitches and dropouts.

Conditional loop and recursion have similar issue. Conditional loop is a loop that the count of it is not known at the time of compilation. This means that program may loop for so many times that it blocks the audio thread and causes audio glitches. Recursion have the same problem.

That's why I did not add dynamic-size array and conditional loop to it, and prohibited recursion.

### Operator Definition

In KASL, all operators are written in KASL language, not in the compiler.

For example, here is the definition of arithmetic operators in `std` library of KASL (The full source code can be found in [hatya-mouse/kasl-std](https;//github.com/hatya-mouse/kasl-std))

This means that you cannot even use basic arithmetic operators without importing `std` library.

```kasl
operator infix + {
    precedence: 90,
    associativity: left
}

operator infix - {
    precedence: 90,
    associativity: left
}

operator infix * {
    precedence: 100,
    associativity: left
}

operator infix / {
    precedence: 100,
    associativity: left
}
```

```kasl
func infix +(lhs: Int, rhs: Int) -> Int {
    return Builtin.iadd(lhs, rhs)
}

func infix -(lhs: Int, rhs: Int) -> Int {
    return Builtin.isub(lhs, rhs)
}

func infix *(lhs: Int, rhs: Int) -> Int {
    return Builtin.imul(lhs, rhs)
}

func infix /(lhs: Int, rhs: Int) -> Int {
    return Builtin.idiv(lhs, rhs)
}
```

## KASL Sample Program
> `audio` module is built-in module provided by Kadent. Note that these programs below will not work outside the DAW.
### Gain Node
```kasl
import audio
import std

input in = audio.zero_sample()
input gain = 0.0
output out = audio.zero_sample()

func main() {
    out = in * [gain; audio.max_channels]
}
```
### Mix Node
```kasl
import std
import std/math/float as f
import audio

input input_1 = audio.zero_sample()
input input_2 = audio.zero_sample()
input factor = 0.0
output out = audio.zero_sample()

func main() {
    let clamped_factor = f.clamp(factor, 0.0, 1.0)
    let multiplied_input_1: audio.Sample = input_1 * [clamped_factor; audio.max_channels]
    let multiplied_input_2: audio.Sample = input_2 * [1.0 - clamped_factor; audio.max_channels]
    out = multiplied_input_1 + multiplied_input_2
}
```
### Sawtooth Wave Synthesizer
```kasl
import audio as a
import std
import std/math/float as f

input notes = a.EventSlot()
input pitch = 1.0
output out = a.zero_sample()
state allocator = a.Allocator()

let base_freq = 440.0
let volume = 0.1

func main() {
    allocator.update(notes)

    var i = 0
    loop a.max_voices {
        let voice = allocator.voices[i]
        let freq = pitch * base_freq * f.pow(2.0, (voice.pitch - 69.0) / 12.0)

        if voice.is_active {
            let t = voice.age

            let sample = saw(freq: freq, time: t)
            out = out + [sample * volume; a.max_channels]
        }
        i = i + 1
    }
}

func saw(freq f = 440.0, time t: Float) -> Float {
    return 2.0 * (t * f) % 1.0 - 1.0
}
```

## Screenshots

![Screenshot 1](https://raw.githubusercontent.com/hatya-mouse/kadent/main/assets/readme/screenshot_1.png)
*Code editor with some errors shown*

![Screenshot 2](https://raw.githubusercontent.com/hatya-mouse/kadent/main/assets/readme/screenshot_2.png)
*During playback*

![MIDI Demo](https://raw.githubusercontent.com/hatya-mouse/kadent/main/assets/readme/midi_demo_1.png)
*Connect a MIDI controller to Kadent and play the synthesizer*

## Installation

### Download Prebuilt Binary

#### macOS

Download the installer from [Releases](https://github.com/hatya-mouse/kadent/releases) and open it to install.

#### Windows

Download the ZIP archive of the prebuilt binary from [Releases](https://github.com/hatya-mouse/kadent/releases), extract the archive, and move it to your desired location.

#### Linux

Download the .tar.gz archive of the prebuilt binary from [Releases](https://github.com/hatya-mouse/kadent/releases), extract the archive, and move it to your desired location.

### Building from Source (macOS / Linux)

#### Build Bundle

```bash
task bundle-macos
task bundle-windows
task bundle-linux
```

#### Build Installer or Compressed Archive

You don't need to build bundle before building an installer or creating an archive.

```bash
task installer-macos
task compressed-windows
task compressed-linux
```

### Without Taskfile

```bash
cargo build --release
```

## Profiling

```bash
cargo build --profile profiling
samply record ./target/profiling/kadent
```
