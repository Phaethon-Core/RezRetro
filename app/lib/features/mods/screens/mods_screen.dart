// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'package:flutter/material.dart';
import '../../../../widgets/ressu_scaffold.dart';
import '../../../../core/theme.dart';
import '../logic/mods_controller.dart';
import '../widgets/mod_list_item.dart';

class ModsScreen extends StatefulWidget {
  const ModsScreen({Key? key}) : super(key: key);

  @override
  State<ModsScreen> createState() => _ModsScreenState();
}

class _ModsScreenState extends State<ModsScreen> {
  final ModsController _controller = ModsController();
  List<Mod> _mods = [];
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _loadMods();
  }

  Future<void> _loadMods() async {
    setState(() => _isLoading = true);
    final mods = await _controller.loadInstalledMods();
    setState(() {
      _mods = mods;
      _isLoading = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    return RessuScaffold(
      title: 'Mod Configuration',
      body: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const SizedBox(height: 16),
            const Text(
              'Available Mods',
              style: TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.bold,
                color: RessuTheme.textColor,
              ),
            ),
            const SizedBox(height: 8),
            const Text(
              'Enable or disable resource overrides for RessuRetro J2ME games.',
              style: TextStyle(fontSize: 14, color: RessuTheme.subtitleColor),
            ),
            const SizedBox(height: 16),
            Expanded(
              child: _isLoading
                  ? const Center(child: CircularProgressIndicator())
                  : _mods.isEmpty
                      ? const Center(
                          child: Text(
                            'No mods installed. Place mods inside data/mods/',
                            style: TextStyle(color: RessuTheme.subtitleColor),
                          ),
                        )
                      : ListView.separated(
                          itemCount: _mods.length,
                          separatorBuilder: (_, __) => const SizedBox(height: 12),
                          itemBuilder: (context, index) {
                            final mod = _mods[index];
                            return ModListItem(
                              mod: mod,
                              onToggle: (val) {
                                setState(() {
                                  mod.enabled = val;
                                });
                                _controller.toggleMod(mod.id, val);
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
