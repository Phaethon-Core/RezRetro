// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'dart:io';
import 'package:flutter/material.dart';
import '../../../../widgets/ressu_scaffold.dart';
import '../../../../widgets/ressu_button.dart';
import '../../../../core/routing.dart';
import '../../../../core/theme.dart';
import '../widgets/game_card.dart';
import '../logic/library_controller.dart';

class LibraryScreen extends StatefulWidget {
  const LibraryScreen({Key? key}) : super(key: key);

  @override
  State<LibraryScreen> createState() => _LibraryScreenState();
}

class _LibraryScreenState extends State<LibraryScreen> {
  final LibraryController _controller = LibraryController();
  List<File> _games = [];
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _loadGames();
  }

  Future<void> _loadGames() async {
    setState(() => _isLoading = true);
    final games = await _controller.loadLocalGames();
    setState(() {
      _games = games;
      _isLoading = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    return RessuScaffold(
      title: 'RessuRetro',
      actions: [
        IconButton(
          icon: const Icon(Icons.settings, color: RessuTheme.textColor),
          onPressed: () => Navigator.pushNamed(context, RessuRouter.settings),
        ),
      ],
      body: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const SizedBox(height: 16),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Text(
                  'Your Games',
                  style: TextStyle(
                    fontFamily: 'Rajdhani',
                    fontSize: 22,
                    fontWeight: FontWeight.bold,
                    color: RessuTheme.textColor,
                    letterSpacing: 0.5,
                  ),
                ),
                RessuButton(
                  text: 'Refresh',
                  icon: Icons.refresh,
                  onTap: _loadGames,
                ),
              ],
            ),
            const SizedBox(height: 16),
            Expanded(
              child: _isLoading
                  ? const Center(
                      child: CircularProgressIndicator(
                        valueColor: AlwaysStoppedAnimation<Color>(RessuTheme.primaryColor),
                      ),
                    )
                  : _games.isEmpty
                      ? Center(
                          child: Column(
                            mainAxisAlignment: MainAxisAlignment.center,
                            children: [
                              Container(
                                width: 80,
                                height: 80,
                                decoration: BoxDecoration(
                                  color: RessuTheme.primaryColor.withOpacity(0.05),
                                  shape: BoxShape.circle,
                                  border: Border.all(
                                    color: RessuTheme.primaryColor.withOpacity(0.15),
                                    width: 1.5,
                                  ),
                                ),
                                child: const Icon(
                                  Icons.folder_open,
                                  size: 40,
                                  color: RessuTheme.primaryColor,
                                ),
                              ),
                              const SizedBox(height: 24),
                              const Text(
                                'No J2ME Games Found',
                                style: TextStyle(
                                  fontFamily: 'Rajdhani',
                                  fontSize: 18,
                                  fontWeight: FontWeight.bold,
                                  color: RessuTheme.textColor,
                                ),
                              ),
                              const SizedBox(height: 8),
                              const Text(
                                'Place .jar game files in your /data/games/ directory',
                                style: TextStyle(
                                  fontFamily: 'Outfit',
                                  fontSize: 13,
                                  color: RessuTheme.subtitleColor,
                                ),
                                textAlign: TextAlign.center,
                              ),
                            ],
                          ),
                        )
                      : ListView.separated(
                          itemCount: _games.length,
                          separatorBuilder: (_, __) => const SizedBox(height: 12),
                          itemBuilder: (context, index) {
                            final file = _games[index];
                            final title = file.path.split(Platform.pathSeparator).last.replaceAll('.jar', '');
                            return GameCard(
                              title: title,
                              vendor: 'J2ME MIDlet',
                              version: '1.0.0',
                              onTap: () {
                                Navigator.pushNamed(
                                  context,
                                  RessuRouter.emulator,
                                  arguments: file.path,
                                );
                              },
                            );
                          },
                        ),
            ),
          ],
        ),
      ),
    );
  }
}
