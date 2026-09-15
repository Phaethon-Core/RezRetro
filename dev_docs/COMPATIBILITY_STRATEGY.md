# Game Compatibility Strategy

Achieving high compatibility with real-world J2ME games is the primary goal of the RessuRetro emulator. This document outlines our strategy for game validation, vendor API integration, and troubleshooting.

---

## 1. Real-Game Testing Suite

Static specifications are not enough to ensure compatibility, as many historical mobile games relied on undocumented behaviors, vendor-specific implementation quirks, or non-standard class configurations.

### Automated Test Harness
- Implement an automated test runner inside Rust (`core/tests/`) that can load a directory of test JARs.
- These test JARs run in headless mode inside the emulator, verifying:
  - Valid manifest parsing and entry point location.
  - Successful execution of basic JVM stack operations.
  - Zero crashes during initialization of the main MIDlet class.
  - Basic RMS storage serialization.

### Headless Verification Flow
```text
  Test Directory
┌─────────────────┐
│ game_test.jar   │ ──> Load Emulator ──> Execute VM ──> Verify no runtime panic
└─────────────────┘
```

---

## 2. Vendor API Isolation

Real games frequently imported packages outside the standard CLDC/MIDP specs. We isolate these external APIs behind abstraction traits in Rust:

### Abstraction Trait Pattern
Do not pollute the core JVM interpreter with vendor-specific cases. Instead, define class overrides using a modular native library dispatch:

```rust
pub trait VendorApiHandler {
    fn handle_static_call(&self, class: &str, method: &str, args: &[VmValue]) -> Option<VmValue>;
    fn create_vendor_object(&self, class: &str) -> Option<VmObject>;
}
```

### Implementing Major Vendor API Libraries
- **Nokia UI (`com.nokia.mid.ui`)**: Expose `DirectUtils` subclass functions to allow games to create mutable images and execute direct scaling operations.
- **Nokia Sound (`com.nokia.mid.sound`)**: Map tone channels and basic vibration methods to the host audio output.
- **Samsung Utility (`com.samsung.util`)**: Handle vibration request triggers.
- **Sony Ericsson (`com.sonyericsson.vibration`)**: Expose SE-specific vibration callbacks.

---

## 3. Per-Game Device Profiles

Different phone models had different screen sizes, keypad designs, and CPU execution limits. RessuRetro supports per-game configurations via `DeviceProfile` configs:

### Profile Attributes
When a JAR is loaded, RessuRetro checks for a matching JSON profile in `data/config/games/<game_id>.json`. This profile configures:
- **Display Resolution**: Override virtual screen size (e.g. force `128x160` for older Nokia titles, `240x320` for standard MIDP 2.0).
- **Aspect Ratio Fit**: Choose between integer pixel scaling, aspect-preserving stretch, or full stretch.
- **Key Mappings**: Override default PC keyboard controls specifically for games requiring custom key layouts.
- **API Quirks**: Enable specific compatibility flags (e.g. ignore missing classes, return dummy values for network operations).

---

## 4. Compatibility Troubleshooting Protocol

If a game fails to run:
1. **Analyze logs**: Look at the class loading console prints. Identify if the VM crashed on a missing class or an unsupported bytecode instruction.
2. **Stub missing classes**: If a game imports a proprietary vendor API class that is missing, create a stub class in Rust that returns default values instead of crashing.
3. **Verify screen coordinates**: If rendering looks distorted or elements are cut off, modify the `DeviceProfile` to match the exact phone resolution the game was originally designed for.
