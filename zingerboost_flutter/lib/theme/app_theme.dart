import 'package:flutter/material.dart';

class AppTheme {
  static const _brand = Color(0xFF0EA5E9);
  static const _safe = Color(0xFF10B981);
  static const _moderate = Color(0xFFF59E0B);
  static const _advanced = Color(0xFFEF4444);

  static final dark = ThemeData(
    brightness: Brightness.dark,
    scaffoldBackgroundColor: const Color(0xFF0A0A0A),
    colorScheme: ColorScheme.dark(
      primary: _brand,
      secondary: _brand,
      surface: const Color(0xFF171717),
    ),
    appBarTheme: const AppBarTheme(backgroundColor: Color(0xFF171717)),
    cardTheme: CardTheme(color: const Color(0xFF171717), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12), side: const BorderSide(color: Color(0xFF262626)))),
  );

  static final light = ThemeData(
    brightness: Brightness.light,
    scaffoldBackgroundColor: Colors.white,
    colorScheme: ColorScheme.light(primary: _brand, secondary: _brand),
    appBarTheme: const AppBarTheme(backgroundColor: Colors.white, foregroundColor: Colors.black),
  );
}
