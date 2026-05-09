import 'dart:async';
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

const _pages = [Dash(), Tweaks(), Services(), Cleaner(), Snaps(), Debloat(), Software(), Sett()];
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
          selectedLabelTextStyle: const TextStyle(color: Color(0xFF0EA5E9)),
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
// Dashboard
// =============================================
class Dash extends StatefulWidget {
  const Dash({super.key});
  @override State<Dash> createState() => _DashState();
}
class _DashState extends State<Dash> {
  Timer? _timer;
  @override void initState() { super.initState(); _timer = Timer.periodic(const Duration(seconds: 2), (_) => mounted ? setState(() {}) : null); }
  @override void dispose() { _timer?.cancel(); super.dispose(); }
  final _recs = [
    'Disable Transparency Effects — reduce GPU load',
    'Disable Game DVR — free CPU while gaming',
    'Show File Extensions — security best practice',
  ];
  @override Widget build(BuildContext c) {
    final mc = (IconData ic, String l, String v, String s, Color clr) => Card(
      color: const Color(0xFF171717),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12), side: const BorderSide(color: Color(0xFF262626))),
      child: Padding(padding: const EdgeInsets.all(14), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
        Container(padding: const EdgeInsets.all(6), decoration: BoxDecoration(color: clr.withOpacity(0.15), borderRadius: BorderRadius.circular(8)), child: Icon(ic, color: clr, size: 20)),
        const Spacer(),
        Text(l, style: TextStyle(color: Colors.grey.shade400, fontSize: 12)),
        const SizedBox(height: 4),
        Text(v, style: const TextStyle(fontSize: 22, fontWeight: FontWeight.bold)),
        if (s.isNotEmpty) Text(s, style: TextStyle(color: Colors.grey.shade500, fontSize: 11)),
      ])),
    );
    return ListView(padding: const EdgeInsets.all(16), children: [
      GridView.count(crossAxisCount: 2, shrinkWrap: true, physics: const NeverScrollableScrollPhysics(), childAspectRatio: 1.6, mainAxisSpacing: 12, crossAxisSpacing: 12, children: [
        mc(Icons.computer, 'CPU Usage', '15%', '', const Color(0xFF0EA5E9)),
        mc(Icons.memory, 'RAM Usage', '42%', '6.9 / 16.3 GB', const Color(0xFF10B981)),
        mc(Icons.storage, 'Disk Active', '5%', '', const Color(0xFFF59E0B)),
        mc(Icons.wifi, 'Network', '0.5 Mbps', '↑ 0.1 Mbps', const Color(0xFF8B5CF6)),
      ]),
      const SizedBox(height: 20),
      Card(color: const Color(0xFF171717), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12), side: const BorderSide(color: Color(0xFF262626))),
        child: Padding(padding: const EdgeInsets.all(16), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Row(children: [
            Container(padding: const EdgeInsets.all(4), decoration: BoxDecoration(color: const Color(0xFF10B981).withOpacity(0.15), borderRadius: BorderRadius.circular(6)),
              child: const Icon(Icons.tips_and_updates, color: Color(0xFF10B981), size: 16)),
            const SizedBox(width: 8),
            const Text('Recommended Actions', style: TextStyle(fontWeight: FontWeight.w600, fontSize: 14)),
          ]),
          const SizedBox(height: 10),
          for (final e in _recs)
            Padding(padding: const EdgeInsets.only(bottom: 6), child: Row(children: [
              Icon(Icons.check_circle_outline, color: const Color(0xFF10B981), size: 14),
              const SizedBox(width: 8),
              Expanded(child: Text(e, style: const TextStyle(fontSize: 12))),
            ])),
        ]))),
    ]);
  }
}

// =============================================
// Tweaks
// =============================================
class Tweaks extends StatefulWidget {
  const Tweaks({super.key});
  @override State<Tweaks> createState() => _TweaksState();
}
class _TweaksState extends State<Tweaks> {
  String _filter = 'All', _search = '';
  static const _items = [
    _T('Disable Transparency Effects','Turns off acrylic effects reducing GPU load','Visual','Safe'),
    _T('Disable Game DVR','Stops background recording freeing CPU/GPU','Gaming','Safe'),
    _T('Show File Extensions','Always show extensions in Explorer','Visual','Safe'),
    _T('Disable Telemetry','Sets diagnostic data to minimum','Privacy','Safe'),
    _T('Disable Startup Delay','Removes 10 second startup delay','Performance','Safe'),
    _T('Disable Sticky Keys Popup','No more Shift x5 interruptions','Visual','Safe'),
    _T('Disable Background Apps','Stops UWP background processes','Privacy','Safe'),
    _T('High Performance Power Plan','Prevent CPU downclocking','Performance','Safe'),
    _T('Disable Menu Delay','Instant menu popup','Visual','Safe'),
    _T('Disable Aero Shake','Stop window shake minimize','Visual','Safe'),
    _T('Disable Lock Screen Ads','Removes lock screen ads','Privacy','Safe'),
    _T('Disable Advertising ID','Turns off ad tracking','Privacy','Safe'),
  ];
  static const _cats = ['All','Visual','Privacy','Performance','Gaming'];

  @override Widget build(BuildContext c) {
    final filtered = _items.where((t) =>
      (_filter == 'All' || t.cat == _filter) &&
      (_search.isEmpty || t.name.toLowerCase().contains(_search.toLowerCase()) || t.desc.toLowerCase().contains(_search.toLowerCase()))
    ).toList();
    return Column(children: [
      Padding(padding: const EdgeInsets.fromLTRB(16, 0, 16, 8), child: TextField(
        decoration: InputDecoration(
          hintText: 'Search tweaks...', prefixIcon: const Icon(Icons.search),
          filled: true, fillColor: const Color(0xFF171717),
          border: OutlineInputBorder(borderRadius: BorderRadius.circular(10), borderSide: const BorderSide(color: Color(0xFF262626))),
          enabledBorder: OutlineInputBorder(borderRadius: BorderRadius.circular(10), borderSide: const BorderSide(color: Color(0xFF262626))),
        ),
        style: const TextStyle(fontSize: 13),
        onChanged: (v) => setState(() => _search = v),
      )),
      SizedBox(height: 36, child: ListView(scrollDirection: Axis.horizontal, padding: const EdgeInsets.symmetric(horizontal: 12), children: [
        ..._cats.map((c) => Padding(padding: const EdgeInsets.only(right: 6), child: ChoiceChip(
          label: Text(c, style: const TextStyle(fontSize: 12)),
          selected: _filter == c,
          onSelected: (_) => setState(() => _filter = c),
          selectedColor: const Color(0xFF0EA5E9), backgroundColor: const Color(0xFF171717),
          labelStyle: TextStyle(color: _filter == c ? Colors.white : Colors.grey),
          side: const BorderSide(color: Color(0xFF262626)),
        ))),
      ])),
      Expanded(child: ListView.builder(padding: const EdgeInsets.all(12), itemCount: filtered.length, itemBuilder: (_, i) {
        final t = filtered[i];
        final rc = t.risk == 'Safe' ? const Color(0xFF10B981) : t.risk == 'Moderate' ? const Color(0xFFF59E0B) : const Color(0xFFEF4444);
        return Card(color: const Color(0xFF171717), margin: const EdgeInsets.only(bottom: 8),
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10), side: const BorderSide(color: Color(0xFF262626))),
          child: Padding(padding: const EdgeInsets.all(14), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
            Row(children: [
              Expanded(child: Text(t.name, style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 14))),
              Container(padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                decoration: BoxDecoration(color: rc.withOpacity(0.15), borderRadius: BorderRadius.circular(20)),
                child: Text(t.risk, style: TextStyle(color: rc, fontSize: 11, fontWeight: FontWeight.w500))),
            ]),
            const SizedBox(height: 4),
            Text(t.desc, style: TextStyle(color: Colors.grey.shade400, fontSize: 12)),
            const SizedBox(height: 10),
            Row(mainAxisAlignment: MainAxisAlignment.end, children: [
              OutlinedButton(onPressed: () => ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('${t.name} applied!'))),
                style: OutlinedButton.styleFrom(foregroundColor: const Color(0xFF0EA5E9), side: const BorderSide(color: Color(0xFF0EA5E9)),
                  padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 6), textStyle: const TextStyle(fontSize: 12)),
                child: const Text('Apply')),
              const SizedBox(width: 8),
              TextButton(onPressed: () => ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('${t.name} reverted!'))),
                style: TextButton.styleFrom(foregroundColor: Colors.grey, padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 6), textStyle: const TextStyle(fontSize: 12)),
                child: const Text('Revert')),
            ]),
          ])));
      })),
    ]);
  }
}
class _T { final String name, desc, cat, risk; const _T(this.name, this.desc, this.cat, this.risk); }

// =============================================
// Services
// =============================================
class Services extends StatelessWidget {
  const Services({super.key});
  static const _s = [
    _S('DiagTrack','Connected User Experiences and Telemetry',true),
    _S('SysMain','SysMain / Superfetch',true),
    _S('WSearch','Windows Search',true),
    _S('dmwappushservice','Device Management WAP Push',true),
    _S('Fax','Fax Service',false),
    _S('WerSvc','Windows Error Reporting',true),
    _S('MapsBroker','Downloaded Maps Manager',false),
    _S('XboxNetApiSvc','Xbox Live Networking',false),
    _S('WpnService','Windows Push Notifications',true),
    _S('PcaSvc','Program Compatibility Assistant',true),
    _S('FontCache','Windows Font Cache',true),
    _S('RemoteRegistry','Remote Registry',false),
  ];
  @override Widget build(BuildContext c) {
    return ListView(padding: const EdgeInsets.all(12), children: [
      Container(padding: const EdgeInsets.all(12),
        decoration: BoxDecoration(color: const Color(0xFFF59E0B).withOpacity(0.08), borderRadius: BorderRadius.circular(10), border: Border.all(color: const Color(0xFFF59E0B).withOpacity(0.2))),
        child: const Row(children: [Icon(Icons.info_outline, color: Color(0xFFF59E0B), size: 16), SizedBox(width: 8), Expanded(child: Text('These services are safe to disable', style: TextStyle(color: Color(0xFFF59E0B), fontSize: 12)))])),
      const SizedBox(height: 12),
      ..._s.map((s) => Card(color: const Color(0xFF171717), margin: const EdgeInsets.only(bottom: 8),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10), side: const BorderSide(color: Color(0xFF262626))),
        child: Padding(padding: const EdgeInsets.all(14), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Row(children: [
            Expanded(child: Text(s.name, style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 14))),
            Container(padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
              decoration: BoxDecoration(color: (s.running ? const Color(0xFF10B981) : Colors.grey).withOpacity(0.15), borderRadius: BorderRadius.circular(20)),
              child: Text(s.running ? 'Running' : 'Stopped', style: TextStyle(color: s.running ? const Color(0xFF10B981) : Colors.grey, fontSize: 11, fontWeight: FontWeight.w500))),
          ]),
          const SizedBox(height: 4),
          Text(s.desc, style: TextStyle(color: Colors.grey.shade400, fontSize: 12)),
          const SizedBox(height: 10),
          Row(mainAxisAlignment: MainAxisAlignment.end, children: [
            TextButton(onPressed: () => ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('Stopping ${s.name}...'))),
              style: TextButton.styleFrom(foregroundColor: const Color(0xFFF59E0B), textStyle: const TextStyle(fontSize: 12)), child: const Text('Stop')),
            const SizedBox(width: 8),
            TextButton(onPressed: () => ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('Disabling ${s.name}...'))),
              style: TextButton.styleFrom(foregroundColor: Colors.red, textStyle: const TextStyle(fontSize: 12)), child: const Text('Disable')),
          ]),
        ])))),
    ]);
  }
}
class _S { final String name, desc; final bool running; const _S(this.name, this.desc, this.running); }

// =============================================
// Cleaner
// =============================================
class Cleaner extends StatelessWidget {
  const Cleaner({super.key});
  static const _cs = [
    _C('Recycle Bin', 'Files in Recycle Bin', 'Safe', 245.3, Icons.delete),
    _C('Temporary Files', 'User temp files %TEMP%', 'Safe', 512.1, Icons.folder),
    _C('Browser Cache', 'Chrome, Edge, Firefox cache', 'Safe', 389.7, Icons.public),
    _C('Windows Temp', 'System temp files', 'Safe', 156.2, Icons.computer),
    _C('DNS Cache', 'Flush DNS resolver cache', 'Safe', 0.0, Icons.dns),
    _C('Thumbnail Cache', 'Explorer thumbnail cache', 'Safe', 98.3, Icons.image),
    _C('Windows Logs', 'System and application logs', 'Moderate', 723.5, Icons.description),
    _C('Update Cache', 'Old Windows Update files', 'Moderate', 1234.0, Icons.update),
    _C('Prefetch Data', 'Windows prefetch files', 'Moderate', 89.1, Icons.speed),
  ];
  @override Widget build(BuildContext c) {
    return Column(children: [
      Padding(padding: const EdgeInsets.fromLTRB(12, 0, 12, 8), child: Row(children: [
        Expanded(child: ElevatedButton.icon(onPressed: () => ScaffoldMessenger.of(c).showSnackBar(const SnackBar(content: Text('Scanning...'))),
          icon: const Icon(Icons.search, size: 16), label: const Text('Scan'),
          style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFF0EA5E9), foregroundColor: Colors.white))),
        const SizedBox(width: 8),
        Expanded(child: ElevatedButton.icon(onPressed: () => ScaffoldMessenger.of(c).showSnackBar(const SnackBar(content: Text('Cleaning all safe items...'))),
          icon: const Icon(Icons.cleaning_services, size: 16), label: const Text('Clean All Safe'),
          style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFF10B981), foregroundColor: Colors.white))),
      ])),
      Expanded(child: ListView.builder(padding: const EdgeInsets.all(12), itemCount: _cs.length, itemBuilder: (_, i) {
        final x = _cs[i];
        final rc = x.risk == 'Safe' ? const Color(0xFF10B981) : const Color(0xFFF59E0B);
        return Card(color: const Color(0xFF171717), margin: const EdgeInsets.only(bottom: 8),
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10), side: const BorderSide(color: Color(0xFF262626))),
          child: Padding(padding: const EdgeInsets.all(14), child: Row(children: [
            Container(padding: const EdgeInsets.all(8), decoration: BoxDecoration(color: rc.withOpacity(0.1), borderRadius: BorderRadius.circular(8)), child: Icon(x.icon, color: rc, size: 20)),
            const SizedBox(width: 12),
            Expanded(child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              Row(children: [
                Text(x.name, style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 13)),
                const SizedBox(width: 6),
                Container(padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 1), decoration: BoxDecoration(color: rc.withOpacity(0.15), borderRadius: BorderRadius.circular(20)),
                  child: Text(x.risk, style: TextStyle(color: rc, fontSize: 10, fontWeight: FontWeight.w500))),
              ]),
              const SizedBox(height: 2),
              Text(x.desc, style: TextStyle(color: Colors.grey.shade400, fontSize: 11)),
            ])),
            Column(children: [
              Text('${x.sizeMB > 0 ? x.sizeMB.toStringAsFixed(1) : "< 1"} MB', style: TextStyle(color: Colors.grey.shade400, fontSize: 12, fontWeight: FontWeight.w500)),
              const SizedBox(height: 4),
              SizedBox(height: 26, child: ElevatedButton(onPressed: () => ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('Cleaning ${x.name}...'))),
                style: ElevatedButton.styleFrom(backgroundColor: rc, foregroundColor: Colors.white, padding: const EdgeInsets.symmetric(horizontal: 10), textStyle: const TextStyle(fontSize: 11)),
                child: const Text('Clean'))),
            ]),
          ])));
      })),
    ]);
  }
}
class _C { final String name, desc, risk; final double sizeMB; final IconData icon; const _C(this.name, this.desc, this.risk, this.sizeMB, this.icon); }

// =============================================
// Snapshots
// =============================================
class Snaps extends StatefulWidget {
  const Snaps({super.key});
  @override State<Snaps> createState() => _SnapsState();
}
class _SnapsState extends State<Snaps> {
  static const _shots = [
    _Shot('Batch apply 3 tweaks', '2026-05-09 14:30', 3),
    _Shot('Applied tweak visual_disable_transparency', '2026-05-09 12:15', 1),
    _Shot('Applied tweak gaming_disable_dvr', '2026-05-08 18:45', 1),
  ];
  void _restore(int i, BuildContext c) {
    showDialog(context: c, builder: (ctx) => AlertDialog(
      title: const Text('Restore Snapshot?'),
      content: Text('Revert to snapshot from ${_shots[i].date}?'),
      actions: [
        TextButton(child: const Text('Cancel'), onPressed: () => Navigator.pop(ctx)),
        TextButton(child: const Text('Restore', style: TextStyle(color: Colors.amber)), onPressed: () {
          Navigator.pop(ctx);
          ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('Restored snapshot from ${_shots[i].date}')));
        }),
      ],
    ));
  }
  @override Widget build(BuildContext c) {
    return ListView(padding: const EdgeInsets.all(12), children: [
      if (_shots.isEmpty)
        Center(child: Column(mainAxisSize: MainAxisSize.min, children: [
          const SizedBox(height: 80),
          Icon(Icons.history, size: 48, color: Colors.grey.shade700),
          const SizedBox(height: 12),
          Text('No snapshots yet', style: TextStyle(color: Colors.grey.shade400, fontSize: 16)),
          const SizedBox(height: 4),
          Text('Snapshots are created when you apply tweaks', style: TextStyle(color: Colors.grey.shade600, fontSize: 12)),
        ]))
      else
        ..._shots.asMap().entries.map((e) => Card(color: const Color(0xFF171717), margin: const EdgeInsets.only(bottom: 8),
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10), side: const BorderSide(color: Color(0xFF262626))),
          child: Padding(padding: const EdgeInsets.all(14), child: Row(children: [
            Container(padding: const EdgeInsets.all(8), decoration: BoxDecoration(color: const Color(0xFF0EA5E9).withOpacity(0.1), borderRadius: BorderRadius.circular(8)),
              child: const Icon(Icons.restore, color: Color(0xFF0EA5E9), size: 20)),
            const SizedBox(width: 12),
            Expanded(child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              Text(e.value.desc, style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 13)),
              Text(e.value.date, style: TextStyle(color: Colors.grey.shade400, fontSize: 11)),
            ])),
            Column(children: [
              Text('${e.value.count} tweak${e.value.count != 1 ? "s" : ""}', style: TextStyle(color: Colors.grey.shade500, fontSize: 11)),
              const SizedBox(height: 4),
              TextButton(onPressed: () => _restore(e.key, c),
                style: TextButton.styleFrom(foregroundColor: const Color(0xFFF59E0B), padding: const EdgeInsets.symmetric(horizontal: 12), textStyle: const TextStyle(fontSize: 12)),
                child: const Text('Restore')),
            ]),
          ])))),
    ]);
  }
}
class _Shot { final String desc, date; final int count; const _Shot(this.desc, this.date, this.count); }

// =============================================
// Debloat
// =============================================
class Debloat extends StatefulWidget {
  const Debloat({super.key});
  @override State<Debloat> createState() => _DebloatState();
}
class _DebloatState extends State<Debloat> {
  static const _all = [
    'Candy Crush Saga','Microsoft Solitaire Collection','Xbox Console Companion','Bing Weather',
    'Bing News','Bing Sports','Bing Finance','Get Help','Microsoft Tips','Feedback Hub',
    'Microsoft Office Hub','Mixed Reality Portal','3D Viewer','Paint 3D','Skype',
    'Mail and Calendar','Microsoft People','Groove Music','Movies & TV','Windows Maps','OneNote',
    'Outlook for Windows','LinkedIn','Microsoft Copilot','Clipchamp','OneDrive','Quick Assist',
    'Sticky Notes','Microsoft Teams','Phone Link','Microsoft To Do','Xbox Game Bar',
    'Windows Widgets','Cortana',
  ];
  final Set<String> _selected = {};
  bool get _allSelected => _selected.length == _all.length;

  void _toggle(String s) => setState(() { if (_selected.contains(s)) _selected.remove(s); else _selected.add(s); });
  void _selectAll() => setState(() => _selected.addAll(_all));
  void _deselectAll() => setState(() => _selected.clear());

  void _remove(bool all, BuildContext c) {
    final target = all ? _all : _selected.toList();
    if (target.isEmpty) return;
    showDialog(context: c, builder: (ctx) => AlertDialog(
      title: Text(all ? 'Remove ALL Bloatware?' : 'Remove Selected?'),
      content: Text('${target.length} app${target.length != 1 ? "s" : ""} will be removed.\nCan be reinstalled from Microsoft Store.'),
      actions: [
        TextButton(child: const Text('Cancel'), onPressed: () => Navigator.pop(ctx)),
        TextButton(child: const Text('Remove', style: TextStyle(color: Colors.red)), onPressed: () {
          Navigator.pop(ctx);
          ScaffoldMessenger.of(c).showSnackBar(SnackBar(
            content: Text(all ? 'Removing all bloatware...' : 'Removing ${target.length} app${target.length != 1 ? "s" : ""}...'),
          ));
        }),
      ],
    ));
  }

  @override Widget build(BuildContext c) {
    return ListView(padding: const EdgeInsets.all(12), children: [
      Container(padding: const EdgeInsets.all(12),
        decoration: BoxDecoration(color: const Color(0xFFF59E0B).withOpacity(0.08), borderRadius: BorderRadius.circular(10), border: Border.all(color: const Color(0xFFF59E0B).withOpacity(0.2))),
        child: const Row(children: [Icon(Icons.info_outline, color: Color(0xFFF59E0B), size: 16), SizedBox(width: 8), Expanded(child: Text('Can be reinstalled from Microsoft Store', style: TextStyle(color: Color(0xFFF59E0B), fontSize: 12)))])),
      const SizedBox(height: 12),
      Row(children: [
        OutlinedButton(onPressed: _allSelected ? _deselectAll : _selectAll,
          style: OutlinedButton.styleFrom(foregroundColor: const Color(0xFF0EA5E9), textStyle: const TextStyle(fontSize: 12)),
          child: Text(_allSelected ? 'Deselect All' : 'Select All')),
        const Spacer(),
        if (_selected.isNotEmpty)
          ElevatedButton(onPressed: () => _remove(false, c),
            style: ElevatedButton.styleFrom(backgroundColor: Colors.red, foregroundColor: Colors.white, textStyle: const TextStyle(fontSize: 12)),
            child: Text('Remove (${_selected.length})')),
        const SizedBox(width: 8),
        ElevatedButton(onPressed: () => _remove(true, c),
          style: ElevatedButton.styleFrom(backgroundColor: Colors.red.shade800, foregroundColor: Colors.white, textStyle: const TextStyle(fontSize: 12)),
          child: const Text('Remove All')),
      ]),
      const SizedBox(height: 8),
      ..._all.map((a) => CheckboxListTile(
        dense: true,
        title: Text(a, style: const TextStyle(fontSize: 13)),
        value: _selected.contains(a),
        onChanged: (_) => _toggle(a),
        activeColor: const Color(0xFF0EA5E9),
        checkboxShape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(4)),
      )),
      const SizedBox(height: 16),
      Container(padding: const EdgeInsets.all(12),
        decoration: BoxDecoration(color: const Color(0xFF10B981).withOpacity(0.08), borderRadius: BorderRadius.circular(10), border: Border.all(color: const Color(0xFF10B981).withOpacity(0.2))),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Text('Protected Apps', style: TextStyle(color: Color(0xFF10B981), fontWeight: FontWeight.w600, fontSize: 13)),
          SizedBox(height: 4),
          Text('Notepad, Calculator, Store, Photos, Camera, Snipping Tool, Terminal, VCLibs, .NET Native', style: TextStyle(color: Color(0xFF10B981).withOpacity(0.7), fontSize: 11)),
        ])),
    ]);
  }
}

// =============================================
// Software
// =============================================
class Software extends StatefulWidget {
  const Software({super.key});
  @override State<Software> createState() => _SoftwareState();
}
class _SoftwareState extends State<Software> with SingleTickerProviderStateMixin {
  late TabController _tab;
  String _cat = 'All';
  @override void initState() { super.initState(); _tab = TabController(length: 2, vsync: this); }
  @override void dispose() { _tab.dispose(); super.dispose(); }
  static const _apps = [
    _A('Chrome','Fast secure browser','Browsers',Icons.language),
    _A('Brave','Built-in ad blocker','Browsers',Icons.language),
    _A('Zen Browser','Minimalist calm browser','Browsers',Icons.public),
    _A('Arc','Modern reimagined browser','Browsers',Icons.explore),
    _A('Vivaldi','Highly customizable','Browsers',Icons.web),
    _A('Microsoft Edge','Built-in browser','Browsers',Icons.open_in_browser),
    _A('VLC','Versatile media player','Media Players',Icons.play_circle),
    _A('Screenbox','Modern media player','Media Players',Icons.slideshow),
    _A('PotPlayer','Advanced media player','Media Players',Icons.video_library),
    _A('Spotify','Music streaming','Music',Icons.music_note),
    _A('Anghami','Arabic music platform','Music',Icons.headphones),
    _A('Windows Media Player','Classic player','Music',Icons.album),
    _A('Steam','PC gaming platform','Gaming',Icons.sports_esports),
    _A('Epic Games','Game store','Gaming',Icons.store),
    _A('Discord','Voice and chat','Gaming',Icons.chat),
    _A('7-Zip','File archiver','Utilities',Icons.archive),
    _A('Notepad++','Text editor','Utilities',Icons.code),
    _A('VS Code','Code editor','Development',Icons.code),
    _A('Git','Version control','Development',Icons.code),
    _A('Telegram','Messaging','Communication',Icons.chat),
  ];

  @override Widget build(BuildContext c) {
    return Column(children: [
      TabBar(controller: _tab, labelColor: const Color(0xFF0EA5E9), unselectedLabelColor: Colors.grey,
        indicatorColor: const Color(0xFF0EA5E9),
        tabs: const [Tab(text: 'Install'), Tab(text: 'Debloat')]),
      Expanded(child: TabBarView(controller: _tab, children: [
        _buildInstall(c),
        const Debloat(),
      ])),
    ]);
  }
  Widget _buildInstall(BuildContext c) {
    final cats = ['All','Browsers','Media Players','Music','Gaming','Utilities','Development','Communication'];
    final filtered = _cat == 'All' ? _apps : _apps.where((a) => a.cat == _cat).toList();
    return Column(children: [
      SizedBox(height: 36, child: ListView(scrollDirection: Axis.horizontal, padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
        children: cats.map((ct) => Padding(padding: const EdgeInsets.only(right: 6), child: ChoiceChip(
          label: Text(ct, style: const TextStyle(fontSize: 11)),
          selected: _cat == ct,
          onSelected: (_) => setState(() => _cat = ct),
          selectedColor: const Color(0xFF0EA5E9), backgroundColor: const Color(0xFF171717),
          labelStyle: TextStyle(color: _cat == ct ? Colors.white : Colors.grey),
          side: const BorderSide(color: Color(0xFF262626)),
        ))).toList(),
      )),
      Expanded(child: GridView.builder(padding: const EdgeInsets.all(12), gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(crossAxisCount: 3, mainAxisSpacing: 8, crossAxisSpacing: 8, childAspectRatio: 1.0),
        itemCount: filtered.length, itemBuilder: (_, i) {
          final a = filtered[i];
          return Card(color: const Color(0xFF171717),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10), side: const BorderSide(color: Color(0xFF262626))),
            child: Padding(padding: const EdgeInsets.all(10), child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
              Icon(a.ic, color: const Color(0xFF0EA5E9), size: 24),
              const SizedBox(height: 6),
              Text(a.name, style: const TextStyle(fontSize: 11, fontWeight: FontWeight.w600), textAlign: TextAlign.center, maxLines: 1, overflow: TextOverflow.ellipsis),
              Text(a.cat, style: TextStyle(fontSize: 9, color: Colors.grey.shade500)),
              const SizedBox(height: 4),
              SizedBox(height: 24, child: ElevatedButton(onPressed: () => ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('Installing ${a.name}...'))),
                style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFF0EA5E9), foregroundColor: Colors.white, padding: const EdgeInsets.symmetric(horizontal: 8), textStyle: const TextStyle(fontSize: 10)),
                child: const Text('Install'))),
            ])));
        }),
      )),
    ];
  }
}

class _A { final String name, cat; final IconData ic; const _A(this.name, String _, this.cat, this.ic); }

// =============================================
// Settings
// =============================================
class Sett extends StatelessWidget {
  const Sett({super.key});
  @override Widget build(BuildContext c) {
    final isDark = Theme.of(c).brightness == Brightness.dark;
    return ListView(padding: const EdgeInsets.all(16), children: [
      Card(color: const Color(0xFF171717), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        child: SwitchListTile(title: const Text('Dark Mode', style: TextStyle(fontSize: 14)),
          subtitle: Text(isDark ? 'Switch to light theme' : 'Switch to dark theme', style: const TextStyle(fontSize: 12)),
          value: isDark, onChanged: (_) => ZingerBoostApp.of(c)?.toggleTheme(),
          secondary: Icon(isDark ? Icons.dark_mode : Icons.light_mode, color: isDark ? const Color(0xFF0EA5E9) : Colors.amber))),
      const SizedBox(height: 12),
      Card(color: const Color(0xFF171717), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        child: Padding(padding: const EdgeInsets.all(16), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          const Text('Version', style: TextStyle(fontSize: 14, fontWeight: FontWeight.w600)),
          const SizedBox(height: 4),
          Container(padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
            decoration: BoxDecoration(color: const Color(0xFF0EA5E9).withOpacity(0.15), borderRadius: BorderRadius.circular(6)),
            child: const Text('v0.3.1', style: TextStyle(color: Color(0xFF0EA5E9), fontSize: 12, fontWeight: FontWeight.w500))),
        ]))),
      const SizedBox(height: 12),
      Card(color: const Color(0xFF171717), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        child: const Padding(padding: EdgeInsets.all(16), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Text('About', style: TextStyle(fontSize: 14, fontWeight: FontWeight.w600)),
          SizedBox(height: 6),
          Text('Safe, reversible Windows optimization. 29 tweaks, 19 services, 9 cleaner categories, 34 debloat targets.', style: TextStyle(fontSize: 12, color: Colors.grey)),
          SizedBox(height: 8),
          Text('Author: YousefMohiey | MIT License', style: TextStyle(fontSize: 11, color: Colors.grey)),
        ]))),
      const SizedBox(height: 16),
      OutlinedButton.icon(onPressed: () => showDialog(context: c, builder: (ctx) => AlertDialog(
        title: const Text('Reset All Tweaks?'),
        content: const Text('This will revert every tweak to its original state.'),
        actions: [
          TextButton(child: const Text('Cancel'), onPressed: () => Navigator.pop(ctx)),
          TextButton(child: const Text('Reset', style: TextStyle(color: Colors.red)), onPressed: () {
            Navigator.pop(ctx);
            ScaffoldMessenger.of(c).showSnackBar(const SnackBar(content: Text('All tweaks reset')));
          }),
        ],
      )), icon: const Icon(Icons.refresh, size: 16), label: const Text('Reset All Tweaks', style: TextStyle(fontSize: 12)),
        style: OutlinedButton.styleFrom(foregroundColor: Colors.red, side: const BorderSide(color: Colors.red), padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10))),
    ]);
  }
}
