// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'package:flutter/material.dart';
import '../../../../widgets/ressu_card.dart';
import '../../../../core/theme.dart';

class GameCard extends StatefulWidget {
  final String title;
  final String version;
  final String vendor;
  final VoidCallback onTap;

  const GameCard({
    Key? key,
    required this.title,
    required this.version,
    required this.vendor,
    required this.onTap,
  }) : super(key: key);

  @override
  State<GameCard> createState() => _GameCardState();
}

class _GameCardState extends State<GameCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final monogram = widget.title.isNotEmpty ? widget.title[0].toUpperCase() : '?';

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: RessuCard(
        padding: const EdgeInsets.all(12),
        onTap: widget.onTap,
        child: Row(
          children: [
            AnimatedContainer(
              duration: RessuTheme.animationDuration,
              curve: RessuTheme.animationCurve,
              width: 52,
              height: 52,
              decoration: BoxDecoration(
                color: _isHovered
                    ? RessuTheme.primaryColor.withOpacity(0.15)
                    : RessuTheme.primaryColor.withOpacity(0.06),
                borderRadius: BorderRadius.circular(12),
                border: Border.all(
                  color: _isHovered
                      ? RessuTheme.primaryColor.withOpacity(0.3)
                      : Colors.white.withOpacity(0.04),
                ),
              ),
              alignment: Alignment.center,
              child: Text(
                monogram,
                style: const TextStyle(
                  fontFamily: 'Rajdhani',
                  fontSize: 22,
                  fontWeight: FontWeight.bold,
                  color: RessuTheme.primaryColor,
                ),
              ),
            ),
            const SizedBox(width: 16),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    widget.title,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 16,
                      fontWeight: FontWeight.bold,
                      color: RessuTheme.textColor,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    '${widget.vendor} • v${widget.version}',
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(
                      fontFamily: 'Outfit',
                      fontSize: 12,
                      color: RessuTheme.subtitleColor,
                    ),
                  ),
                ],
              ),
            ),
            AnimatedPadding(
              duration: RessuTheme.animationDuration,
              curve: RessuTheme.animationCurve,
              padding: EdgeInsets.only(right: _isHovered ? 4.0 : 8.0, left: _isHovered ? 8.0 : 4.0),
              child: const Icon(
                Icons.chevron_right,
                color: RessuTheme.primaryColor,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
