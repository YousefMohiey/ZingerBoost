import 'package:flutter/material.dart';

const _success = Color(0xFF10B981);
const _error = Color(0xFFEF4444);
const _warning = Color(0xFFF59E0B);
const _info = Color(0xFF0EA5E9);
const _surface = Color(0xFF171717);

class ToastOverlay {
  ToastOverlay._();

  static void show(BuildContext context, String message, {String type = 'success'}) {
    final (color, icon) = switch (type) {
      'success' => (_success, Icons.check_circle_rounded),
      'error' => (_error, Icons.cancel_rounded),
      'warning' => (_warning, Icons.warning_rounded),
      'info' => (_info, Icons.info_rounded),
      _ => (_info, Icons.info_rounded),
    };

    ScaffoldMessenger.of(context).clearSnackBars();
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Row(
          children: [
            Icon(icon, color: color, size: 20),
            const SizedBox(width: 10),
            Expanded(
              child: Text(
                message,
                style: const TextStyle(
                  color: Colors.white,
                  fontSize: 14,
                  fontWeight: FontWeight.w500,
                ),
              ),
            ),
          ],
        ),
        backgroundColor: _surface,
        behavior: SnackBarBehavior.floating,
        margin: const EdgeInsets.all(16),
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(10),
          side: BorderSide(color: color.withValues(alpha: 0.3)),
        ),
        duration: const Duration(seconds: 3),
        dismissDirection: DismissDirection.horizontal,
      ),
    );
  }
}
