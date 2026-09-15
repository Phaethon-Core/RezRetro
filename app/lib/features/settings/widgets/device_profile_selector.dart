// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'package:flutter/material.dart';
import '../../../../widgets/ressu_card.dart';
import '../../../../core/theme.dart';

class DeviceProfileSelector extends StatelessWidget {
  final String activeProfile;
  final Function(String) onProfileSelected;

  const DeviceProfileSelector({
    Key? key,
    required this.activeProfile,
    required this.onProfileSelected,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final profiles = ['Nokia 240x320', 'Sony Ericsson 176x220', 'Samsung 128x160'];
    return RessuCard(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'Target Device Profile',
            style: TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              color: RessuTheme.textColor,
            ),
          ),
          const SizedBox(height: 12),
          Column(
            children: profiles.map((profile) {
              return RadioListTile<String>(
                title: Text(profile, style: const TextStyle(color: RessuTheme.textColor)),
                value: profile,
                groupValue: activeProfile,
                activeColor: RessuTheme.accentColor,
                onChanged: (val) {
                  if (val != null) onProfileSelected(val);
                },
              );
            }).toList(),
          ),
        ],
      ),
    );
  }
}
