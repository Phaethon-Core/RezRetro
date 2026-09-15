// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'package:flutter/material.dart';

class RessuTheme {
  static const Color primaryColor = Color(0xFF00D4FF);
  static const Color accentColor = Color(0xFF008B99);
  static const Color backgroundColor = Color(0xFF0A0A0F);
  static const Color surfaceColor = Color(0xFF141419);
  static const Color cardColor = Color(0xCC141419);
  static const Color textColor = Color(0xFFE8E8ED);
  static const Color subtitleColor = Color(0xFF71717A);
  static const Color dangerColor = Color(0xFFFF6B6B);

  static const Duration animationDuration = Duration(milliseconds: 200);
  static const Curve animationCurve = Curves.easeOutCubic;

  static ThemeData get darkTheme {
    return ThemeData(
      brightness: Brightness.dark,
      primaryColor: primaryColor,
      scaffoldBackgroundColor: backgroundColor,
      cardColor: surfaceColor,
      fontFamily: 'Outfit',
      textTheme: const TextTheme(
        headlineMedium: TextStyle(
          fontFamily: 'Rajdhani',
          fontSize: 26,
          fontWeight: FontWeight.bold,
          color: textColor,
          letterSpacing: 0.8,
        ),
        titleLarge: TextStyle(
          fontFamily: 'Rajdhani',
          fontSize: 20,
          fontWeight: FontWeight.w600,
          color: textColor,
          letterSpacing: 0.5,
        ),
        bodyMedium: TextStyle(
          fontFamily: 'Outfit',
          fontSize: 14,
          color: subtitleColor,
        ),
      ),
      colorScheme: const ColorScheme.dark(
        primary: primaryColor,
        secondary: accentColor,
        surface: surfaceColor,
        background: backgroundColor,
        error: dangerColor,
      ),
    );
  }
}
