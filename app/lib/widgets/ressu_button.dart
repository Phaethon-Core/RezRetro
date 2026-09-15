// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

import 'package:flutter/material.dart';
import '../core/theme.dart';

class RessuButton extends StatefulWidget {
  final String text;
  final VoidCallback onTap;
  final IconData? icon;
  final bool isPrimary;

  const RessuButton({
    Key? key,
    required this.text,
    required this.onTap,
    this.icon,
    this.isPrimary = false,
  }) : super(key: key);

  @override
  State<RessuButton> createState() => _RessuButtonState();
}

class _RessuButtonState extends State<RessuButton> {
  bool _isHovered = false;

  @override
  Widget build(BuildContext context) {
    Color getTextColor() {
      if (widget.isPrimary) {
        return _isHovered ? RessuTheme.textColor : const Color(0xFF0A0A0F);
      }
      return RessuTheme.textColor;
    }

    Color getIconColor() {
      return getTextColor();
    }

    return MouseRegion(
      onEnter: (_) => setState(() => _isHovered = true),
      onExit: (_) => setState(() => _isHovered = false),
      child: AnimatedScale(
        scale: _isHovered ? 1.05 : 1.0,
        duration: RessuTheme.animationDuration,
        curve: RessuTheme.animationCurve,
        child: AnimatedContainer(
          duration: RessuTheme.animationDuration,
          curve: RessuTheme.animationCurve,
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(12),
            color: widget.isPrimary
                ? (_isHovered ? RessuTheme.accentColor : RessuTheme.primaryColor)
                : (_isHovered ? RessuTheme.primaryColor.withOpacity(0.08) : Colors.transparent),
            border: Border.all(
              color: widget.isPrimary
                  ? Colors.transparent
                  : (_isHovered ? RessuTheme.primaryColor : RessuTheme.primaryColor.withOpacity(0.3)),
              width: 1.5,
            ),
            boxShadow: _isHovered
                ? [
                    BoxShadow(
                      color: RessuTheme.primaryColor.withOpacity(0.2),
                      blurRadius: 16,
                      offset: const Offset(0, 4),
                    )
                  ]
                : [],
          ),
          child: Material(
            color: Colors.transparent,
            child: InkWell(
              borderRadius: BorderRadius.circular(12),
              onTap: widget.onTap,
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    if (widget.icon != null) ...[
                      Icon(widget.icon, color: getIconColor(), size: 18),
                      const SizedBox(width: 8),
                    ],
                    Text(
                      widget.text,
                      style: TextStyle(
                        fontFamily: 'Outfit',
                        color: getTextColor(),
                        fontWeight: FontWeight.bold,
                        fontSize: 15,
                        letterSpacing: 0.5,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
