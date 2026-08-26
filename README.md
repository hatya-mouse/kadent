# Kadent

Kadent is a DAW (Digital Audio Workstation) software. It supports building synthesizers and effects using KASL (Kadent Audio Shading Language).

# Features

- Build your own synthesizers and effects using KASL language!
- Live performance with MIDI controller
- Project save & load

# Building from Source

## Using Taskfile

### Debug Run

```bash
task dev
```

### Release

```bash
task build
```

### Package

```bash
task package -- <target>
```

## Without Taskfile

### Debug Run

```bash
cargo run
```

### Release

```bash
cargo build --release
```

# Profiling

```bash
cargo build --profile profiling
samply record ./target/profiling/kadent
```

# Directory Map

```
src/
├── background_thread
├── core
│   ├── audio_engine
│   │   ├── audio_data
│   │   ├── data_types
│   │   ├── graph
│   │   ├── mixer
│   │   ├── node
│   │   │   └── builtin
│   │   │       └── automation_node
│   │   ├── thread
│   │   ├── timing
│   │   └── track
│   │       ├── audio_track
│   │       │   └── track_impl
│   │       └── note_track
│   │           └── track_impl
│   ├── kasl_node
│   └── metadata
├── storage
│   ├── app_state
│   └── project
│       └── serial
│           ├── data
│           └── meta
└── ui
    ├── components
    ├── editor
    │   ├── actions
    │   │   ├── graph
    │   │   ├── note
    │   │   └── region
    │   ├── dialog
    │   ├── panel
    │   ├── state
    │   ├── status_bar
    │   ├── toolbar
    │   └── views
    │       ├── automation
    │       ├── code_editor
    │       ├── error_list
    │       ├── inspector
    │       ├── node_graph
    │       ├── piano_roll
    │       └── timeline
    │           └── edit_panel
    ├── splash
    └── theme

49 directories
```

## `core/`

This directory hosts core components for Kadent, including the audio engine and KASL node.

### `audio_engine/`

This directory contains the audio engine implementation.

### `kasl_node/`

This directory contains the KASL node implementation.

### `metadata/`

This directory contains structures used to store information that are not directly related to audio processing, e.g. track name, track color, etc.

## `storage/`

This directory contains the implementation of project file serialization and last opened project information.

### `app_state`

This directory contains the last opened project storage implementation.

### `project/`

This directory contains the project file serialization implementation.

## `ui/`

This directory contains the implementation of all the user interface.

### `components/`

This directory contains the implementation of user interface components that are used in multiple places.

### `editor/`

This directory contains the implementation of editor user interface.

### `splash/`

This directory contains the implementation of splash screen user interface.

### `theme/`

This directory defines the theme of the user interface, such as colors.
