# Development Environment Setup

This document outlines the prerequisites and setup instructions required to develop, build, and debug the RessuRetro emulator.

## Prerequisites

RessuRetro is a cross-platform project utilizing Rust for the emulation core and Flutter for the user interface. You will need to install toolchains for both environments.

### 1. General Tools
- **Git**: For version control and source code management.

### 2. Rust Toolchain
- **Rustup**: The official installer and version manager for Rust. Install it from [rustup.rs](https://rustup.rs/).
- **Cargo**: Packaged automatically with Rustup. The project uses the **2021 edition**.
- Ensure you have the stable toolchain installed:
  ```powershell
  rustup default stable
  ```

### 3. Flutter & Dart SDK
- **Flutter SDK**: Install the latest stable version of Flutter from [flutter.dev](https://docs.flutter.dev/get-started/install).
- Add the `flutter` binary folder to your system's `PATH`.
- Verify the installation:
  ```powershell
  flutter doctor
  ```

---

## Platform-Specific Native Requirements

To build the Rust core dynamic library (`.dll` or `.so`) and run the Flutter application on your desktop, you must install the platform SDKs.

### Windows (Host)
- **Visual Studio 2022**: Install Visual Studio (Community, Professional, or Enterprise).
- **C++ Workload**: During installation, select the **Desktop development with C++** workload. This installs the MSVC compiler, Windows SDK, and C++ build tools.

### Linux (Host)
Install the required system compilers and packages.
- **Ubuntu/Debian**:
  ```bash
  sudo apt update
  sudo apt install -y build-essential clang cmake ninja-build pkg-config libgtk-3-dev liblzma-dev
  ```

### Android (Target)
To build and run on Android, you must configure the Android SDK and Rust targets.
1. **Android Studio**: Download and install Android Studio.
2. **SDK Tools**: Open Android Studio SDK Manager and install:
   - Android SDK Build-Tools
   - Android NDK (Side-by-side) - required for compiling the Rust library to Android
   - Android SDK Command-line Tools
   - Android Emulator
3. **Rust Android Targets**: Add the compilation targets for Android architectures (ARM64, ARM, x86_64):
   ```powershell
   rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
   ```
4. **Cargo NDK**: Install `cargo-ndk` to simplify compiling Rust code with the Android NDK:
   ```powershell
   cargo install cargo-ndk
   ```

---

## Recommended Editor Configuration

### Visual Studio Code
This is the recommended IDE for lightweight development. Install the following extensions:
- **rust-analyzer**: Provides rich language features for Rust.
- **Flutter**: Enables Flutter GUI editing, launching, and debugging.
- **Dart**: Auto-installed with the Flutter extension.
- **Even Better TOML**: Syntax highlighting for Cargo configuration files.

### Android Studio / IntelliJ IDEA
Excellent for Android-specific work and full workspace integration.
- Install the **Rust** plugin from the marketplace.
- Install the **Flutter** and **Dart** plugins.
