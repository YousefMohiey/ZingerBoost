import 'package:flutter/material.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const ZingerBoostApp());
}

class ZingerBoostApp extends StatefulWidget {
  const ZingerBoostApp({super.key});

  static _ZingerBoostAppState? of(BuildContext context) =>
      context.findAncestorStateOfType<_ZingerBoostAppState>();

  @override
  State<ZingerBoostApp> createState() => _ZingerBoostAppState();
}

class _ZingerBoostAppState extends State<ZingerBoostApp> {
  ThemeMode _themeMode = ThemeMode.dark;

  void toggleTheme() {
    setState(() {
      _themeMode =
          _themeMode == ThemeMode.dark ? ThemeMode.light : ThemeMode.dark;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'ZingerBoost',
      debugShowCheckedModeBanner: false,
      theme: ThemeData.light(useMaterial3: true).copyWith(
        colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xFF0EA5E9)),
      ),
      darkTheme: ThemeData.dark(useMaterial3: true).copyWith(
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color(0xFF0EA5E9),
          brightness: Brightness.dark,
        ),
        scaffoldBackgroundColor: const Color(0xFF0A0A0A),
        cardColor: const Color(0xFF171717),
      ),
      themeMode: _themeMode,
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
  int _index = 0;

  static const _pages = <Widget>[
    DashboardPage(),
    TweaksPage(),
    ServicesPage(),
    CleanerPage(),
    SnapshotsPage(),
    DebloatPage(),
    SoftwarePage(),
    SettingsPage(),
  ];

  static const _titles = [
    'Dashboard', 'Tweaks', 'Services', 'Cleaner',
    'Snapshots', 'Debloat', 'Software', 'Settings',
  ];

  static const _icons = [
    Icons.dashboard, Icons.tune, Icons.settings, Icons.cleaning_services,
    Icons.history, Icons.delete_forever, Icons.download, Icons.palette,
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Row(
        children: [
          NavigationRail(
            selectedIndex: _index,
            onDestinationSelected: (i) => setState(() => _index = i),
            labelType: NavigationRailLabelType.all,
            backgroundColor: Theme.of(context).brightness == Brightness.dark
                ? const Color(0xFF171717)
                : Colors.grey.shade100,
            selectedIconTheme: const IconThemeData(color: Color(0xFF0EA5E9)),
            selectedLabelTextStyle: const TextStyle(color: Color(0xFF0EA5E9)),
            destinations: List.generate(
              _pages.length,
              (i) => NavigationRailDestination(
                icon: Icon(_icons[i]),
                label: Text(_titles[i]),
              ),
            ),
          ),
          const VerticalDivider(width: 1),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Padding(
                  padding: const EdgeInsets.all(16),
                  child: Text(
                    _titles[_index],
                    style: const TextStyle(
                      fontSize: 22,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
                Expanded(child: _pages[_index]),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class DashboardPage extends StatelessWidget { const DashboardPage({super.key}); @override Widget build(BuildContext context) => const Center(child: Text('Dashboard — loading...')); }
class TweaksPage extends StatelessWidget { const TweaksPage({super.key}); @override Widget build(BuildContext context) => const Center(child: Text('Tweaks — loading...')); }
class ServicesPage extends StatelessWidget { const ServicesPage({super.key}); @override Widget build(BuildContext context) => const Center(child: Text('Services — loading...')); }
class CleanerPage extends StatelessWidget { const CleanerPage({super.key}); @override Widget build(BuildContext context) => const Center(child: Text('Cleaner — loading...')); }

class SnapshotsPage extends StatefulWidget {
  const SnapshotsPage({super.key});

  @override
  State<SnapshotsPage> createState() => _SnapshotsPageState();
}

class _SnapshotsPageState extends State<SnapshotsPage> {
  static const _card = Color(0xFF171717);
  static const _border = Color(0xFF262626);
  static const _brand = Color(0xFF0EA5E9);

  final _snapshots = [
    _Snapshot('2026-05-09 14:30', 'Batch apply: 3 tweaks', 3),
    _Snapshot('2026-05-09 12:15', 'Applied tweak: visual_disable_transparency', 1),
    _Snapshot('2026-05-08 18:45', 'Applied tweak: gaming_disable_dvr', 1),
  ];

  void _restoreSnapshot(BuildContext context, _Snapshot snap) {
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: _card,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: _border),
        ),
        title: const Text('Confirm Restore'),
        content: Text('Restore snapshot from ${snap.dateTime}?\n\n${snap.description}'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              Navigator.pop(ctx);
              if (mounted) {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(content: Text('Restoring snapshot: ${snap.description}')),
                );
              }
            },
            child: const Text('Restore'),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    if (_snapshots.isEmpty) {
      return Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.history, size: 64, color: Colors.grey.shade600),
            const SizedBox(height: 16),
            Text(
              'No snapshots yet',
              style: TextStyle(fontSize: 18, color: Colors.grey.shade500),
            ),
          ],
        ),
      );
    }

    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: _snapshots.length,
      itemBuilder: (context, index) {
        final snap = _snapshots[index];
        return Card(
          color: _card,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
            side: const BorderSide(color: _border),
          ),
          margin: const EdgeInsets.only(bottom: 12),
          child: Padding(
            padding: const EdgeInsets.all(16),
            child: Row(
              children: [
                Container(
                  width: 40,
                  height: 40,
                  decoration: BoxDecoration(
                    color: _brand.withOpacity(0.1),
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: const Icon(Icons.restore, color: _brand),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        snap.dateTime,
                        style: const TextStyle(
                          fontSize: 14,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                      const SizedBox(height: 4),
                      Text(
                        snap.description,
                        style: TextStyle(
                          fontSize: 13,
                          color: Colors.grey.shade400,
                        ),
                      ),
                      const SizedBox(height: 4),
                      Text(
                        '${snap.records} tweak records',
                        style: TextStyle(
                          fontSize: 12,
                          color: Colors.grey.shade500,
                        ),
                      ),
                    ],
                  ),
                ),
                OutlinedButton(
                  onPressed: () => _restoreSnapshot(context, snap),
                  style: OutlinedButton.styleFrom(
                    foregroundColor: _brand,
                    side: const BorderSide(color: _brand),
                    padding: const EdgeInsets.symmetric(
                      horizontal: 16,
                      vertical: 8,
                    ),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(8),
                    ),
                  ),
                  child: const Text('Restore'),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}

class _Snapshot {
  final String dateTime;
  final String description;
  final int records;

  const _Snapshot(this.dateTime, this.description, this.records);
}

class DebloatPage extends StatefulWidget {
  const DebloatPage({super.key});

  @override
  State<DebloatPage> createState() => _DebloatPageState();
}

class _DebloatPageState extends State<DebloatPage> {
  static const _card = Color(0xFF171717);
  static const _border = Color(0xFF262626);
  static const _brand = Color(0xFF0EA5E9);
  static const _red = Color(0xFFEF4444);
  static const _amber = Color(0xFFF59E0B);
  static const _green = Color(0xFF10B981);

  final _apps = [
    _BloatApp('Candy Crush Saga', 'King puzzle game pre-installed with Windows'),
    _BloatApp('Microsoft Solitaire Collection', 'Card games collection'),
    _BloatApp('Xbox Console Companion', 'Xbox integration and streaming app'),
    _BloatApp('Bing Weather', 'Weather forecast application'),
    _BloatApp('Bing News', 'News aggregation application'),
    _BloatApp('Bing Sports', 'Sports news and scores'),
    _BloatApp('Bing Finance', 'Financial news and stock tracker'),
    _BloatApp('Get Help', 'Windows built-in help system'),
    _BloatApp('Microsoft Tips', 'Windows tips and tricks app'),
    _BloatApp('Feedback Hub', 'Windows feedback submission tool'),
    _BloatApp('Microsoft Office Hub', 'Office applications launcher hub'),
    _BloatApp('Mixed Reality Portal', 'Virtual and augmented reality portal'),
    _BloatApp('3D Viewer', '3D model viewing application'),
    _BloatApp('Paint 3D', '3D painting and modeling application'),
    _BloatApp('Skype', 'Video calling and messaging application'),
    _BloatApp('Mail and Calendar', 'Built-in email and calendar client'),
    _BloatApp('Microsoft People', 'Contact management application'),
    _BloatApp('Groove Music', 'Music player application'),
    _BloatApp('Movies & TV', 'Video player and media library'),
    _BloatApp('Windows Maps', 'Map and navigation application'),
    _BloatApp('OneNote', 'Note-taking application'),
    _BloatApp('Outlook for Windows', 'Email and calendar client'),
    _BloatApp('LinkedIn', 'Professional networking application'),
    _BloatApp('Microsoft Copilot', 'AI assistant integration'),
    _BloatApp('Clipchamp', 'Video editing application'),
    _BloatApp('OneDrive', 'Cloud storage sync client'),
    _BloatApp('Quick Assist', 'Remote assistance tool'),
    _BloatApp('Sticky Notes', 'Desktop note-taking tool'),
    _BloatApp('Microsoft Teams', 'Team collaboration platform'),
    _BloatApp('Phone Link', 'Phone-to-PC integration app'),
    _BloatApp('Microsoft To Do', 'Task management application'),
    _BloatApp('Xbox Game Bar', 'Gaming overlay and capture tool'),
    _BloatApp('Windows Widgets', 'Desktop widgets system'),
    _BloatApp('Cortana', 'Virtual assistant application'),
  ];

  final _selected = <_BloatApp>{};

  bool get _allSelected => _selected.length == _apps.length;

  void _toggleAll(bool select) {
    setState(() {
      if (select) {
        _selected.addAll(_apps);
      } else {
        _selected.clear();
      }
    });
  }

  void _toggleApp(_BloatApp app) {
    setState(() {
      if (_selected.contains(app)) {
        _selected.remove(app);
      } else {
        _selected.add(app);
      }
    });
  }

  void _confirmRemove({bool all = false}) {
    final target = all ? _apps : _selected.toList();
    if (target.isEmpty) return;

    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: _card,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: _border),
        ),
        title: const Text('Confirm Removal'),
        content: Text(
          'Remove ${target.length} app(s)? '
          'These apps can be reinstalled from the Microsoft Store.',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              Navigator.pop(ctx);
              if (mounted) {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(
                    content: Text('Removing ${target.length} bloatware app(s)...'),
                  ),
                );
              }
            },
            style: TextButton.styleFrom(foregroundColor: _red),
            child: const Text('Remove'),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Container(
          margin: const EdgeInsets.fromLTRB(16, 16, 16, 0),
          padding: const EdgeInsets.all(12),
          decoration: BoxDecoration(
            color: _card,
            borderRadius: BorderRadius.circular(8),
            border: Border.all(color: _border),
          ),
          child: Row(
            children: [
              const Icon(Icons.info_outline, color: _brand, size: 20),
              const SizedBox(width: 10),
              const Expanded(
                child: Text(
                  'These changes are reversible — apps can be reinstalled from Microsoft Store',
                  style: TextStyle(fontSize: 13),
                ),
              ),
            ],
          ),
        ),
        Padding(
          padding: const EdgeInsets.fromLTRB(16, 12, 16, 0),
          child: SizedBox(
            width: double.infinity,
            child: ElevatedButton.icon(
              onPressed: () => _confirmRemove(all: true),
              style: ElevatedButton.styleFrom(
                backgroundColor: _red,
                foregroundColor: Colors.white,
                padding: const EdgeInsets.symmetric(vertical: 14),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(8),
                ),
              ),
              icon: const Icon(Icons.delete_forever),
              label: const Text('Remove All Bloatware'),
            ),
          ),
        ),
        Padding(
          padding: const EdgeInsets.fromLTRB(16, 12, 16, 4),
          child: Row(
            children: [
              TextButton.icon(
                onPressed: () => _toggleAll(true),
                icon: const Icon(Icons.select_all, size: 18),
                label: const Text('Select All'),
              ),
              const SizedBox(width: 4),
              TextButton.icon(
                onPressed: () => _toggleAll(false),
                icon: const Icon(Icons.deselect, size: 18),
                label: const Text('Deselect All'),
              ),
              const Spacer(),
              ElevatedButton.icon(
                onPressed: _selected.isEmpty ? null : () => _confirmRemove(),
                style: ElevatedButton.styleFrom(
                  backgroundColor: _selected.isEmpty ? Colors.grey.shade700 : _red,
                  foregroundColor: Colors.white,
                  padding: const EdgeInsets.symmetric(
                    horizontal: 14,
                    vertical: 8,
                  ),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(8),
                  ),
                ),
                icon: const Icon(Icons.delete_outline, size: 18),
                label: Text('Remove Selected (${_selected.length})'),
              ),
            ],
          ),
        ),
        Expanded(
          child: ListView.builder(
            padding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
            itemCount: _apps.length + 1,
            itemBuilder: (context, index) {
              if (index == _apps.length) {
                return Padding(
                  padding: const EdgeInsets.symmetric(vertical: 12),
                  child: Container(
                    padding: const EdgeInsets.all(12),
                    decoration: BoxDecoration(
                      color: _card,
                      borderRadius: BorderRadius.circular(8),
                      border: Border.all(color: _border),
                    ),
                    child: const Row(
                      children: [
                        Icon(Icons.shield, color: _green, size: 18),
                        SizedBox(width: 8),
                        Expanded(
                          child: Text(
                            'Protected: Notepad, Calculator, Store, Photos, Camera, Snipping Tool, Terminal, VCLibs, .NET Native',
                            style: TextStyle(
                              fontSize: 12,
                              color: _green,
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
                );
              }
              final app = _apps[index];
              final isSelected = _selected.contains(app);
              return Container(
                margin: const EdgeInsets.only(bottom: 4),
                decoration: BoxDecoration(
                  color: _card,
                  borderRadius: BorderRadius.circular(8),
                  border: Border.all(
                    color: isSelected ? _red.withOpacity(0.4) : _border,
                  ),
                ),
                child: CheckboxListTile(
                  value: isSelected,
                  onChanged: (_) => _toggleApp(app),
                  activeColor: _brand,
                  checkColor: Colors.white,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(8),
                  ),
                  side: const BorderSide(color: _border),
                  title: Text(
                    app.name,
                    style: const TextStyle(
                      fontSize: 14,
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                  subtitle: Text(
                    app.description,
                    style: TextStyle(
                      fontSize: 12,
                      color: Colors.grey.shade500,
                    ),
                  ),
                  contentPadding: const EdgeInsets.symmetric(
                    horizontal: 8,
                    vertical: 0,
                  ),
                ),
              );
            },
          ),
        ),
      ],
    );
  }
}

class _BloatApp {
  final String name;
  final String description;

  const _BloatApp(this.name, this.description);
}

class SoftwarePage extends StatefulWidget {
  const SoftwarePage({super.key});

  @override
  State<SoftwarePage> createState() => _SoftwarePageState();
}

class _SoftwarePageState extends State<SoftwarePage>
    with SingleTickerProviderStateMixin {
  late final TabController _tabController;
  String _selectedCategory = 'All';

  static const _card = Color(0xFF171717);
  static const _border = Color(0xFF262626);
  static const _brand = Color(0xFF0EA5E9);

  static const _categories = [
    'All',
    'Browsers',
    'Media Players',
    'Music',
    'Gaming',
    'Utilities',
    'Drivers',
    'Communication',
    'Development',
    'Cloud Storage',
  ];

  final _apps = [
    _SoftwareApp('Google Chrome', 'Fast, secure web browser', 'Browsers', Icons.language),
    _SoftwareApp('Brave', 'Privacy-focused web browser', 'Browsers', Icons.shield),
    _SoftwareApp('Zen Browser', 'Minimalist and calm browser', 'Browsers', Icons.public),
    _SoftwareApp('Arc', 'Modern reimagined browser', 'Browsers', Icons.explore),
    _SoftwareApp('Vivaldi', 'Highly customizable browser', 'Browsers', Icons.web),
    _SoftwareApp('Microsoft Edge', 'Built-in Windows browser', 'Browsers', Icons.language),
    _SoftwareApp('VLC Media Player', 'Versatile open-source media player', 'Media Players', Icons.play_circle),
    _SoftwareApp('Screenbox', 'Modern media player for Windows', 'Media Players', Icons.slideshow),
    _SoftwareApp('PotPlayer', 'Advanced media player with codecs', 'Media Players', Icons.video_library),
    _SoftwareApp('Spotify', 'Music streaming service', 'Music', Icons.music_note),
    _SoftwareApp('Anghami', 'Arabic music streaming platform', 'Music', Icons.headphones),
    _SoftwareApp('Windows Media Player', 'Classic Windows media player', 'Music', Icons.album),
    _SoftwareApp('Steam', 'PC gaming platform and store', 'Gaming', Icons.sports_esports),
    _SoftwareApp('Epic Games', 'Game store and game launcher', 'Gaming', Icons.store),
    _SoftwareApp('Riot Client', 'Riot Games game launcher', 'Gaming', Icons.games),
    _SoftwareApp('Discord', 'Voice, video and text chat', 'Gaming', Icons.chat),
    _SoftwareApp('7-Zip', 'File compression and archiver', 'Utilities', Icons.archive),
    _SoftwareApp('Notepad++', 'Advanced text and code editor', 'Utilities', Icons.edit_note),
  ];

  List<_SoftwareApp> get _filteredApps => _selectedCategory == 'All'
      ? _apps
      : _apps.where((a) => a.category == _selectedCategory).toList();

  void _installApp(_SoftwareApp app) {
    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Installing ${app.name}...'),
          backgroundColor: _brand,
        ),
      );
    }
  }

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 2, vsync: this);
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        ColoredBox(
          color: _card,
          child: TabBar(
            controller: _tabController,
            indicatorColor: _brand,
            labelColor: _brand,
            unselectedLabelColor: Colors.grey,
            indicatorWeight: 2,
            labelStyle: const TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.w600,
            ),
            unselectedLabelStyle: const TextStyle(fontSize: 14),
            tabs: const [
              Tab(text: 'Install'),
              Tab(text: 'Debloat'),
            ],
          ),
        ),
        Expanded(
          child: TabBarView(
            controller: _tabController,
            children: [
              _buildInstallTab(),
              const DebloatPage(),
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildInstallTab() {
    return Column(
      children: [
        SizedBox(
          height: 48,
          child: ListView.separated(
            scrollDirection: Axis.horizontal,
            padding: const EdgeInsets.fromLTRB(16, 12, 16, 0),
            itemCount: _categories.length,
            separatorBuilder: (_, __) => const SizedBox(width: 8),
            itemBuilder: (context, index) {
              final cat = _categories[index];
              final selected = _selectedCategory == cat;
              return FilterChip(
                label: Text(cat),
                selected: selected,
                onSelected: (_) => setState(() => _selectedCategory = cat),
                selectedColor: _brand,
                checkmarkColor: Colors.white,
                labelStyle: TextStyle(
                  color: selected ? Colors.white : Colors.grey.shade300,
                  fontSize: 12,
                  fontWeight: FontWeight.w500,
                ),
                backgroundColor: _card,
                side: BorderSide(
                  color: selected ? _brand : _border,
                ),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(20),
                ),
                padding: const EdgeInsets.symmetric(horizontal: 4),
                visualDensity: VisualDensity.compact,
              );
            },
          ),
        ),
        Expanded(
          child: GridView.builder(
            padding: const EdgeInsets.all(16),
            gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: 3,
              crossAxisSpacing: 12,
              mainAxisSpacing: 12,
              childAspectRatio: 0.82,
            ),
            itemCount: _filteredApps.length,
            itemBuilder: (context, index) {
              final app = _filteredApps[index];
              return Card(
                color: _card,
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(12),
                  side: const BorderSide(color: _border),
                ),
                child: Padding(
                  padding: const EdgeInsets.all(12),
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Container(
                        width: 40,
                        height: 40,
                        decoration: BoxDecoration(
                          color: _brand.withOpacity(0.1),
                          borderRadius: BorderRadius.circular(10),
                        ),
                        child: Icon(app.icon, size: 22, color: _brand),
                      ),
                      const SizedBox(height: 8),
                      Text(
                        app.name,
                        style: const TextStyle(
                          fontSize: 13,
                          fontWeight: FontWeight.w600,
                        ),
                        textAlign: TextAlign.center,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                      const SizedBox(height: 4),
                      Expanded(
                        child: Text(
                          app.description,
                          style: TextStyle(
                            fontSize: 11,
                            color: Colors.grey.shade500,
                          ),
                          textAlign: TextAlign.center,
                          maxLines: 2,
                          overflow: TextOverflow.ellipsis,
                        ),
                      ),
                      const SizedBox(height: 8),
                      SizedBox(
                        width: double.infinity,
                        child: ElevatedButton(
                          onPressed: () => _installApp(app),
                          style: ElevatedButton.styleFrom(
                            backgroundColor: _brand,
                            foregroundColor: Colors.white,
                            padding: const EdgeInsets.symmetric(vertical: 6),
                            textStyle: const TextStyle(fontSize: 11),
                            shape: RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(8),
                            ),
                          ),
                          child: const Text('Install via Winget'),
                        ),
                      ),
                    ],
                  ),
                ),
              );
            },
          ),
        ),
      ],
    );
  }
}

class _SoftwareApp {
  final String name;
  final String description;
  final String category;
  final IconData icon;

  const _SoftwareApp(this.name, this.description, this.category, this.icon);
}

class SettingsPage extends StatelessWidget { const SettingsPage({super.key}); @override Widget build(BuildContext context) => const Center(child: Text('Settings — loading...')); }
