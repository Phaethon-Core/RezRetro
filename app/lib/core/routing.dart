// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'package:flutter/material.dart';
import '../features/library/screens/library_screen.dart';
import '../features/emulator/screens/emulator_screen.dart';
import '../features/settings/screens/settings_screen.dart';
import '../features/mods/screens/mods_screen.dart';

class RessuRouter {
  static const String root = '/';
  static const String emulator = '/emulator';
  static const String settings = '/settings';
  static const String mods = '/mods';

  static Route<dynamic> generateRoute(RouteSettings routeSettings) {
    switch (routeSettings.name) {
      case root:
        return MaterialPageRoute(builder: (_) => const LibraryScreen());
      case emulator:
        final jarPath = routeSettings.arguments as String? ?? '';
        return MaterialPageRoute(builder: (_) => EmulatorScreen(jarPath: jarPath));
      case settings:
        return MaterialPageRoute(builder: (_) => const SettingsScreen());
      case mods:
        return MaterialPageRoute(builder: (_) => const ModsScreen());
      default:
        return MaterialPageRoute(
          builder: (_) => Scaffold(
            body: Center(child: Text('No route defined for ${routeSettings.name}')),
          ),
        );
    }
  }
}
