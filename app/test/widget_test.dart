// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ressu_retro/core/theme.dart';

void main() {
  testWidgets('App theme and scaffold smoke test', (WidgetTester tester) async {
    await tester.pumpWidget(
      MaterialApp(
        theme: RessuTheme.darkTheme,
        home: const Scaffold(
          body: Center(
            child: Text('RezRetro'),
          ),
        ),
      ),
    );

    expect(find.text('RezRetro'), findsOneWidget);
  });
}
