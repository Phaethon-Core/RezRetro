// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'package:flutter/material.dart';
import 'core/theme.dart';
import 'core/routing.dart';
import 'src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const RessuRetroApp());
}

class RessuRetroApp extends StatelessWidget {
  const RessuRetroApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'RessuRetro',
      debugShowCheckedModeBanner: false,
      theme: RessuTheme.darkTheme,
      initialRoute: RessuRouter.root,
      onGenerateRoute: RessuRouter.generateRoute,
    );
  }
}
