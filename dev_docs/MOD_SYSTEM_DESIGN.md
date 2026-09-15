# Mod System Design

This document details the RessuRetro mod system, describing packaging formats, manifestations, load hierarchies, and security sandboxing.

---

## 1. Mod Preservation Architecture

J2ME game preservation requires that the original game files (`.jar` / `.jad`) remain completely untouched. 

To modify graphics, sound, or translations, RessuRetro uses a **Runtime Override Layer**. This layer overlays resources in memory, intercepting guest assets requests at runtime:

```text
  Original JAR File
┌─────────────────────┐
│  assets/player.png  │ ──┐
└─────────────────────┘   │   Virtual File Loader
                          ├──> (Checks Overrides) ──> Loaded Player Asset
┌─────────────────────┐   │   If found: use Mod
│  assets/player.png  │ ──┘   If not: use original
└─────────────────────┘
  Mod ZIP Package
```

---

## 2. Package & Manifest Specification

Mods are distributed as standard ZIP containers with the suffix `.jar-mod.zip`.

### Directory Layout
```text
MyMod.jar-mod.zip
├── manifest.json       - Mandatory metadata description file
├── assets/             - Asset overrides folder (replicates target JAR layout)
│   ├── player.png      - Overrides the player sprite
│   └── music.mid       - Overrides default background track
└── patches/            - Future bytecode patch descriptors
```

### Manifest Format (`manifest.json`)
```json
{
  "id": "com.example.hdtextures",
  "name": "High-Definition Texture Pack",
  "version": "1.0.2",
  "author": "Preservationist",
  "description": "Replaces original pixel-art tiles with clean high-res alternatives.",
  "target_game": "com.nokia.spacerebel",
  "target_game_version": ">=1.0.0",
  "min_emulator_version": "0.1.0",
  "dependencies": [],
  "conflicts": []
}
```

---

## 3. Installation & Lifecycle

### Installation Flow
1. The user selects a `.jar-mod.zip` in Flutter.
2. Flutter copies the zip file to `data/mods/<mod_id>.jar-mod.zip`.
3. Rust parses the mod manifest to validate compatibility with the active game.

### Enabling a Mod
- When enabled, Rust opens the mod ZIP, scans files starting with `assets/`, reads them into memory, and inserts them into the `ModSystem::overrides` hashmap.
- Keys are mapped relative to the assets folder (e.g. `assets/player.png` becomes `player.png`).

### Disabling a Mod
- The mod flag is set to `enabled = false`.
- To ensure correct load order, all overrides are cleared, and only the remaining active mods are re-loaded sequentially.

---

## 4. Conflict & Load Ordering

If multiple mods override the same asset:
- **Load Order Priority**: Mods are loaded in the order they are configured in settings. A mod loaded later overrides assets from a mod loaded earlier.
- **Dependency Resolving**: If a mod lists a dependency, that dependency must be enabled and loaded first.
- **Conflicts**: If a mod lists a conflict (e.g. conflicting HD packs), the emulator blocks execution and displays a warning to the user.

---

## 5. Security & Isolation

Mods pose a potential vector for malicious scripts. To enforce a secure environment:
- **Asset Isolation**: Mods only have write access inside the `data/mods/` directory.
- **Memory-Only Overrides**: Resource replacement is restricted to JVM memory space; guest code cannot execute native host commands.
- **No Host File Access**: Mods cannot access the user's host operating system. If scripting (e.g. Lua) is added in future, it must run inside a strict sandbox.
