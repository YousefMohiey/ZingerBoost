import 'package:flutter/material.dart';

import '../models/software.dart';
import '../services/rust_bridge.dart';

class SoftwarePage extends StatefulWidget {
  const SoftwarePage({super.key});

  @override
  State<SoftwarePage> createState() => _SoftwarePageState();
}

class _SoftwarePageState extends State<SoftwarePage>
    with SingleTickerProviderStateMixin {
  late TabController _tabCtrl;
  late List<SoftwareInfo> _apps;
  late List<SoftwareInfo> _filteredApps;
  late bool _loading;
  late String _selectedCategory;

  static const _bg = Color(0xFF0A0A0A);
  static const _card = Color(0xFF171717);
  static const _border = Color(0xFF262626);
  static const _brand = Color(0xFF0EA5E9);
  static const _green = Color(0xFF10B981);

  static const _categories = {
    'All': Icons.apps,
    'browsers': Icons.language,
    'media_players': Icons.play_circle,
    'music': Icons.music_note,
    'gaming': Icons.sports_esports,
    'utilities': Icons.build,
    'drivers': Icons.usb,
    'communication': Icons.chat,
    'development': Icons.code,
    'cloud_storage': Icons.cloud,
  };

  String _categoryLabel(String cat) {
    return cat.replaceAll('_', ' ').split(' ').map((w) {
      if (w.isEmpty) return '';
      return w[0].toUpperCase() + w.substring(1);
    }).join(' ');
  }

  @override
  void initState() {
    super.initState();
    _tabCtrl = TabController(length: 2, vsync: this);
    _apps = [];
    _filteredApps = [];
    _loading = true;
    _selectedCategory = 'All';
    WidgetsBinding.instance.addPostFrameCallback((_) => _loadApps());
  }

  Future<void> _loadApps() async {
    try {
      final apps = await RustBridge.listSoftware();
      if (mounted) {
        setState(() {
          _apps = apps;
          _loading = false;
        });
        _applyFilter();
      }
    } catch (_) {
      if (mounted) setState(() => _loading = false);
    }
  }

  void _applyFilter() {
    setState(() {
      if (_selectedCategory == 'All') {
        _filteredApps = List.from(_apps);
      } else {
        _filteredApps =
            _apps.where((a) => a.category == _selectedCategory).toList();
      }
    });
  }

  Future<void> _installApp(String id, String name) async {
    try {
      await RustBridge.installSoftware(id);
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Installing $name...'),
            backgroundColor: _brand,
          ),
        );
      }
    } catch (_) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Failed to install $name'),
            backgroundColor: const Color(0xFFEF4444),
          ),
        );
      }
    }
  }

  @override
  void dispose() {
    _tabCtrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: _bg,
      body: _loading
          ? const Center(
              child: CircularProgressIndicator(color: _brand),
            )
          : Column(
              children: [
                _buildHeader(),
                _buildCategoryPills(),
                Expanded(
                  child: TabBarView(
                    controller: _tabCtrl,
                    children: [
                      _buildAppGrid(),
                      _buildDebloatRedirect(),
                    ],
                  ),
                ),
              ],
            ),
    );
  }

  Widget _buildHeader() {
    return Padding(
      padding: const EdgeInsets.fromLTRB(24, 24, 24, 0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'Software',
            style: TextStyle(
              fontSize: 28,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 16),
          TabBar(
            controller: _tabCtrl,
            labelColor: _brand,
            unselectedLabelColor: Colors.grey[500],
            indicatorColor: _brand,
            indicatorSize: TabBarIndicatorSize.label,
            labelStyle: const TextStyle(
                fontSize: 14, fontWeight: FontWeight.w600),
            unselectedLabelStyle: const TextStyle(fontSize: 14),
            tabs: const [
              Tab(text: 'Install'),
              Tab(text: 'Debloat'),
            ],
            onTap: (index) {
              if (index == 1) {
                Navigator.pushNamed(context, '/debloat');
              }
            },
          ),
        ],
      ),
    );
  }

  Widget _buildCategoryPills() {
    final cats = _categories.keys.toList();
    return SizedBox(
      height: 44,
      child: ListView.separated(
        scrollDirection: Axis.horizontal,
        padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 6),
        itemCount: cats.length,
        separatorBuilder: (_, __) => const SizedBox(width: 8),
        itemBuilder: (context, index) {
          final cat = cats[index];
          final selected = _selectedCategory == cat;
          return ChoiceChip(
            label: Text(_categoryLabel(cat)),
            selected: selected,
            onSelected: (_) {
              setState(() => _selectedCategory = cat);
              _applyFilter();
            },
            backgroundColor: _card,
            selectedColor: _brand,
            labelStyle: TextStyle(
              color: selected ? Colors.white : Colors.grey[400],
              fontSize: 12,
              fontWeight: FontWeight.w500,
            ),
            side: BorderSide(color: selected ? _brand : _border),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
          );
        },
      ),
    );
  }

  Widget _buildAppGrid() {
    if (_filteredApps.isEmpty) {
      return Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.apps_outlined, size: 48, color: Colors.grey[600]),
            const SizedBox(height: 12),
            Text(
              'No apps in this category',
              style: TextStyle(fontSize: 16, color: Colors.grey[500]),
            ),
          ],
        ),
      );
    }

    return Padding(
      padding: const EdgeInsets.all(24),
      child: GridView.builder(
        gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
          crossAxisCount: 2,
          crossAxisSpacing: 16,
          mainAxisSpacing: 16,
          childAspectRatio: 1.6,
        ),
        itemCount: _filteredApps.length,
        itemBuilder: (context, index) {
          final app = _filteredApps[index];

          return Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: _card,
              borderRadius: BorderRadius.circular(12),
              border: Border.all(color: _border),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Expanded(
                      child: Text(
                        app.name,
                        style: const TextStyle(
                          fontSize: 15,
                          fontWeight: FontWeight.w600,
                          color: Colors.white,
                        ),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                    Container(
                      padding: const EdgeInsets.symmetric(
                          horizontal: 8, vertical: 3),
                      decoration: BoxDecoration(
                        color: _brand.withOpacity(0.1),
                        borderRadius: BorderRadius.circular(6),
                      ),
                      child: Text(
                        _categoryLabel(app.category),
                        style: const TextStyle(
                          fontSize: 10,
                          fontWeight: FontWeight.w500,
                          color: _brand,
                        ),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 8),
                Expanded(
                  child: Text(
                    app.description,
                    style: TextStyle(
                      fontSize: 12,
                      color: Colors.grey[400],
                      height: 1.3,
                    ),
                    maxLines: 3,
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                const SizedBox(height: 12),
                SizedBox(
                  width: double.infinity,
                  child: ElevatedButton.icon(
                    onPressed: () => _installApp(app.id, app.name),
                    icon: const Icon(Icons.download, size: 14),
                    label: const Text('Install via Winget'),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: _green,
                      foregroundColor: Colors.white,
                      padding:
                          const EdgeInsets.symmetric(vertical: 8),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(8),
                      ),
                      textStyle: const TextStyle(fontSize: 12),
                    ),
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _buildDebloatRedirect() {
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(Icons.cleaning_services, size: 64, color: Colors.grey[600]),
          const SizedBox(height: 20),
          const Text(
            'Debloat Tools',
            style: TextStyle(
              fontSize: 20,
              fontWeight: FontWeight.w600,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 8),
          Text(
            'Remove unwanted pre-installed applications',
            style: TextStyle(fontSize: 14, color: Colors.grey[500]),
          ),
          const SizedBox(height: 24),
          ElevatedButton.icon(
            onPressed: () => Navigator.pushNamed(context, '/debloat'),
            icon: const Icon(Icons.open_in_new, size: 16),
            label: const Text('Open Debloat Page'),
            style: ElevatedButton.styleFrom(
              backgroundColor: _brand,
              foregroundColor: Colors.white,
              padding:
                  const EdgeInsets.symmetric(horizontal: 24, vertical: 14),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(10),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
