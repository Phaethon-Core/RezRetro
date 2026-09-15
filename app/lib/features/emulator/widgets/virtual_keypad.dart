// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'package:flutter/material.dart';
import '../../../core/theme.dart';

class VirtualKeypad extends StatelessWidget {
  final Function(int, bool) onKeyEvent;

  const VirtualKeypad({
    Key? key,
    required this.onKeyEvent,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: RessuTheme.cardColor,
        borderRadius: BorderRadius.circular(20),
        border: Border.all(color: Colors.white.withOpacity(0.06)),
      ),
      child: GridView.count(
        shrinkWrap: true,
        crossAxisCount: 3,
        childAspectRatio: 1.6,
        mainAxisSpacing: 10,
        crossAxisSpacing: 10,
        physics: const NeverScrollableScrollPhysics(),
        children: [
          KeypadKey(label: '1', code: 49, onKeyEvent: onKeyEvent),
          KeypadKey(label: '2', code: 50, onKeyEvent: onKeyEvent),
          KeypadKey(label: '3', code: 51, onKeyEvent: onKeyEvent),
          KeypadKey(label: '4', code: 52, onKeyEvent: onKeyEvent),
          KeypadKey(label: '5', code: 53, onKeyEvent: onKeyEvent),
          KeypadKey(label: '6', code: 54, onKeyEvent: onKeyEvent),
          KeypadKey(label: '7', code: 55, onKeyEvent: onKeyEvent),
          KeypadKey(label: '8', code: 56, onKeyEvent: onKeyEvent),
          KeypadKey(label: '9', code: 57, onKeyEvent: onKeyEvent),
          KeypadKey(label: '*', code: 42, onKeyEvent: onKeyEvent),
          KeypadKey(label: '0', code: 48, onKeyEvent: onKeyEvent),
          KeypadKey(label: '#', code: 35, onKeyEvent: onKeyEvent),
        ],
      ),
    );
  }
}

class KeypadKey extends StatefulWidget {
  final String label;
  final int code;
  final Function(int, bool) onKeyEvent;

  const KeypadKey({
    Key? key,
    required this.label,
    required this.code,
    required this.onKeyEvent,
  }) : super(key: key);

  @override
  State<KeypadKey> createState() => _KeypadKeyState();
}

class _KeypadKeyState extends State<KeypadKey> {
  bool _isHovered = false;
  bool _isPressed = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: GestureDetector(
        onTapDown: (_) {
          setState(() => _isPressed = true);
          widget.onKeyEvent(widget.code, true);
        },
        onTapUp: (_) {
          setState(() => _isPressed = false);
          widget.onKeyEvent(widget.code, false);
        },
        onTapCancel: () {
          setState(() => _isPressed = false);
          widget.onKeyEvent(widget.code, false);
        },
        child: AnimatedScale(
          scale: _isPressed ? 0.95 : (_isHovered ? 1.05 : 1.0),
          duration: RessuTheme.animationDuration,
          curve: RessuTheme.animationCurve,
          child: AnimatedContainer(
            duration: RessuTheme.animationDuration,
            curve: RessuTheme.animationCurve,
            alignment: Alignment.center,
            decoration: BoxDecoration(
              color: _isPressed
                  ? RessuTheme.primaryColor.withOpacity(0.12)
                  : (_isHovered
                      ? Colors.white.withOpacity(0.08)
                      : Colors.white.withOpacity(0.03)),
              borderRadius: BorderRadius.circular(12),
              border: Border.all(
                color: _isPressed
                    ? RessuTheme.primaryColor
                    : (_isHovered
                        ? RessuTheme.primaryColor.withOpacity(0.4)
                        : Colors.white.withOpacity(0.06)),
                width: 1.2,
              ),
              boxShadow: (_isHovered || _isPressed)
                  ? [
                      BoxShadow(
                        color: RessuTheme.primaryColor.withOpacity(0.12),
                        blurRadius: 10,
                        spreadRadius: 1,
                      )
                    ]
                  : [],
            ),
            child: Text(
              widget.label,
              style: TextStyle(
                fontFamily: 'Rajdhani',
                color: (_isPressed || _isHovered)
                    ? RessuTheme.primaryColor
                    : RessuTheme.textColor,
                fontWeight: FontWeight.bold,
                fontSize: 18,
              ),
            ),
          ),
        ),
      ),
    );
  }
}
