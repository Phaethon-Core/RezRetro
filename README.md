# RezRetro

> A high-performance, local-first, cross-platform Java ME (J2ME / CLDC / MIDP) mobile game emulator built with Rust and Flutter.

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Flutter](https://img.shields.io/badge/Flutter-3.x-02569B.svg)](https://flutter.dev/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20Android-green.svg)](#getting-started)

---

## Overview

**RezRetro** is a modern, standalone emulator engineered to preserve, execute, and enhance classic Java ME (J2ME / MIDP 2.0 / CLDC 1.1) mobile games on modern personal computers (Windows, Linux) and mobile devices (Android).

### Problem
During the 2000s and early 2010s, tens of thousands of video games and applications were created for Java-enabled feature phones (Nokia, Sony Ericsson, Samsung, Motorola). These titles were packaged as `.jar` / `.jad` archives designed around fixed LCD resolutions, 12-key numeric keypads, restricted hardware, and proprietary vendor APIs. Modern operating systems lack native runtimes for these applications, putting decades of mobile gaming history at risk of obsolescence.

### Solution & Vision
RezRetro avoids the overhead of emulating full mobile baseband hardware and proprietary operating system kernels. Instead, it implements a lean, sandboxed, high-performance software runtime in **Rust** coupled with a responsive, polished user interface in **Flutter**:
- Directly parses Java class bytecode and executes MIDlet lifecycles in an isolated virtual machine.
- Renders virtual framebuffers and software graphics primitives directly to native textures.
- Provides a clean, offline-first experience with custom keypads, per-game device profiles, and dynamic modding support.

### Target Audience
- **Retro Gaming & Preservation Enthusiasts**: Players seeking an accurate, frictionless way to revisit classic mobile titles on PC or modern smartphones.
- **Modders & Translators**: Creators looking to replace assets, translate strings, or upscale textures without altering original `.jar` binaries.
- **Researchers & Developers**: Anyone exploring clean-room JVM implementation, bytecode interpretation, and Rust/Flutter FFI interoperability.

---

## Features

### 🦀 Core Emulation Engine (Rust)
- **CLDC 1.0 / 1.1 & JVM Interpreter**: Custom stack-based bytecode interpreter implementing standard Java bytecode execution, method frames, operand stacks, and garbage collection abstractions.
- **MIDP 2.0 Graphics**: Native software rasterizer rendering to an ARGB8888 virtual framebuffer with integer scaling, clipping, font rasterization, and primitive drawing (`drawLine`, `drawRect`, `drawString`, `drawImage`).
- **Audio Subsystem**: Event-driven audio queue handling tone generation, frequency pitches, and MIDI playback mappings.
- **Record Management System (RMS)**: Persistent, isolated local storage emulating J2ME record stores per game without host filesystem pollution.
- **Vendor Compatibility Shims**: Built-in compatibility layers for popular vendor extensions (such as Nokia UI `com.nokia.mid.ui.DirectUtils`, Nokia Sound, and Samsung vibration utilities).

### 📱 Modern UI Shell (Flutter / Dart)
- **Local Game Library**: Scan, organize, and launch your `.jar` and `.jad` collection with automatic metadata extraction (game title, vendor, version, icon).
- **Virtual Keypad & Controls**: Configurable on-screen touch keypad layouts for mobile devices and seamless physical keyboard mapping for desktop.
- **Device Profiles**: Per-game configurations for LCD screen resolution (e.g., `128x128`, `176x208`, `240x320`), scaling algorithms, and frame-rate caps.
- **In-Memory Mod Engine**: Support for `.jar-mod.zip` packages to inject HD textures, audio replacements, and translations dynamically at runtime without modifying the original game archive.

### 🔒 Local-First & Private
- **100% Offline**: Operates completely offline with zero telemetry, zero analytics, and zero cloud accounts required.
- **Sandboxed Execution**: Guest game code and mods run strictly inside the isolated interpreter memory space with no direct access to host system calls.

---

## Screenshots

### Game Library & Launchpad
| Game Library Interface | In-Game Viewport & Virtual Controls |
| :---: | :---: |
| ![Library Interface](assets/screenshots/library_preview.png) | ![Gameplay Viewport](assets/screenshots/emulator_preview.png) |

### Device Profiles & Mod Management
| Device Profile Configuration | Mod Manager & Resource Packs |
| :---: | :---: |
| ![Settings Preview](assets/screenshots/settings_preview.png) | ![Mod Manager](assets/screenshots/mods_preview.png) |

---

## Tech Stack

| Category | Technology | Description |
| :--- | :--- | :--- |
| **Core Engine** | [Rust](https://www.rust-lang.org/) (2021 Edition) | High-performance, memory-safe JVM bytecode interpreter & J2ME runtime |
| **Frontend UI** | [Flutter](https://flutter.dev/) / [Dart](https://dart.dev/) | Cross-platform UI shell for Desktop (Windows, Linux) and Mobile (Android) |
| **FFI Bridge** | [flutter_rust_bridge](https://cjycode.com/flutter_rust_bridge/) (v2.0) | Zero-overhead, type-safe asynchronous Foreign Function Interface bridge |
| **Container & Zip** | `zip` & `byteorder` Crates | Efficient manifest parsing, resource extraction, and endian-safe binary decoding |
| **Serialization** | `serde` & `serde_json` | Configuration serialization, device profiles, and mod manifests |
| **Build Tools** | Cargo, Flutter CLI, `cargo-ndk` | Multi-target compilation pipelines for desktop and mobile architectures |

---

## Architecture

RezRetro is organized into a modular, decoupled architecture where the Rust core acts as an independent headless engine and Flutter handles display presentation and user input.

```text
┌──────────────────────────────────────────────────────────────────┐
│                   Flutter Frontend (UI Shell)                    │
│  ┌──────────────────┐  ┌──────────────────┐  ┌────────────────┐  │
│  │   Game Library   │  │  Virtual Keypad  │  │  Mod Manager   │  │
│  └────────┬─────────┘  └────────┬─────────┘  └────────┬───────┘  │
└───────────┼─────────────────────┼─────────────────────┼──────────┘
            │                     │                     │
            ▼                     ▼                     ▼
┌──────────────────────────────────────────────────────────────────┐
│             Flutter Rust Bridge (FFI C-Compatible ABI)           │
└─────────────────────────────────┬────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────┐
│                   Rust Core Engine (ressu_core)                  │
│  ┌──────────────────┐  ┌──────────────────┐  ┌────────────────┐  │
│  │  JAR & Manifest  │  │  JVM Interpreter │  │ MIDlet Runtime │  │
│  │     Parser       │  │ (Bytecode Engine)│  │   Lifecycle    │  │
│  └────────┬─────────┘  └────────┬─────────┘  └────────┬───────┘  │
│           │                     │                     │          │
│  ┌────────┴─────────┐  ┌────────┴─────────┐  ┌────────┴───────┐  │
│  │ ARGB Framebuffer │  │   Audio Queue    │  │  RMS Database  │  │
│  │   & 2D Graphics  │  │  (Tone & MIDI)   │  │    Storage     │  │
│  └──────────────────┘  └──────────────────┘  └────────────────┘  │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │         Modding Subsystem & Device Profile Manager         │  │
│  └────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────┬────────────────────────────────┘
                                  │
                                  ▼
┌──────────────────────────────────────────────────────────────────┐
│                      Host Local Filesystem                       │
│      data/games/   │   data/saves/   │   data/mods/              │
└──────────────────────────────────────────────────────────────────┘
```

---

## Project Structure

```text
RezRetro/
├── Cargo.toml                  # Cargo Workspace configuration
├── LICENSE                     # Apache 2.0 License file
├── NOTICE                      # Copyright and attribution notice
├── flutter_rust_bridge.yaml    # Code generation settings for FFI bridge
├── core/                       # Core Rust Emulation Engine
│   ├── Cargo.toml              # Rust crate manifest
│   ├── src/
│   │   ├── api/                # High-level emulator API exposed to FFI
│   │   ├── audio/              # Audio queue, tone, and MIDI abstractions
│   │   ├── classfile/          # Java .class binary parser & constant pool
│   │   ├── config/             # Device profile definitions and screen dimensions
│   │   ├── ffi.rs              # C-compatible FFI entry points
│   │   ├── graphics/           # Virtual framebuffer & software drawing engine
│   │   ├── input/              # Bitfield key state tracking & touch coordinates
│   │   ├── jar/                # ZIP extraction, resource loading & manifest reader
│   │   ├── mods/               # In-memory resource override engine (.jar-mod.zip)
│   │   ├── runtime/            # MIDlet state container and event loops
│   │   ├── storage/            # RMS record store persistent serialization
│   │   └── vm/                 # JVM bytecode interpreter, opcodes & stack frames
│   └── tests/                  # Core integration and compatibility tests
├── app/                        # Flutter Application Frontend
│   ├── pubspec.yaml            # Dart package configuration & dependencies
│   ├── lib/
│   │   ├── core/               # Routing, constants, and themes
│   │   ├── features/
│   │   │   ├── emulator/       # Game viewport screen and virtual keypad
│   │   │   ├── library/        # Local game library grid & metadata views
│   │   │   ├── mods/           # Mod management and installation screens
│   │   │   └── settings/       # Device profile and resolution selector
│   │   └── src/rust/           # Auto-generated Flutter-Rust-Bridge bindings
│   ├── android/                # Android platform runner and Gradle configuration
│   ├── linux/                  # Linux platform desktop runner (CMake)
│   └── windows/                # Windows platform desktop runner (C++ Win32)
└── data/                       # Local runtime directory structure
    ├── config/                 # User preferences and per-game JSON profiles
    └── mods/                   # Installed .jar-mod.zip resource packs
```

---

## Getting Started

### Prerequisites

Before compiling RezRetro, ensure you have the following installed on your development workstation:

1. **Rust Toolchain**: Stable version `1.75+` (with `cargo`).
   ```bash
   # Verify Rust installation
   rustc --version
   cargo --version
   ```
2. **Flutter SDK**: Version `3.12+` (Channel `stable`).
   ```bash
   # Verify Flutter installation
   flutter doctor
   ```
3. **Flutter Rust Bridge Code Generator**:
   ```bash
   cargo install flutter_rust_bridge_codegen --version 2.0.0
   ```
4. **Platform-Specific Build Tools**:
   - **Windows**: Visual Studio 2022 with "Desktop development with C++" workload.
   - **Linux**: `clang`, `cmake`, `ninja-build`, `pkg-config`, `libgtk-3-dev`.
   - **Android**: Android Studio, Android SDK (`API 33+`), and `cargo-ndk` for cross-compilation.

---

### Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/Phaethon-Core/RezRetro.git
   cd RezRetro
   ```

2. **Fetch Flutter Dependencies**:
   ```bash
   cd app
   flutter pub get
   cd ..
   ```

3. **Generate FFI Bindings** *(if updating FFI methods)*:
   ```bash
   flutter_rust_bridge_codegen generate
   ```

---

### Building & Running the Project

#### 🪟 Windows Desktop
1. Build the Rust dynamic library:
   ```powershell
   cargo build --package ressu_core
   ```
2. Run the Flutter Windows application:
   ```powershell
   cd app
   flutter run -d windows
   ```

#### 🐧 Linux Desktop
1. Build the Rust shared library:
   ```bash
   cargo build --package ressu_core
   ```
2. Run the Flutter Linux application:
   ```bash
   cd app
   flutter run -d linux
   ```

#### 🤖 Android (Device or Emulator)
1. Build native libraries using `cargo-ndk`:
   ```bash
   # For physical 64-bit ARM devices
   cargo ndk -t arm64-v8a -o app/android/app/src/main/jniLibs/ build --package ressu_core --lib

   # For x86_64 Android Emulator
   cargo ndk -t x86_64 -o app/android/app/src/main/jniLibs/ build --package ressu_core --lib
   ```
2. Run the Flutter Android application:
   ```bash
   cd app
   flutter run -d <device-id>
   ```

---

## Configuration

RezRetro uses a modular directory layout for game files, saves, configurations, and mods.

### Directory Layout
```text
data/
├── games/          # Put your J2ME games here (*.jar, *.jad)
├── saves/          # Persistent RMS storage databases (*.rms)
├── mods/           # Drop your .jar-mod.zip packages here
└── config/         # Device profiles and application settings
```

### Device Profiles (`config/games/<game_id>.json`)
You can fine-tune display dimensions, framerate caps, and vendor quirks per game:

```json
{
  "profile_name": "Nokia N95 Standard",
  "screen_width": 240,
  "screen_height": 320,
  "fps_target": 60,
  "vendor_quirks": {
    "nokia_direct_graphics": true,
    "ignore_missing_sound_banks": true
  },
  "scaling_mode": "integer_fit"
}
```

---

## Usage

### 1. Adding Games
- Copy your `.jar` and `.jad` files into the `data/games/` folder, or click the **"Import Game"** button inside the library view to select a file from your disk.
- The emulator will automatically parse the `META-INF/MANIFEST.MF` to extract the title, vendor, version, and MIDlet execution entry point.

### 2. Controls & Navigation
On Desktop, the standard J2ME keypad maps to the following physical keys by default:

| J2ME Key | Keyboard Key | Description |
| :--- | :--- | :--- |
| **Softkey 1** (Left) | `Q` / `F1` | Left Context Action / Menu |
| **Softkey 2** (Right) | `E` / `F2` | Right Context Action / Back |
| **D-Pad / Joystick** | `Arrow Keys` or `W`, `A`, `S`, `D` | Up, Down, Left, Right Movement |
| **Select / Fire** | `Enter` / `Space` / `Key 5` | Action / OK / Fire |
| **Numeric 0–9** | `0`–`9` (Number Row or Numpad) | Number Input / Game Shortcuts |
| **Star / Hash** | `*` / `#` | Special Game Controls |

*On Android, a fully customizable, haptic-enabled virtual keypad overlay is displayed automatically.*

### 3. Installing Mods (`.jar-mod.zip`)
1. Place the mod package into `data/mods/`.
2. Open the **Mod Manager** tab in RezRetro.
3. Toggle the mod to **Enabled**. The engine will intercept asset queries and swap in high-resolution sprites, updated audio, or translations on the fly.

---

## Known Issues & Limitations

- **M3G (JSR 184) 3D Graphics**: 3D graphics extensions are currently experimental; complex 3D titles may experience rendering anomalies or fallback to software emulation.
- **Network / Sockets (GCF)**: Socket and HTTP connections return simulated offline responses; multiplayer internet features are not yet connected to live relays.
- **Audio Soundfonts**: Audio playback relies on platform MIDI synthesis or built-in square/sine wave tone generators; sound quality may vary across operating systems.

---

## Contributing

Contributions are welcome! If you would like to help improve RezRetro:

1. **Fork the Repository** on GitHub.
2. **Create a Feature Branch**:
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Ensure License Compliance**:
   Every new source file (`.rs`, `.dart`, etc.) must include the SPDX license header:
   ```text
   // SPDX-License-Identifier: Apache-2.0
   // Copyright 2026 Phaethon
   ```
4. **Commit Your Changes**:
   ```bash
   git commit -m "feat: Add support for custom color palette shaders"
   ```
5. **Push to Your Branch**:
   ```bash
   git push origin feature/amazing-feature
   ```
6. **Open a Pull Request** with a detailed explanation of your changes.

---

## License

This project is licensed under the **Apache License 2.0**.

For full details, see the [LICENSE](LICENSE) and [NOTICE](NOTICE) files.

---

## Acknowledgements

- The global **J2ME preservation community** for reverse-engineering and archiving classic mobile phone software.
- The **[flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge)** developers for enabling seamless, safe FFI bindings between Rust and Dart.
- The **Rust** and **Flutter** developer ecosystems for providing the robust foundation required for modern cross-platform engineering.

---

## Disclaimer

**RezRetro** is an independent, non-commercial open-source preservation project. 

- Java, Java ME, and J2ME are trademarks or registered trademarks of Oracle and/or its affiliates.
- Nokia, Sony Ericsson, Samsung, Gameloft, and all other mobile phone brands or game titles mentioned are trademarks or registered trademarks of their respective owners.
- RezRetro does **not** include, bundle, or distribute any copyrighted game ROMs, proprietary firmware, or commercial `.jar` files. Users must provide their own legally acquired game backups.

---

## Author

**Phaethon**

- **GitHub**: [Phaethon-Core](https://github.com/Phaethon-Core)
- **Email**: [phaethon.dev@gmail.com](mailto:phaethon.dev@gmail.com)
