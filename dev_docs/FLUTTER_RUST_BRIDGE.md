# Flutter ↔ Rust Bridge (FRB) Guide

This document describes how Flutter communicates with the Rust core library using `flutter_rust_bridge` (v2), detailing types resolution, configuration, and codegen flows.

---

## 1. Bridge Architecture Strategy

RessuRetro uses `flutter_rust_bridge` to automate dynamic linking and type conversions between Dart and Rust.

```text
  Flutter Dart Runtime                   FRB Codegen Layer               Rust Core Library
┌──────────────────────┐              ┌───────────────────────┐        ┌──────────────────┐
│  src/rust/api/       │ ────────────>│  frb_generated.dart/  │ ──────>│  api::emulator   │
│  Dart Native API     │              │  frb_generated.rs     │        │  Rust Core logic │
└──────────────────────┘              └───────────────────────┘        └──────────────────┘
```

---

## 2. Generating the Bridge Bindings

The bridge generation compiles the Rust code exposed under `core/src/api/` and creates native binding wrappers for Dart automatically.

Run this command from the root of the project to generate the bridge:
```powershell
flutter_rust_bridge_codegen generate
```

This updates:
- `core/src/frb_generated.rs` (Native C bindings registry).
- `app/lib/src/rust/frb_generated.dart` (Dart FFI dynamic loading library).
- `app/lib/src/rust/api/emulator.dart` (Dart representation of exposed Rust functions).

---

## 3. Stateful API Management

To avoid leaking raw memory pointers across the boundary, RessuRetro manages J2ME state directly in Rust using a thread-safe static mutex:

```rust
// core/src/api/emulator.rs
static EMULATOR_RUNTIME: Mutex<Option<J2meRuntime>> = Mutex::new(None);
```

Dart simply calls functions without managing pointers:

```dart
// app/lib/features/emulator/logic/emulator_controller.dart
import '../../../src/rust/api/emulator.dart' as rust;

await rust.initEmulator();
await rust.loadGame(jarPath: jarPath);
await rust.startGame();
```

---

## 4. Automatic Type Conversions

`flutter_rust_bridge` automatically maps complex Rust types to Dart types:

| Rust Type | Dart Type | Description |
| :--- | :--- | :--- |
| `String` | `String` | Automated UTF-8 conversion without `toNativeUtf8()` |
| `Vec<u32>` | `Uint32List` | Memory-safe vector mapping |
| `Result<T, String>` | `Future<T>` / `ThrowsException` | Map `Err` to Dart exceptions automatically |

### Accessing the Screen Buffer
In FRB, returning `Vec<u32>` provides a clean, safe array of pixels:

```rust
// core/src/api/emulator.rs
pub fn get_screen_pixels() -> Vec<u32> {
    // Returns active pixels
}
```

Dart accesses this list directly:
```dart
final List<int> pixels = await rust.getScreenPixels();
```

---

## 5. Execution Scheduling

Calling Rust functions asynchronously using FRB does not block the Flutter main thread, as FRB uses Rust thread pools to execute functions in the background.

To continuously fetch frames:
- Set up a periodic timer in Dart.
- Request screen frame updates asynchronously from the Rust thread pool.
- Add frames to the stream controller to trigger custom painter repaints.
