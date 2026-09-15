// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'dart:io';
import '../../../../core/constants.dart';

class LibraryController {
  Future<List<File>> loadLocalGames() async {
    final dir = Directory(RessuConstants.gamesDirectory);
    if (!await dir.exists()) {
      await dir.create(recursive: true);
    }
    
    final List<File> jarFiles = [];
    try {
      final list = dir.listSync();
      for (var item in list) {
        if (item is File && item.path.endsWith('.jar')) {
          jarFiles.add(item);
        }
      }
    } catch (_) {
      // Return empty list on failure
    }
    
    return jarFiles;
  }
}
