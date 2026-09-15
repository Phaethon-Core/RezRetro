// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../../../../widgets/ressu_scaffold.dart';
import '../../../../core/theme.dart';
import '../logic/emulator_controller.dart';
import '../widgets/game_viewport.dart';
import '../widgets/virtual_keypad.dart';
import '../../../../src/rust/api/emulator.dart' as rust;

class EmulatorScreen extends StatefulWidget {
  final String jarPath;

  const EmulatorScreen({
    Key? key,
    required this.jarPath,
  }) : super(key: key);

  @override
  State<EmulatorScreen> createState() => _ScreenState();
}

class _ScreenState extends State<EmulatorScreen> {
  final EmulatorController _controller = EmulatorController();
  final FocusNode _focusNode = FocusNode();

  @override
  void initState() {
    super.initState();
    _controller.launchGame(widget.jarPath);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _focusNode.requestFocus();
    });
  }

  @override
  void dispose() {
    _focusNode.dispose();
    _controller.dispose();
    super.dispose();
  }

  void _handleKeyEvent(int code, bool pressed) {
    rust.sendKeyEvent(keyCode: code, pressed: pressed);
  }

  Future<void> _quickSave() async {
    try {
      await rust.saveState(filepath: 'quicksave.json');
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('State Saved Successfully')),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Save State Failed: $e')),
        );
      }
    }
  }

  Future<void> _quickLoad() async {
    try {
      await rust.loadState(filepath: 'quicksave.json');
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('State Loaded Successfully')),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Load State Failed: $e')),
        );
      }
    }
  }

  void _onKeyboardKey(KeyEvent event) {
    final bool pressed = event is KeyDownEvent || event is KeyRepeatEvent;
    final bool released = event is KeyUpEvent;

    if (!pressed && !released) return;

    int? j2meCode;
    final LogicalKeyboardKey key = event.logicalKey;

    if (key == LogicalKeyboardKey.arrowUp) {
      j2meCode = -1;
    } else if (key == LogicalKeyboardKey.arrowDown) {
      j2meCode = -2;
    } else if (key == LogicalKeyboardKey.arrowLeft) {
      j2meCode = -3;
    } else if (key == LogicalKeyboardKey.arrowRight) {
      j2meCode = -4;
    } else if (key == LogicalKeyboardKey.enter || key == LogicalKeyboardKey.space) {
      j2meCode = -5;
    } else if (key == LogicalKeyboardKey.digit0 || key == LogicalKeyboardKey.numpad0) {
      j2meCode = 48;
    } else if (key == LogicalKeyboardKey.digit1 || key == LogicalKeyboardKey.numpad1) {
      j2meCode = 49;
    } else if (key == LogicalKeyboardKey.digit2 || key == LogicalKeyboardKey.numpad2) {
      j2meCode = 50;
    } else if (key == LogicalKeyboardKey.digit3 || key == LogicalKeyboardKey.numpad3) {
      j2meCode = 51;
    } else if (key == LogicalKeyboardKey.digit4 || key == LogicalKeyboardKey.numpad4) {
      j2meCode = 52;
    } else if (key == LogicalKeyboardKey.digit5 || key == LogicalKeyboardKey.numpad5) {
      j2meCode = 53;
    } else if (key == LogicalKeyboardKey.digit6 || key == LogicalKeyboardKey.numpad6) {
      j2meCode = 54;
    } else if (key == LogicalKeyboardKey.digit7 || key == LogicalKeyboardKey.numpad7) {
      j2meCode = 55;
    } else if (key == LogicalKeyboardKey.digit8 || key == LogicalKeyboardKey.numpad8) {
      j2meCode = 56;
    } else if (key == LogicalKeyboardKey.digit9 || key == LogicalKeyboardKey.numpad9) {
      j2meCode = 57;
    } else if (key == LogicalKeyboardKey.numpadMultiply || key == LogicalKeyboardKey.asterisk) {
      j2meCode = 42;
    } else if (key == LogicalKeyboardKey.numberSign) {
      j2meCode = 35;
    }

    if (j2meCode != null) {
      _handleKeyEvent(j2meCode, pressed);
    }
  }

  @override
  Widget build(BuildContext context) {
    return KeyboardListener(
      focusNode: _focusNode,
      autofocus: true,
      onKeyEvent: _onKeyboardKey,
      child: RessuScaffold(
        title: 'Emulation',
        actions: [
          IconButton(
            icon: const Icon(Icons.save, color: RessuTheme.primaryColor),
            tooltip: 'Quick Save',
            onPressed: _quickSave,
          ),
          IconButton(
            icon: const Icon(Icons.upload, color: RessuTheme.primaryColor),
            tooltip: 'Quick Load',
            onPressed: _quickLoad,
          ),
          IconButton(
            icon: const Icon(Icons.pause, color: RessuTheme.textColor),
            onPressed: () => _controller.pause(),
          ),
          IconButton(
            icon: const Icon(Icons.play_arrow, color: RessuTheme.textColor),
            onPressed: () => _controller.resume(),
          ),
        ],
        body: Center(
          child: SingleChildScrollView(
            padding: const EdgeInsets.symmetric(horizontal: 24),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                const SizedBox(height: 16),
                Container(
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(color: Colors.white.withOpacity(0.08)),
                    boxShadow: [
                      BoxShadow(
                        color: Colors.black.withOpacity(0.4),
                        blurRadius: 20,
                      ),
                    ],
                  ),
                  child: ClipRRect(
                    borderRadius: BorderRadius.circular(15),
                    child: GameViewport(pixelStream: _controller.screenStream),
                  ),
                ),
                const SizedBox(height: 32),
                VirtualKeypad(onKeyEvent: _handleKeyEvent),
                const SizedBox(height: 24),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
