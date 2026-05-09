import 'package:flutter/material.dart';
import 'dash_page.dart';
import 'tweaks_page.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const ZingerBoostApp());
}

class ZingerBoostApp extends StatefulWidget {
  const ZingerBoostApp({super.key});
  static _ZingerBoostAppState? of(BuildContext ctx) =>
      ctx.findAncestorStateOfType<_ZingerBoostAppState>();
  @override
  State<ZingerBoostApp> createState() => _ZingerBoostAppState();
}

class _ZingerBoostAppState extends State<ZingerBoostApp> {
  ThemeMode _theme = ThemeMode.dark;
  void toggleTheme() =>
      setState(() => _theme = _theme == ThemeMode.dark ? ThemeMode.light : ThemeMode.dark);
  @override
  Widget build(BuildContext c) => MaterialApp(
    title: 'ZingerBoost',
    debugShowCheckedModeBanner: false,
    theme: ThemeData.light(useMaterial3: true),
    darkTheme: ThemeData.dark(useMaterial3: true).copyWith(
      scaffoldBackgroundColor: const Color(0xFF0A0A0A),
      cardColor: const Color(0xFF171717),
    ),
    themeMode: _theme,
    home: const Shell(),
  );
}

const _titles = ['Dashboard','Tweaks','Services','Cleaner','Snapshots','Debloat','Software','Settings'];
const _icons = [
  Icons.dashboard, Icons.tune, Icons.settings, Icons.cleaning_services,
  Icons.history, Icons.delete_forever, Icons.download, Icons.palette,
];

class Shell extends StatefulWidget {
  const Shell({super.key});
  @override State<Shell> createState() => _ShellState();
}

class _ShellState extends State<Shell> {
  int _idx = 0;
  final _pages = const [
    DashPage(),
    TweaksPage(),
    Center(child: Text('Services')),
    Center(child: Text('Cleaner')),
    Center(child: Text('Snapshots')),
    Center(child: Text('Debloat')),
    Center(child: Text('Software')),
    SettPage(),
  ];
  @override
  Widget build(BuildContext c) {
    return Scaffold(
      body: Row(children: [
        NavigationRail(
          selectedIndex: _idx,
          onDestinationSelected: (v) => setState(() => _idx = v),
          labelType: NavigationRailLabelType.all,
          backgroundColor: Theme.of(c).brightness == Brightness.dark
              ? const Color(0xFF171717) : Colors.grey.shade100,
          selectedIconTheme: const IconThemeData(color: Color(0xFF0EA5E9)),
          destinations: List.generate(8, (x) =>
            NavigationRailDestination(icon: Icon(_icons[x]), label: Text(_titles[x]))),
        ),
        const VerticalDivider(width: 1),
        Expanded(
          child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
            Padding(
              padding: const EdgeInsets.all(16),
              child: Text(_titles[_idx],
                  style: const TextStyle(fontSize: 22, fontWeight: FontWeight.bold)),
            ),
            Expanded(child: _pages[_idx]),
          ]),
        ),
      ]),
    );
  }
}

// =============================================
// Settings
// =============================================
class SettPage extends StatelessWidget {
  const SettPage({super.key});
  @override Widget build(BuildContext c) {
    final isDark = Theme.of(c).brightness == Brightness.dark;
    return ListView(padding: const EdgeInsets.all(16), children: [
      Card(color: const Color(0xFF171717), child: SwitchListTile(
        title: const Text('Dark Mode', style: TextStyle(fontSize: 14)),
        subtitle: Text(isDark ? 'Switch to light' : 'Switch to dark', style: const TextStyle(fontSize: 12)),
        value: isDark,
        onChanged: (_) => ZingerBoostApp.of(c)?.toggleTheme(),
        secondary: Icon(isDark ? Icons.dark_mode : Icons.light_mode, color: isDark ? const Color(0xFF0EA5E9) : Colors.amber),
      )),
      const SizedBox(height: 12),
      Card(color: const Color(0xFF171717), child: Padding(
        padding: const EdgeInsets.all(16), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          const Text('Version', style: TextStyle(fontWeight: FontWeight.w600, fontSize: 14)),
          const SizedBox(height: 4),
          Container(padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
            decoration: BoxDecoration(color: const Color(0xFF0EA5E9).withOpacity(0.15), borderRadius: BorderRadius.circular(6)),
            child: const Text('v0.3.1', style: TextStyle(color: Color(0xFF0EA5E9), fontSize: 12, fontWeight: FontWeight.w500))),
        ]),
      )),
      const SizedBox(height: 12),
      Card(color: const Color(0xFF171717), child: Padding(
        padding: const EdgeInsets.all(16), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          const Text('About', style: TextStyle(fontWeight: FontWeight.w600, fontSize: 14)),
          const SizedBox(height: 6),
          const Text('29 tweaks, 19 services, 9 cleaner categories, 34 debloat targets.', style: TextStyle(fontSize: 12, color: Colors.grey)),
          const Text('Author: YousefMohiey | MIT License', style: TextStyle(fontSize: 11, color: Colors.grey)),
        ]),
      )),
      const SizedBox(height: 16),
      OutlinedButton.icon(
        onPressed: () => showDialog(context: c, builder: (ctx) => AlertDialog(
          title: const Text('Reset All Tweaks?'),
          content: const Text('This reverts every tweak.'),
          actions: [
            TextButton(child: const Text('Cancel'), onPressed: () => Navigator.pop(ctx)),
            TextButton(child: const Text('Reset', style: TextStyle(color: Colors.red)),
              onPressed: () { Navigator.pop(ctx); ScaffoldMessenger.of(c).showSnackBar(const SnackBar(content: Text('All tweaks reset'))); }),
          ],
        )),
        icon: const Icon(Icons.refresh, size: 16),
        label: const Text('Reset All Tweaks', style: TextStyle(fontSize: 12)),
        style: OutlinedButton.styleFrom(foregroundColor: Colors.red, side: const BorderSide(color: Colors.red), padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10)),
      ),
    ]);
  }
}
