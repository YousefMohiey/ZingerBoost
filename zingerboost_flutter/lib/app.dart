import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'theme/app_theme.dart';
import 'theme/theme_provider.dart';
import 'widgets/app_sidebar.dart';
import 'pages/dashboard_page.dart';
import 'pages/debloat_page.dart';
import 'pages/maintenance_page.dart';
import 'pages/settings_page.dart';
import 'pages/snapshots_page.dart';
import 'pages/software_page.dart';
import 'pages/tweaks_page.dart';
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
    try {
      RustBridge.init();
    } catch (_) {}
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
      routes: {
        '/debloat': (_) => const DebloatPage(),
      },
      home: const MainShell(),
    );
  }
}

class MainShell extends StatefulWidget {
  const MainShell({super.key});

  @override
  State<MainShell> createState() => _MainShellState();
}

class _MainShellState extends State<MainShell> {
  int _currentIndex = 0;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Row(
        children: [
          AppSidebar(
            selectedIndex: _currentIndex,
            onSelect: (i) => setState(() => _currentIndex = i),
          ),
          Expanded(child: _buildPage()),
        ],
      ),
    );
  }

  Widget _buildPage() {
    switch (_currentIndex) {
      case 0:
        return const DashboardPage();
      case 1:
        return const TweaksPage();
      case 2:
        return const SnapshotsPage();
      case 3:
        return const DebloatPage();
      case 4:
        return const SoftwarePage();
      case 5:
      case 6:
        return const MaintenancePage();
      case 7:
        return const SettingsPage();
      default:
        return const DashboardPage();
    }
  }
}
