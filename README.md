<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/hatya-mouse/kadent/main/assets/logo/kadent_logo_white.png">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/hatya-mouse/kadent/main/assets/logo/kadent_logo_black.png">
  <img src="https://raw.githubusercontent.com/hatya-mouse/kadent/main/assets/logo/kadent_logo_white_on_black.png" width="320px" alt="Description">
</picture>

**DAW with a KASL language to create custom synths & effects**

---

</div>

Kadent is a DAW (Digital Audio Workstation) software. It supports building synthesizers and effects using built-in KASL language.

## Features

- Build your own synthesizers and effects using KASL language!
- Live performance with MIDI controller
- Project save & load

## Installation

### Download Prebuilt Binary

#### macOS

Download the installer from [Releases](https://github.com/hatya-mouse/kadent/releases) and open it to install.

#### Windows

Download the ZIP archive of the prebuilt binary from [Releases](https://github.com/hatya-mouse/kadent/releases), extract the archive, and move it to your desired location.

#### Linux

Download the .tar.gz archive of the prebuilt binary from [Releases](https://github.com/hatya-mouse/kadent/releases), extract the archive, and move it to your desired location.

### Building from Source

#### Using Taskfile

##### Build Bundle

```bash
task bundle-macos
task bundle-windows
task bundle-linux
```

##### Build Installer or Compressed Archive

You don't need to build bundle before building an installer or creating an archive.

```bash
task installer-macos
task compressed-windows
task compressed-linux
```

#### Without Taskfile

```bash
cargo build --release
```

## Profiling

```bash
cargo build --profile profiling
samply record ./target/profiling/kadent
```
