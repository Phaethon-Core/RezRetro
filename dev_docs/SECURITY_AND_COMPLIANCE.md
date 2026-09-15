# Security & License Compliance

This document outlines the security specifications and Apache License 2.0 compliance checklist for RessuRetro.

---

## 1. Security Architecture

RessuRetro is designed to run entirely locally without requiring internet access or cloud databases. However, because J2ME game files (`.jar`) and mods are external user inputs, they must be treated as untrusted.

### A. Game Isolation
- **No Native Execution**: J2ME bytecode operates strictly inside the Rust VM interpreter. It does not compile to native host instructions and cannot access host APIs directly.
- **Isolated Storage**: Each game's RMS database is saved in a separate directory named after the game's unique identifier. Games cannot access other games' save directories.
- **Restricted APIs**: File system APIs (e.g. JSR 75 FileConnection) must be sandboxed. They must only read/write from a designated folder (`data/games/` or `data/saves/`) and are blocked from access to host directories (like Windows `System32` or user `Documents`).

### B. Mod Sandboxing
- **Resource Limits**: Resource replacement overrides are stored as raw bytes in memory. Mods cannot run executable files or execute command lines on the host.
- **Scripting Security**: If a scripting engine (e.g. Lua or WebAssembly) is integrated for gameplay modifications in the future, it must run inside a sandbox without access to system calls, standard output, or the network.

### C. Safe Parsing (Robust Input Handling)
- JAR files, manifests, and JVM class files are validated during parsing.
- The `ClassFile::parse` function performs bounds checking on binary offsets to prevent buffer overflows or panic loops when parsing malformed files.
- Big-endian byte order is parsed using type-safe APIs (`byteorder` crate) to handle architecture conversion securely.

---

## 2. Apache License 2.0 Compliance Checklist

RessuRetro is licensed under the **Apache License 2.0**. All developers must adhere to the following compliance checklist:

### 1. License & NOTICE Distribution
- Any distribution of RessuRetro (source or compiled binaries) must include:
  - A copy of the [LICENSE](../LICENSE) file.
  - A copy of the [NOTICE](../NOTICE) file containing the original copyright attributions.

### 2. SPDX Headers in Source Files
- Every new source file (`.rs`, `.dart`, `.yaml`, etc.) must include a machine-readable SPDX header at the top of the file:
  ```rust
  // SPDX-License-Identifier: Apache-2.0
  // Copyright 2026 Phaethon
  ```
- This applies to Rust files in `core/` and Dart files in `app/`.

### 3. Third-Party Licenses & Dependency Policy
- **Permissive Libraries Only**: Third-party libraries used in RessuRetro must have permissive licenses (e.g. MIT, BSD, Apache 2.0).
- **Copyleft Restrictions**: Do **NOT** import or link code licensed under copyleft licenses (like GPL or AGPL) into the core engine or frontend. Incorporating copyleft code forces the entire project to be licensed under those terms, violating the Apache 2.0 compliance requirement.
- **NOTICE Additions**: If a third-party dependency requires attribution notices, add them clearly to the project's root `NOTICE` file.

### 4. Code Modification Rule
- If you modify an existing Apache 2.0-licensed file that is copyrighted by another author, you must add a prominent notice in the file stating that you modified it, along with the date of modification.
