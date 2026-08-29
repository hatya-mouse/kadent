# Kadent

Kadent is a DAW (Digital Audio Workstation) software. It supports building synthesizers and effects using KASL (Kadent Audio Shading Language).

# Features

- Build your own synthesizers and effects using KASL language!
- Live performance with MIDI controller
- Project save & load

# Installation

## Download Prebuilt Binary

## macOS

Download the installer from [Releases](https://github.com/hatya-mouse/kadent/releases) and open it to install.

## Windows

Download the ZIP archive of the prebuilt binary from [Releases](https://github.com/hatya-mouse/kadent/releases), extract the archive, and move it to your desired location.

## Linux

Download the .tar.gz archive of the prebuilt binary from [Releases](https://github.com/hatya-mouse/kadent/releases), extract the archive, and move it to your desired location.

## Building from Source

### Using Taskfile

#### Debug Run

```bash
task dev
```

#### Release

```bash
task build
```

#### Bundle

```bash
task bundle-macos
task bundle-windows
task bundle-linux
```

#### Build Installer or Compressed Archive

```bash
task installer-macos
task compressed-windows
task compressed-linux
```

### Without Taskfile

#### Debug Run

```bash
cargo run
```

#### Release

```bash
cargo build --release
```

# Profiling

```bash
cargo build --profile profiling
samply record ./target/profiling/kadent
```
