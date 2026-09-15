# Project Architecture

RessuRetro is built using a decoupled architecture, separating the platform-specific UI shell from the platform-independent emulator engine.

---

## High-Level Architecture Diagram

```mermaid
graph TD
    %% Frontend Layer
    subgraph Flutter Frontend [Flutter Application Shell]
        UI[Library UI & File Picker]
        KLayout[Virtual Keypad Configurator]
        ModM[Mod Installer / Manager]
        Settings[Global & Per-Game Config]
    end

    %% FFI Bridge
    subgraph FFI [Flutter Rust Bridge Boundary]
        FRB[flutter_rust_bridge Code Generator]
        CABI[C-Compatible ABI Autogen]
        FRB <=> CABI
    end

    %% Rust Core Engine
    subgraph Rust Core [Ressu Emulator Engine]
        direction TB
        Runtime[J2ME Runtime & MIDlet Lifecycle]
        VM[JVM Bytecode interpreter]
        ClassParser[Java Class-File Parser]
        JAR[JAR Manifest & Resource Loader]
        Graphics[Virtual Framebuffer & 2D Graphics]
        Audio[Audio Queue: MIDI & Tones]
        Input[Keypad & Pointer Handler]
        Storage[Record Management System - RMS]
        Mods[Resource Override System]
        Config[Device Profiles & Key Mappings]
        
        Runtime --> VM
        VM --> ClassParser
        Runtime --> JAR
        VM --> Graphics
        VM --> Audio
        VM --> Input
        VM --> Storage
        Runtime --> Mods
        Runtime --> Config
    end

    %% File System
    subgraph StorageLayer [Local Host Filesystem]
        GFolder[data/games/]
        SFolder[data/saves/]
        MFolder[data/mods/]
        CFile[data/config/config.json]
    end

    %% Connections
    UI -->|Loads| FRB
    FRB -->|C FFI Calls| CABI
    CABI -->|Invokes API| Runtime
    
    JAR -.->|Read Game JARs| GFolder
    Storage -.->|Read/Write Save files| SFolder
    Mods -.->|Load ZIP Overrides| MFolder
    Config -.->|Load Settings| CFile
```

---

## 1. Architectural Layers

### A. Flutter Frontend Shell (`app/`)
Written in Dart/Flutter. This layer runs on the main host thread and manages the application's visual shell.
- **Library Management**: Scan the local games directory, read parsed metadata returned from Rust, and display games with titles, version details, and vendor descriptions.
- **Settings Screen**: Set audio volume, choose device emulation profile (e.g. Nokia 240x320, SE 176x220), and view keyboard shortcut bindings.
- **Virtual Keypad**: Render configurable overlay buttons on Android (Directional controls, keypad numbers, soft keys).
- **Mod Manager**: Provide drag-and-drop or file pickers for mod installations.
- **Host Canvas Integration**: Receive the pointer to the Rust framebuffer pixels and paint them onto a Flutter native textures layer or custom painter, maintaining aspect ratios.

### B. FRB Boundary (`core/src/api/`)
Exposes Rust functions to Dart automatically via `flutter_rust_bridge`.
- Simplifies type mappings, converting standard Rust types (e.g. `String`, `Vec<u32>`, `Result`) directly to Dart futures and lists.
- Avoids manual memory alloc and free boilerplate.
- The compiled engine is loaded automatically during startup by calling `RustLib.init()`.

### C. Rust Core Engine (`core/src/`)
Written in Rust. Handles complete execution of the J2ME guest container.
- **JAR Parser (`jar/`)**: Extracts classes and assets out of `.jar` files. Reads the manifest to locate entry points.
- **Class-File Parser (`classfile/`)**: Reads binary class structures, parses the constant pool, parses class methods and bytecodes.
- **Bytecode Interpreter (`vm/`)**: Stack-based execution loop that decodes instructions, manipulates the local stack/variables, resolves method dispatches, and invokes native overrides.
- **Runtime Lifecycle (`runtime/`)**: Maps the standard Java ME `MIDlet` states (`startApp`, `pauseApp`, `destroyApp`) to external host controls.
- **Graphics Subsystem (`graphics/`)**: Draws lines, rects, text, and images onto a J2ME virtual framebuffer, separating guest draw logic from host GPU rendering.
- **Audio Subsystem (`audio/`)**: Serializes sound events (tone frequencies, MIDI sequences) and queues them for Flutter or a host library to play.
- **Input Subsystem (`input/`)**: Manages guest key configurations and polls state bitfields (directional keys, keypad inputs, soft keys).
- **Storage Subsystem (`storage/`)**: Isolation-compliant Record Management System (RMS) that saves game records locally.
- **Mod System (`mods/`)**: Intercepts J2ME class resource requests and returns overrides from loaded `.jar-mod.zip` files if they exist.

---

## 2. Core Execution Flow

```text
1. Import J2ME Game
   Flutter Pick File -> Call FFI load_jar(path) -> Rust JAR Parser loads ZIP -> Rust extracts manifest

2. Start Emulation
   Flutter Click Play -> Call FFI start_midlet() -> J2ME Runtime transitions to Active
   -> Rust initializes StackFrame for main class -> VM Execution Loop starts

3. Game Rendering Loop
   VM interprets bytecode -> invokevirtual to Graphic primitives -> Rust draws on Virtual Framebuffer
   -> Flutter polls get_framebuffer_pixels() -> Flutter redraws texture

4. User Action Input
   Flutter Key Press -> Call FFI send_key_event() -> Rust Input updates active KeyStates
   -> VM J2ME GameCanvas query polls KeyStates or event listener fires

5. Exit Game
   Flutter Click Stop -> Call FFI stop_midlet() -> J2ME transitions to Destroyed
   -> Rust Storage saves pending RMS RecordStore to disk -> FFI destroy_emulator() frees Rust heap
```
