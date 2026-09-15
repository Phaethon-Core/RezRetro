// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'package:flutter/material.dart';
import '../../../../widgets/ressu_scaffold.dart';
import '../../../../widgets/ressu_card.dart';
import '../../../../widgets/ressu_button.dart';
import '../../../../core/theme.dart';
import '../logic/settings_controller.dart';
import '../widgets/device_profile_selector.dart';

class SettingsScreen extends StatefulWidget {
  const SettingsScreen({Key? key}) : super(key: key);

  @override
  State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
  final SettingsController _controller = SettingsController();

  @override
  Widget build(BuildContext context) {
    return RessuScaffold(
      title: 'Settings',
      body: SingleChildScrollView(
        padding: const EdgeInsets.symmetric(horizontal: 16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const SizedBox(height: 16),
            const Text(
              'AUDIO',
              style: TextStyle(
                fontFamily: 'Rajdhani',
                fontSize: 14,
                fontWeight: FontWeight.bold,
                color: RessuTheme.primaryColor,
                letterSpacing: 1.0,
              ),
            ),
            const SizedBox(height: 8),
            RessuCard(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    'Master Volume',
                    style: TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 16,
                      fontWeight: FontWeight.bold,
                      color: RessuTheme.textColor,
                    ),
                  ),
                  const SizedBox(height: 12),
                  Row(
                    children: [
                      const Icon(Icons.volume_up, color: RessuTheme.subtitleColor, size: 20),
                      Expanded(
                        child: SliderTheme(
                          data: SliderTheme.of(context).copyWith(
                            activeTrackColor: RessuTheme.primaryColor,
                            inactiveTrackColor: Colors.white10,
                            thumbColor: RessuTheme.primaryColor,
                            overlayColor: RessuTheme.primaryColor.withOpacity(0.12),
                          ),
                          child: Slider(
                            value: _controller.volume,
                            min: 0.0,
                            max: 100.0,
                            onChanged: (val) {
                              setState(() {
                                _controller.updateVolume(val);
                              });
                            },
                          ),
                        ),
                      ),
                      Text(
                        '${_controller.volume.round()}%',
                        style: const TextStyle(
                          fontFamily: 'Outfit',
                          color: RessuTheme.textColor,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
            const SizedBox(height: 24),
            const Text(
              'EMULATION DEVICE',
              style: TextStyle(
                fontFamily: 'Rajdhani',
                fontSize: 14,
                fontWeight: FontWeight.bold,
                color: RessuTheme.primaryColor,
                letterSpacing: 1.0,
              ),
            ),
            const SizedBox(height: 8),
            DeviceProfileSelector(
              activeProfile: _controller.currentProfile,
              onProfileSelected: (profile) {
                setState(() {
                  _controller.updateProfile(profile);
                });
              },
            ),
            const SizedBox(height: 32),
            Center(
              child: RessuButton(
                text: 'Manage Mods',
                icon: Icons.extension,
                onTap: () {
                  Navigator.pushNamed(context, '/mods');
                },
              ),
            ),
            const SizedBox(height: 24),
          ],
        ),
      ),
    );
  }
}
