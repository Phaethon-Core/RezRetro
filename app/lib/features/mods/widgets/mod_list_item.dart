// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'package:flutter/material.dart';
import '../../../../widgets/ressu_card.dart';
import '../../../../core/theme.dart';
import '../logic/mods_controller.dart';

class ModListItem extends StatelessWidget {
  final Mod mod;
  final Function(bool) onToggle;

  const ModListItem({
    Key? key,
    required this.mod,
    required this.onToggle,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return RessuCard(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      child: Row(
        children: [
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  mod.name,
                  style: const TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                    color: RessuTheme.textColor,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  'Author: ${mod.author} • v${mod.version}',
                  style: const TextStyle(
                    fontSize: 12,
                    color: RessuTheme.subtitleColor,
                  ),
                ),
              ],
            ),
          ),
          Switch(
            value: mod.enabled,
            activeColor: RessuTheme.accentColor,
            onChanged: onToggle,
          ),
        ],
      ),
    );
  }
}
