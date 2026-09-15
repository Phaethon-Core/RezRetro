// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'dart:async';
import 'dart:typed_data';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';

class GameViewport extends StatefulWidget {
  final Stream<List<int>> pixelStream;

  const GameViewport({
    Key? key,
    required this.pixelStream,
  }) : super(key: key);

  @override
  State<GameViewport> createState() => _GameViewportState();
}

class _GameViewportState extends State<GameViewport> {
  StreamSubscription<List<int>>? _subscription;
  ui.Image? _cachedImage;
  bool _isDecoding = false;

  @override
  void initState() {
    super.initState();
    _subscription = widget.pixelStream.listen(_onPixelsFrame);
  }

  @override
  void dispose() {
    _subscription?.cancel();
    _cachedImage?.dispose();
    super.dispose();
  }

  Future<void> _onPixelsFrame(List<int> pixels) async {
    if (_isDecoding) return;
    _isDecoding = true;

    try {
      if (pixels.length < 240 * 320) return;

      final Uint8List rgbaBytes = Uint8List(240 * 320 * 4);
      int byteIdx = 0;
      for (int i = 0; i < 240 * 320; i++) {
        final int argb = pixels[i];
        final int a = (argb >> 24) & 0xFF;
        final int r = (argb >> 16) & 0xFF;
        final int g = (argb >> 8) & 0xFF;
        final int b = argb & 0xFF;
        rgbaBytes[byteIdx] = r;
        rgbaBytes[byteIdx + 1] = g;
        rgbaBytes[byteIdx + 2] = b;
        rgbaBytes[byteIdx + 3] = a;
        byteIdx += 4;
      }

      final Completer<ui.Image> completer = Completer<ui.Image>();
      ui.decodeImageFromPixels(
        rgbaBytes,
        240,
        320,
        ui.PixelFormat.rgba8888,
        completer.complete,
      );

      final newImage = await completer.future;
      if (mounted) {
        setState(() {
          _cachedImage?.dispose();
          _cachedImage = newImage;
        });
      } else {
        newImage.dispose();
      }
    } catch (_) {
      // Catch layout parsing exceptions
    } finally {
      _isDecoding = false;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.black,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: Colors.white.withOpacity(0.08), width: 1.5),
      ),
      child: AspectRatio(
        aspectRatio: 240 / 320,
        child: _cachedImage == null
            ? const Center(
                child: Text(
                  'INITIALIZING LCD...',
                  style: TextStyle(
                    fontFamily: 'Rajdhani',
                    fontWeight: FontWeight.bold,
                    color: Colors.white30,
                    letterSpacing: 1.0,
                  ),
                ),
              )
            : CustomPaint(
                painter: PixelPainter(_cachedImage!),
              ),
      ),
    );
  }
}

class PixelPainter extends CustomPainter {
  final ui.Image image;

  PixelPainter(this.image);

  @override
  void paint(Canvas canvas, Size size) {
    canvas.drawImageRect(
      image,
      Rect.fromLTWH(0, 0, image.width.toDouble(), image.height.toDouble()),
      Rect.fromLTWH(0, 0, size.width, size.height),
      Paint()..filterQuality = ui.FilterQuality.none,
    );
  }

  @override
  bool shouldRepaint(covariant PixelPainter oldDelegate) {
    return oldDelegate.image != image;
  }
}
