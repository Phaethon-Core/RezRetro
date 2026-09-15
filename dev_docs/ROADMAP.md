# Development Roadmap

This roadmap outlines the development lifecycle of RessuRetro from initial scaffolding to a stable production-ready emulator.

---

## Phase 1: Foundation & Scaffolding (Current)
*Goal: Establish the basic directory structures, configuration, compliance headers, and documentation.*
- [x] Create root project configurations: `LICENSE`, `NOTICE`, `.gitignore`.
- [x] Create the local directory structure for games, config, saves, and mods.
- [x] Scaffolding Rust core workspaces: create `core/Cargo.toml` and configure submodules.
- [x] Scaffolding Flutter application: run `flutter create` and configure Dart dependencies.
- [x] Write core developer documentation (`dev_docs/`).

---

## Phase 2: JVM Core & Class Loader
*Goal: Interpret Java binary classes and execute basic bytecode instructions.*
- [x] Implement a full binary class reader (`core/src/classfile/`).
- [x] Implement complete Constant Pool resolution logic.
- [x] Implement the execution loop for JVM instructions (`core/src/vm/`).
- [x] Build operand stack manipulation and local variable state systems.
- [x] Add unit tests verifying calculation bytecodes (`iadd`, `isub`, `imul`, etc.) and variable stack frames.

---

## Phase 3: J2ME Core API Integration
*Goal: Run J2ME application loops and render graphics.*
- [x] Parse JAR manifests and resolve MIDlet lifecycle states.
- [x] Implement `javax.microedition.midlet.MIDlet` base classes.
- [x] Build `javax.microedition.lcdui.Graphics` 2D primitive rasterization logic in Rust.
- [x] Support double-buffered drawing context of `javax.microedition.lcdui.game.GameCanvas`.
- [x] Implement RMS database storage: reading and writing `.rms` records.

---

## Phase 4: Flutter Shell & UI Integration
*Goal: Create a premium frontend, load games, and handle user inputs.*
- [x] Implement dynamic library loading and bindings resolution in Dart.
- [x] Create a library screen listing local games with icons, titles, and vendors.
- [x] Bind keyboard inputs on desktop and map them to J2ME key codes.
- [x] Render the J2ME virtual framebuffer onto a Flutter native texture without buffer copies.
- [x] Design and implement virtual touch controls overlay on Android.

---

## Phase 5: Mod System & Sound Playback
*Goal: Add customizable mods and audio playbacks.*
- [x] Implement `.jar-mod.zip` resource interception.
- [x] Build the in-memory overrides lookup system for asset modifications.
- [x] Add mod manager interface in Flutter (Enable, Disable, Reorder).
- [x] Implement tone and MIDI event queuing inside the Rust audio subsystem.
- [x] Hook host audio engines (oboe on Android, rodio on desktop) to play J2ME sounds.

---

## Phase 6: Polish & Game Compatibility
*Goal: Achieve high compatibility with top mobile games and optimize performance.*
- [x] Establish automated testing framework scanning validation games.
- [x] Implement vendor-specific class extensions (Nokia, Samsung, Motorola).
- [x] Profile bytecode execution loops to reduce overhead and CPU load.
- [x] Save/Load game states (save states serialization).
- [x] Package build pipelines to compile stable binaries for Windows, Linux, and Android.
