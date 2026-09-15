# J2ME Emulation Reference Manual

This reference manual provides technical specifications of the J2ME platforms, class structures, and APIs that RessuRetro must emulate.

---

## 1. J2ME Platform Architecture

J2ME (Java 2 Micro Edition) is structured as a stack:
1. **Configuration**: Defines the JVM VM capabilities and core library APIs (`java.lang`, `java.io`, `java.util`).
2. **Profile**: Adds GUI, user input, persistence (RMS), and game APIs suitable for mobile phones.
3. **Optional Packages**: Vendor-specific additions (e.g. 3D graphics, media, file access).

### Configurations
- **CLDC 1.0 (JSR 30)**: Designed for devices with 160-512 KB of memory. Lacks floating-point numbers (`float`, `double`), reflection, and finalization. Use standard interpreter VM.
- **CLDC 1.1 (JSR 139)**: Adds floating-point support (`float`, `double`), weak references, and redefined thread priority rules. Lacks reflection.

### Profiles
- **MIDP 1.0 (JSR 37)**: Core J2ME profile. Basic HTTP network communication, simple LCD UI, and limited event loop capabilities.
- **MIDP 2.0 (JSR 118)**: Standard mobile game profile. Introduces:
  - `javax.microedition.lcdui.game` package (sprites, tiles, game loops).
  - Multi-tone/MIDI audio engines.
  - Custom RMS security rules.
  - Enhanced network configurations (sockets, secure connections).

---

## 2. JVM Class-File Layout & Bytecode

Java ME bytecode is compiled into standard `.class` files wrapped in a compressed `.jar` archive.

### Class-File Format
The binary file consists of a stream of 8-bit bytes. Multi-byte values are stored in **big-endian** format.

```text
ClassFile {
    u4             magic;             // Always 0xCAFEBABE
    u2             minor_version;
    u2             major_version;     // Usually 45.3 (Java 1.1) to 48.0 (Java 1.4) for CLDC
    u2             constant_pool_count;
    cp_info        constant_pool[constant_pool_count-1];
    u2             access_flags;
    u2             this_class;
    u2             super_class;
    u2             interfaces_count;
    u2             interfaces[interfaces_count];
    u2             fields_count;
    field_info     fields[fields_count];
    u2             methods_count;
    method_info    methods[methods_count];
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}
```

### Constant Pool Tags
The constant pool contains strings, numeric values, class names, method descriptors, and field references. Common tags include:
- `1`: UTF-8 string
- `3`: Integer
- `4`: Float
- `5`: Long (occupies 2 constant pool slots)
- `6`: Double (occupies 2 constant pool slots)
- `7`: Class reference (points to a UTF-8 name)
- `8`: String reference (points to a UTF-8 value)
- `9`: Field reference (contains class index and NameAndType index)
- `10`: Method reference (contains class index and NameAndType index)
- `12`: NameAndType descriptor (contains name index and descriptor index)

---

## 3. LCDUI and Game API Emulation

### Canvas vs. GameCanvas
J2ME games subclass either `Canvas` or `GameCanvas` to draw and poll input.

#### Canvas (`javax.microedition.lcdui.Canvas`)
- **Rendering**: Implements immediate-mode drawing. The system calls the overridden `paint(Graphics g)` method whenever the screen needs refresh. Thread-safe execution is forced.
- **Input**: Asynchronous event handlers like `keyPressed(int keyCode)` and `keyReleased(int keyCode)` are called by the system thread.

#### GameCanvas (`javax.microedition.lcdui.game.GameCanvas`)
- **Rendering**: Extends `Canvas` but implements double-buffering. It provides a graphics context via `getGraphics()` drawing to an off-screen buffer. The game loop calls `flushGraphics()` to push drawings to the screen.
- **Input**: Game loops poll input state synchronously by calling `getKeyStates()`. This returns a bitmask representing currently pressed keys (minimizes thread overhead).

### 2.5D Sprite and Tile Layering
MIDP 2.0 introduced the `javax.microedition.lcdui.game` package containing game abstractions:
- **`Sprite`**: Manages a rectangular visual frame, supports clipping, frame animations, and flips/rotations (`TRANS_NONE`, `TRANS_ROT90`, `TRANS_MIRROR`, etc.). Implements collision detection (bounding boxes or pixel-level collision).
- **`TiledLayer`**: Implements 2D grid cells referenced by cell index, mapping to tileset images. Allows efficient tile layout structures.
- **`LayerManager`**: Holds lists of layers (`Sprite`, `TiledLayer`), sets rendering viewports, and draws them in correct Z-order onto the graphics context.

---

## 4. Record Management System (RMS)

RMS represents J2ME's local persistent database:
- **RecordStore**: Named, isolated data tables. Names are case-sensitive and up to 32 characters long.
- **Record**: A record is identified by a unique ID (positive integer starting at 1). Records are simple raw byte arrays (`Vec<u8>`).
- **Functionality**:
  - `addRecord(byte[] data, int offset, int numBytes)`
  - `setRecord(int recordId, byte[] data, int offset, int numBytes)`
  - `getRecord(int recordId)`
  - `deleteRecord(int recordId)`
- **Isolated Storage**: Emulator must store each game's `RecordStore` inside an isolated folder matching the game identifier to satisfy the isolation requirement.

---

## 5. Major Vendor APIs

Many J2ME games require vendor-specific libraries for advanced audio, vibrations, lights, or 2D scaling features:

| Vendor | Package Prefix | Common Functionality |
| :--- | :--- | :--- |
| **Nokia** | `com.nokia.mid.ui` | `FullCanvas` (borders-free canvas), `DirectUtils` (direct pixel drawing / image scaling) |
| **Nokia** | `com.nokia.mid.sound` | `Sound` interface (playing multi-channel tones and vibrations) |
| **Sony Ericsson** | `com.sonyericsson.media` | Advanced audio codecs, camera controls |
| **Sony Ericsson** | `com.sonyericsson.vibration` | Explicit vibration controllers |
| **Samsung** | `com.samsung.util` | Vibration and light notifications |
| **Motorola** | `com.motorola.multimedia` | Advanced sound playing APIs |
