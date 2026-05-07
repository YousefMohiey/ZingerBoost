import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:window_manager/window_manager.dart';
import 'app.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  await windowManager.ensureInitialized();

  final options = WindowOptions(
    size: const Size(1200, 800),
    minimumSize: const Size(900, 600),
    center: true,
    title: 'ZingerBoost',
    skipTaskbar: false,
  );

  await windowManager.waitUntilReadyToShow(options, () async {
    await windowManager.show();
    await windowManager.focus();
  });

  runApp(const ProviderScope(child: ZingerBoostApp()));
}
