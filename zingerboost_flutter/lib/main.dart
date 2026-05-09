import 'package:flutter/material.dart';

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
    Center(child: Text('Dashboard')),
    Center(child: Text('Tweaks')),
    Center(child: Text('Services')),
    Center(child: Text('Cleaner')),
    Center(child: Text('Snapshots')),
    Center(child: Text('Debloat')),
    Center(child: Text('Software')),
    TempSett(),
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

class TempSett extends StatelessWidget {
  const TempSett({super.key});
  @override Widget build(BuildContext c) {
    final isDark = Theme.of(c).brightness == Brightness.dark;
    return ListView(padding: const EdgeInsets.all(16), children: [
      Card(color: const Color(0xFF171717), child: SwitchListTile(
        title: const Text('Dark Mode'),
        subtitle: Text(isDark ? 'Switch to light' : 'Switch to dark'),
        value: isDark,
        onChanged: (_) => ZingerBoostApp.of(c)?.toggleTheme(),
        secondary: Icon(isDark ? Icons.dark_mode : Icons.light_mode),
      )),
    ]);
  }
}
