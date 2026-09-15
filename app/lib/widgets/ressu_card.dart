// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'package:flutter/material.dart';
import '../core/theme.dart';

class RessuCard extends StatefulWidget {
  final Widget child;
  final EdgeInsetsGeometry padding;
  final double? width;
  final double? height;
  final VoidCallback? onTap;

  const RessuCard({
    Key? key,
    required this.child,
    this.padding = const EdgeInsets.all(16),
    this.width,
    this.height,
    this.onTap,
  }) : super(key: key);

  @override
  State<RessuCard> createState() => _RessuCardState();
}

class _RessuCardState extends State<RessuCard> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    final isInteractable = widget.onTap != null;

    Widget cardContent = AnimatedContainer(
      duration: RessuTheme.animationDuration,
      curve: RessuTheme.animationCurve,
      width: widget.width,
      height: widget.height,
      padding: widget.padding,
      decoration: BoxDecoration(
        color: _isHovered && isInteractable
            ? const Color(0xFF1C1C24)
            : RessuTheme.cardColor,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: _isHovered && isInteractable
              ? RessuTheme.primaryColor.withOpacity(0.4)
              : Colors.white.withOpacity(0.08),
          width: 1.2,
        ),
        boxShadow: [
          BoxShadow(
            color: _isHovered && isInteractable
                ? RessuTheme.primaryColor.withOpacity(0.15)
                : Colors.black.withOpacity(0.2),
            blurRadius: _isHovered && isInteractable ? 16 : 10,
            spreadRadius: _isHovered && isInteractable ? 1 : 0,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: widget.child,
    );

    if (isInteractable) {
      return MouseRegion(
        onEnter: (_) => setState(() => _isHovered = true),
        onExit: (_) => setState(() => _isHovered = false),
        child: AnimatedScale(
          scale: _isHovered ? 1.03 : 1.0,
          duration: RessuTheme.animationDuration,
          curve: RessuTheme.animationCurve,
          child: GestureDetector(
            onTap: widget.onTap,
            behavior: HitTestBehavior.opaque,
            child: cardContent,
          ),
        ),
      );
    }

    return cardContent;
  }
}
