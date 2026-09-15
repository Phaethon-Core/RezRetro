// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'dart:async';
import '../../../src/rust/api/emulator.dart' as rust;

class EmulatorController {
  bool isRunning = false;
  Timer? _tickTimer;
  final StreamController<List<int>> _screenStream = StreamController<List<int>>.broadcast();

  Stream<List<int>> get screenStream => _screenStream.stream;

  Future<void> launchGame(String jarPath) async {
    await rust.initEmulator();
    await rust.loadGame(jarPath: jarPath);
    await rust.startGame();
    isRunning = true;
    
    // Simulate game ticks at 30 fps
    _tickTimer = Timer.periodic(const Duration(milliseconds: 33), (timer) async {
      if (isRunning) {
        final pixels = await rust.getScreenPixels();
        _screenStream.add(pixels);
      }
    });
  }

  Future<void> pause() async {
    if (isRunning) {
      await rust.pauseGame();
      isRunning = false;
    }
  }

  Future<void> resume() async {
    if (!isRunning) {
      await rust.resumeGame();
      isRunning = true;
    }
  }

  Future<void> dispose() async {
    _tickTimer?.cancel();
    isRunning = false;
    await rust.stopGame();
    _screenStream.close();
  }
}
