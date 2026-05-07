import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'theme/app_theme.dart';
import 'theme/theme_provider.dart';
import 'widgets/app_sidebar.dart';
import 'pages/dashboard_page.dart';
import 'pages/tweaks_page.dart';
import 'pages/snapshots_page.dart';
import 'pages/debloat_page.dart';
import 'pages/software_page.dart';
import 'pages/settings_page.dart';
import 'services/rust_bridge.dart';

class ZingerBoostApp extends ConsumerStatefulWidget {
  const ZingerBoostApp({super.key});

  @override
  ConsumerState<ZingerBoostApp> createState() => _ZingerBoostAppState();
}

class _ZingerBoostAppState extends ConsumerState<ZingerBoostApp> {
  @override
  void initState() {
    super.initState();
    RustBridge.init();
  }

  @override
  Widget build(BuildContext context) {
    final themeMode = ref.watch(themeModeProvider);
    return MaterialApp(
      title: 'ZingerBoost',
      debugShowCheckedModeBanner: false,
      theme: AppTheme.light,
      darkTheme: AppTheme.dark,
      themeMode: themeMode,
      home: const MainShell(),
    );
  }
}

class MainShell extends ConsumerWidget {
  const MainShell({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      body: Row(
        children: [
          const AppSidebar(),
          Expanded(
            child: Navigator(
              onPopPage: (route, result) => route.didPop(result),
              pages: const [
                MaterialPage(child: DashboardPage()),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
