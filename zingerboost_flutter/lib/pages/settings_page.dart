import 'package:flutter/material.dart';

import '../services/rust_bridge.dart';
import '../theme/theme_provider.dart';

class SettingsPage extends StatefulWidget {
  const SettingsPage({super.key});

  @override
  State<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  late ThemeMode _themeMode;
  late TextEditingController _dataDirCtrl;
  late bool _autoStart;
  late bool _expertMode;
  late double _snapshotRetention;
  late bool _loading;

  static const _bg = Color(0xFF0A0A0A);
  static const _card = Color(0xFF171717);
  static const _border = Color(0xFF262626);
  static const _brand = Color(0xFF0EA5E9);
  static const _green = Color(0xFF10B981);
  static const _red = Color(0xFFEF4444);

  @override
  void initState() {
    super.initState();
    _themeMode = ThemeMode.dark;
    _dataDirCtrl = TextEditingController();
    _autoStart = false;
    _expertMode = false;
    _snapshotRetention = 30;
    _loading = true;
    WidgetsBinding.instance.addPostFrameCallback((_) => _loadSettings());
  }

  Future<void> _loadSettings() async {
    try {
      final settings = await RustBridge.getSettings();
      if (mounted) {
        setState(() {
          _dataDirCtrl.text = settings['data_dir'] ?? '';
          _autoStart = settings['auto_start'] ?? false;
          _expertMode = settings['expert_mode'] ?? false;
          _snapshotRetention =
              (settings['snapshot_retention'] ?? 30).toDouble();
          _loading = false;
        });
      }
    } catch (_) {
      if (mounted) setState(() => _loading = false);
    }
  }

  Future<void> _saveSetting(String key, dynamic value) async {
    try {
      await RustBridge.saveSetting(key, value);
    } catch (_) {}
  }

  Future<void> _exportData() async {
    try {
      await RustBridge.exportSettings();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Settings exported successfully'),
            backgroundColor: _green,
          ),
        );
      }
    } catch (_) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Export failed'),
            backgroundColor: _red,
          ),
        );
      }
    }
  }

  Future<void> _importData() async {
    try {
      await RustBridge.importSettings();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Settings imported successfully'),
            backgroundColor: _green,
          ),
        );
        _loadSettings();
      }
    } catch (_) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Import failed'),
            backgroundColor: _red,
          ),
        );
      }
    }
  }

  Future<void> _checkForUpdates() async {
    try {
      final updateAvailable = await RustBridge.checkForUpdates();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              updateAvailable ? 'Update available!' : 'You are up to date',
            ),
            backgroundColor: updateAvailable ? _brand : _green,
          ),
        );
      }
    } catch (_) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Failed to check for updates'),
            backgroundColor: _red,
          ),
        );
      }
    }
  }

  Future<void> _resetAllTweaks() async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: _card,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: _border),
        ),
        title: const Text(
          'Reset All Tweaks',
          style: TextStyle(color: Colors.white),
        ),
        content: const Text(
          'This will revert all applied tweaks to their default values. This action cannot be undone. Continue?',
          style: TextStyle(color: Colors.white70),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx, false),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () => Navigator.pop(ctx, true),
            style: ElevatedButton.styleFrom(backgroundColor: _red),
            child: const Text('Reset All'),
          ),
        ],
      ),
    );

    if (confirmed != true) return;

    try {
      await RustBridge.resetAllTweaks();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('All tweaks have been reset'),
            backgroundColor: _green,
          ),
        );
      }
    } catch (_) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Failed to reset tweaks'),
            backgroundColor: _red,
          ),
        );
      }
    }
  }

  @override
  void dispose() {
    _dataDirCtrl.dispose();
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
          : ListView(
              padding: const EdgeInsets.all(24),
              children: [
                const Text(
                  'Settings',
                  style: TextStyle(
                    fontSize: 28,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                const SizedBox(height: 24),

                _buildSection('Appearance', [
                  _buildThemeToggle(),
                ]),

                _buildSection('General', [
                  _buildDataDir(),
                  _buildSwitch('Auto-start', _autoStart, (val) {
                    setState(() => _autoStart = val);
                    _saveSetting('auto_start', val);
                  }),
                  _buildSwitch('Expert Mode', _expertMode, (val) {
                    setState(() => _expertMode = val);
                    _saveSetting('expert_mode', val);
                  }),
                ]),

                _buildSection('Snapshots', [
                  _buildSnapshotRetention(),
                ]),

                _buildSection('Data', [
                  _buildExportImport(),
                ]),

                _buildSection('Updates', [
                  _buildUpdateButton(),
                ]),

                _buildSection('Danger Zone', [
                  _buildResetButton(),
                ]),

                const SizedBox(height: 24),
                _buildAboutSection(),
              ],
            ),
    );
  }

  Widget _buildSection(String title, List<Widget> children) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            title,
            style: const TextStyle(
              fontSize: 13,
              fontWeight: FontWeight.w600,
              color: _brand,
              letterSpacing: 0.5,
            ),
          ),
          const SizedBox(height: 10),
          Container(
            decoration: BoxDecoration(
              color: _card,
              borderRadius: BorderRadius.circular(12),
              border: Border.all(color: _border),
            ),
            child: Column(
              children: children,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildThemeToggle() {
    return ListTile(
      contentPadding:
          const EdgeInsets.symmetric(horizontal: 16, vertical: 2),
      leading: const Icon(Icons.palette, color: _brand, size: 22),
      title: const Text(
        'Theme',
        style: TextStyle(color: Colors.white, fontSize: 14),
      ),
      trailing: Container(
        padding: const EdgeInsets.symmetric(horizontal: 12),
        decoration: BoxDecoration(
          color: _bg,
          borderRadius: BorderRadius.circular(8),
          border: Border.all(color: _border),
        ),
        child: DropdownButtonHideUnderline(
          child: DropdownButton<ThemeMode>(
            value: _themeMode,
            dropdownColor: _card,
            style: const TextStyle(color: Colors.white, fontSize: 13),
            items: const [
              DropdownMenuItem(
                value: ThemeMode.dark,
                child: Text('Dark'),
              ),
              DropdownMenuItem(
                value: ThemeMode.light,
                child: Text('Light'),
              ),
              DropdownMenuItem(
                value: ThemeMode.system,
                child: Text('System'),
              ),
            ],
            onChanged: (mode) {
              if (mode != null) {
                setState(() => _themeMode = mode);
                _saveSetting('theme', mode.index);
              }
            },
          ),
        ),
      ),
    );
  }

  Widget _buildDataDir() {
    return ListTile(
      contentPadding:
          const EdgeInsets.symmetric(horizontal: 16, vertical: 2),
      leading: const Icon(Icons.folder, color: _brand, size: 22),
      title: const Text(
        'Data Directory',
        style: TextStyle(color: Colors.white, fontSize: 14),
      ),
      subtitle: TextField(
        controller: _dataDirCtrl,
        style: const TextStyle(color: Colors.white70, fontSize: 12),
        decoration: InputDecoration(
          hintText: 'Select data directory...',
          hintStyle: TextStyle(color: Colors.grey[600]),
          border: InputBorder.none,
          contentPadding: EdgeInsets.zero,
          isDense: true,
        ),
        onSubmitted: (val) => _saveSetting('data_dir', val),
      ),
      trailing: IconButton(
        icon: const Icon(Icons.folder_open, size: 20),
        color: Colors.grey[500],
        onPressed: () async {
          final dir = await RustBridge.pickDirectory();
          if (dir != null && mounted) {
            setState(() => _dataDirCtrl.text = dir);
            _saveSetting('data_dir', dir);
          }
        },
      ),
    );
  }

  Widget _buildSwitch(String label, bool value, ValueChanged<bool> onChanged) {
    return ListTile(
      contentPadding:
          const EdgeInsets.symmetric(horizontal: 16, vertical: 2),
      leading: Icon(
        label == 'Auto-start' ? Icons.power_settings_new : Icons.tune,
        color: _brand,
        size: 22,
      ),
      title: Text(
        label,
        style: const TextStyle(color: Colors.white, fontSize: 14),
      ),
      trailing: Switch(
        value: value,
        onChanged: onChanged,
        activeColor: _brand,
        activeTrackColor: _brand.withOpacity(0.3),
      ),
    );
  }

  Widget _buildSnapshotRetention() {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              const Icon(Icons.history, color: _brand, size: 22),
              const SizedBox(width: 12),
              const Text(
                'Snapshot Retention',
                style: TextStyle(color: Colors.white, fontSize: 14),
              ),
              const Spacer(),
              Text(
                '${_snapshotRetention.round()} days',
                style: const TextStyle(
                  color: _brand,
                  fontSize: 13,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ],
          ),
          Slider(
            value: _snapshotRetention,
            min: 7,
            max: 365,
            divisions: 358,
            activeColor: _brand,
            inactiveColor: _border,
            onChanged: (val) {
              setState(() => _snapshotRetention = val);
            },
            onChangeEnd: (val) {
              _saveSetting('snapshot_retention', val.round());
            },
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text('7 days',
                  style: TextStyle(fontSize: 11, color: Colors.grey[600])),
              Text('365 days',
                  style: TextStyle(fontSize: 11, color: Colors.grey[600])),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildExportImport() {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Row(
        children: [
          Expanded(
            child: OutlinedButton.icon(
              onPressed: _exportData,
              icon: const Icon(Icons.upload, size: 16),
              label: const Text('Export'),
              style: OutlinedButton.styleFrom(
                foregroundColor: _brand,
                side: const BorderSide(color: _border),
                padding: const EdgeInsets.symmetric(vertical: 14),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(8),
                ),
              ),
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: OutlinedButton.icon(
              onPressed: _importData,
              icon: const Icon(Icons.download, size: 16),
              label: const Text('Import'),
              style: OutlinedButton.styleFrom(
                foregroundColor: _brand,
                side: const BorderSide(color: _border),
                padding: const EdgeInsets.symmetric(vertical: 14),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(8),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildUpdateButton() {
    return ListTile(
      contentPadding:
          const EdgeInsets.symmetric(horizontal: 16, vertical: 2),
      leading: const Icon(Icons.system_update, color: _brand, size: 22),
      title: const Text(
        'Check for Updates',
        style: TextStyle(color: Colors.white, fontSize: 14),
      ),
      trailing: const Icon(Icons.chevron_right, color: Colors.grey),
      onTap: _checkForUpdates,
    );
  }

  Widget _buildResetButton() {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: SizedBox(
        width: double.infinity,
        child: OutlinedButton.icon(
          onPressed: _resetAllTweaks,
          icon: const Icon(Icons.restore, size: 16),
          label: const Text('Reset All Tweaks'),
          style: OutlinedButton.styleFrom(
            foregroundColor: _red,
            side: const BorderSide(color: _red),
            padding: const EdgeInsets.symmetric(vertical: 14),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildAboutSection() {
    return Container(
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        color: _card,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: _border),
      ),
      child: Column(
        children: [
          Container(
            width: 56,
            height: 56,
            decoration: BoxDecoration(
              color: _brand.withOpacity(0.1),
              borderRadius: BorderRadius.circular(14),
            ),
            child: const Icon(Icons.bolt, color: _brand, size: 28),
          ),
          const SizedBox(height: 16),
          const Text(
            'ZingerBoost',
            style: TextStyle(
              fontSize: 20,
              fontWeight: FontWeight.bold,
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 6),
          Text(
            'Version 0.1.0',
            style: TextStyle(fontSize: 13, color: Colors.grey[500]),
          ),
          const SizedBox(height: 4),
          Text(
            'Advanced Windows optimization toolkit',
            style: TextStyle(fontSize: 12, color: Colors.grey[600]),
          ),
          const SizedBox(height: 16),
          Text(
            'Built with Tauri + Flutter',
            style: TextStyle(fontSize: 11, color: Colors.grey[700]),
          ),
        ],
      ),
    );
  }
}
