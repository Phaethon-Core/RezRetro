// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'dart:io';
import '../../../../core/constants.dart';
import '../../../src/rust/api/emulator.dart' as rust;

class Mod {
  final String id;
  final String name;
  final String version;
  final String author;
  bool enabled;

  Mod({
    required this.id,
    required this.name,
    required this.version,
    required this.author,
    this.enabled = false,
  });
}

class ModsController {
  Future<List<Mod>> loadInstalledMods() async {
    final dir = Directory(RessuConstants.modsDirectory);
    if (!await dir.exists()) {
      await dir.create(recursive: true);
    }
    
    return [
      Mod(
        id: 'com.ressuretro.hdpack',
        name: 'Retro HD Graphics Pack',
        version: '1.0.0',
        author: 'Phaethon',
      ),
      Mod(
        id: 'com.ressuretro.translation',
        name: 'Community Translation Patch',
        version: '0.9.0',
        author: 'RessuRetroTeam',
      ),
    ];
  }

  Future<void> toggleMod(String id, bool enabled) async {
    try {
      await rust.toggleMod(id: id, enable: enabled);
    } catch (_) {
      // Revert/ignore issues in development
    }
  }
}
