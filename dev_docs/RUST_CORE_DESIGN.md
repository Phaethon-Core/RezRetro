# Rust Emulator Core Design

This document details the software design of the platform-independent Rust core (`core/`) engine.

---

## 1. Modular Subsystem Design

```text
ressu_core/
├── classfile/      - Binary Class parsing and Constant Pool resolving
├── vm/             - JVM stack frame, operand stack, and opcode execution
├── jar/            - ZIP archive loader, file reading, and manifest parser
├── runtime/        - J2ME MIDlet state container and lifecycle handlers
├── graphics/       - ARGB Virtual framebuffer and primitive drawing tools
├── audio/          - MIDI and frequency tone queue buffer
├── input/          - 32-bit bitfield of active keys (key pressed flags)
├── storage/        - RecordStore database serialized to disk (.rms)
├── mods/           - Map of asset replacements parsed from zip files
├── config/         - Target device profiles (dimensions, vendor quirks)
└── ffi.rs          - C FFI entry points called by the Dart host app
```

---

## 2. JVM Bytecode Execution Model

The JVM emulation acts as a stack machine. It processes one method bytecode stream at a time using stack frames.

### Virtual Machine State (`vm::Vm`)
- **`classes`**: A hashmap storing loaded `ClassFile` models keyed by class name (e.g. `com/phaethon/Game`).
- **`heap`**: Vector of JVM objects allocation. Objects index this vector to prevent Rust borrow-checker conflicts.

### Stack Frame (`vm::StackFrame`)
Each method invocation allocates a stack frame containing:
- **`local_variables`**: An indexed array storing local references, ints, or floats.
- **`operand_stack`**: The push/pop stack workspace for math operations, loading variables, and prepping arguments.
- **`program_counter`**: Index offset inside the active method bytecode.
- **`bytecode`**: Reference or copy of the method bytecode array.

### Execution Loop (`vm::Vm::execute`)
The interpreter loops over the bytecode array:
1. Fetch opcode byte at `program_counter`.
2. Decode to `Opcode` enum.
3. Advance `program_counter` based on instructions and arguments.
4. Execute operations (modify stack, heap, local variables).
5. If method returns, pop the StackFrame and resume the previous frame.

---

## 3. Graphics & Virtual Framebuffer

J2ME games draw onto a virtual screen of fixed LCD dimensions.

### Virtual Framebuffer (`graphics::Framebuffer`)
- Holds a raw 1D pixel buffer represented as: `Vec<u32>` of size `width * height`.
- Pixels are stored in **ARGB8888** layout (32 bits: Alpha, Red, Green, Blue).
- Provides functions for bounds-safe pixel manipulation and clipping.

### Drawing Semantics
- Primitive operations (`drawLine`, `drawRect`, `fillRect`, `drawString`) are implemented in Rust.
- They manipulate the virtual framebuffer directly.
- The `ffi::get_framebuffer_pixels` function returns a raw pointer to this buffer, letting Flutter read and paint the screen directly.

---

## 4. Audio Engine

The J2ME guest executes on a separate thread than the host audio device. We decouple audio generation using an event queue.

### Audio Queue (`audio::AudioSystem`)
- Holds an event queue (`Vec<AudioEvent>`).
- When the game calls `playTone(freq, dur)` or starts playing MIDI, Rust packages an `AudioEvent` with frequency, duration, or raw MIDI bytes.
- The host shell polls or is notified of new audio events and feeds the data to its native sound APIs.

---

## 5. Storage (RMS)

RMS data is written to the host filesystem inside the `data/saves/` directory.

### Storage Layout
- Storage files are isolated in folders named after the game identifier:
  `data/saves/<game_id>/<record_store_name>.rms`
- File serialization starts with record count, followed by a series of blocks:
  `[u8 active_flag] [u32 byte_length] [bytes data]`
- Active records map to their index location in J2ME. Deleted records have `active_flag = 0` to preserve the ID indexing structure.

---

## 6. Mod Override Engine

The mod system overrides files inside the JAR archive dynamically without modifying the JAR.

### Interception Path
1. When the J2ME runtime requests a class asset (e.g. `assets/logo.png`):
   ```text
   Resource Request -> Check ModSystem Overrides -> Found? Return Mod bytes
                                                 -> Not Found? Read from JAR
   ```
2. The `ModSystem` loads and reads enabled `.jar-mod.zip` files on startup, parsing their `manifest.json` configurations.
3. Assets found inside `assets/` in the mod ZIP file are loaded into the Rust `overrides` hashmap.
4. When resource loading is triggered in the VM, it pulls the bytes from `overrides` first, and falls back to the J2ME JAR container only on cache miss.
5. This ensures total isolation and preservation of the original game files.
