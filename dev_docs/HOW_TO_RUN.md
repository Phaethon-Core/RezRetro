# How to Build and Run in Development

This document describes the compilation pipeline for the Rust core library and how to run the Flutter frontend in development mode.

---

## 1. Generating Bridge Bindings & Compiling the Rust Core

Before compiling the Rust core library, you must generate the Dart-Rust communication bindings using `flutter_rust_bridge_codegen`.

Run this command from the root directory:
```powershell
flutter_rust_bridge_codegen generate
```

After generating the bridge files, compile the Rust emulator core (`core/`) into a platform-specific dynamic library (`ressu_core.dll` on Windows, `libressu_core.so` on Linux/Android):

### Compiling for Windows
Build the library in debug or release mode using cargo:
```powershell
# From the root directory:
cargo build --package ressu_core
```
The output will be located at:
`target/debug/ressu_core.dll`

### Compiling for Linux
Build the library:
```bash
cargo build --package ressu_core
```
The output will be located at:
`target/debug/libressu_core.so`

### Compiling for Android
Android requires cross-compilation for target devices using `cargo-ndk`. Make sure the NDK targets are installed (see [Development Setup](DEVELOPMENT_SETUP.md)).

Run this command to build the dynamic libraries for arm64 (modern phones) and x86_64 (emulators) and place them directly in the Android build directories:
```powershell
# Create the jniLibs target directories inside the Flutter app
mkdir -p app/android/app/src/main/jniLibs/arm64-v8a
mkdir -p app/android/app/src/main/jniLibs/x86_64

# Build for ARM64-v8a
cargo ndk -t arm64-v8a -o app/android/app/src/main/jniLibs/ build --package ressu_core --lib

# Build for x86_64 (for Android Emulator)
cargo ndk -t x86_64 -o app/android/app/src/main/jniLibs/ build --package ressu_core --lib
```

---

## 2. Setting Up Dynamic Libraries for Desktop Runs

Before launching the Flutter desktop applications, you must ensure the built dynamic library is available to the Dart runtime.

### Windows Desktop Configuration
To load the DLL successfully when debugging the Windows app:
1. Copy the DLL from `target/debug/ressu_core.dll` directly to the Flutter application build output folder, or place it in the application's binary directory:
   ```powershell
   # Compile core
   cargo build --package ressu_core
   
   # Run Flutter build to generate directory structure
   cd app
   flutter build windows --debug
   
   # Copy the DLL into the output folder next to the executable
   copy ..\target\debug\ressu_core.dll build\windows\x64\runner\Debug\
   ```

2. Alternatively, you can copy the DLL to the root of the `app/` folder so it can be loaded in Dart scripts.

### Linux Desktop Configuration
For Linux desktop debugging:
```bash
# Compile core
cargo build --package ressu_core

# Build directory structure
cd app
flutter build linux --debug

# Copy the shared object next to the runner binary
cp ../target/debug/libressu_core.so build/linux/x64/debug/bundle/lib/
```

---

## 3. Running the Flutter Application

Once the native libraries are placed, navigate to the `app/` directory and use the standard Flutter commands.

### Check Connected Devices
Ensure your PC, Android emulator, or connected physical device is recognized:
```powershell
flutter devices
```

### Running the App
Run the app in debug mode on your chosen platform:
```powershell
# Run on Windows
flutter run -d windows

# Run on Linux
flutter run -d linux

# Run on a connected Android phone or emulator
flutter run -d <device-id>
```

### Hot Reload / Hot Restart
- Press `r` in the terminal to trigger a **Hot Reload** (for Dart code modifications).
- Press `R` to trigger a **Hot Restart** (clears app state, reinits FFI and JVM).
- Note: Changing Rust code requires you to stop the app, rebuild the Rust library using `cargo build`, copy the new library to the build folders, and restart the Flutter app.
