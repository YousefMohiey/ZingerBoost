import 'package:flutter/material.dart';

import '../models/software.dart';
import '../services/rust_bridge.dart';

class DebloatPage extends StatefulWidget {
  const DebloatPage({super.key});

  @override
  State<DebloatPage> createState() => _DebloatPageState();
}

class _DebloatPageState extends State<DebloatPage> {
  late List<SoftwareInfo> _bloatware;
  late Set<String> _selectedIds;
  late bool _loading;
  late bool _removing;

  static const _bg = Color(0xFF0A0A0A);
  static const _card = Color(0xFF171717);
  static const _border = Color(0xFF262626);
  static const _brand = Color(0xFF0EA5E9);
  static const _red = Color(0xFFEF4444);
  static const _amber = Color(0xFFF59E0B);
  static const _green = Color(0xFF10B981);

  @override
  void initState() {
    super.initState();
    _bloatware = [];
    _selectedIds = {};
    _loading = true;
    _removing = false;
    WidgetsBinding.instance.addPostFrameCallback((_) => _loadBloatware());
  }

  Future<void> _loadBloatware() async {
    try {
      final list = await RustBridge.listBloatware();
      if (mounted) {
        setState(() {
          _bloatware = list;
          _loading = false;
        });
      }
    } catch (_) {
      if (mounted) setState(() => _loading = false);
    }
  }

  Future<void> _removeAll() async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: _card,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: _border),
        ),
        title: const Text(
          'Remove All Bloatware',
          style: TextStyle(color: Colors.white),
        ),
        content: const Text(
          'This will remove all detected bloatware applications. This action cannot be undone. Continue?',
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
            child: const Text('Remove All'),
          ),
        ],
      ),
    );

    if (confirmed != true) return;

    setState(() => _removing = true);
    try {
      await RustBridge.removeAllBloatware();
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('All bloatware removed'),
            backgroundColor: _green,
          ),
        );
        _loadBloatware();
      }
    } catch (_) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Failed to remove bloatware'),
            backgroundColor: _red,
          ),
        );
      }
    } finally {
      if (mounted) setState(() => _removing = false);
    }
  }

  Future<void> _removeSelected() async {
    if (_selectedIds.isEmpty) return;

    setState(() => _removing = true);
    try {
      for (final id in _selectedIds.toList()) {
        await RustBridge.removeBloatware(id);
      }
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('${_selectedIds.length} apps removed'),
            backgroundColor: _green,
          ),
        );
        _selectedIds.clear();
        _loadBloatware();
      }
    } catch (_) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Failed to remove some apps'),
            backgroundColor: _red,
          ),
        );
      }
    } finally {
      if (mounted) setState(() => _removing = false);
    }
  }

  List<SoftwareInfo> get _bloat => _bloatware.where((b) => !b.isProtected).toList();
  List<SoftwareInfo> get _protected => _bloatware.where((b) => b.isProtected).toList();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: _bg,
      body: _loading
          ? const Center(
              child: CircularProgressIndicator(color: _brand),
            )
          : Padding(
              padding: const EdgeInsets.all(24),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      const Expanded(
                        child: Text(
                          'Debloat',
                          style: TextStyle(
                            fontSize: 28,
                            fontWeight: FontWeight.bold,
                            color: Colors.white,
                          ),
                        ),
                      ),
                      SizedBox(
                        height: 40,
                        child: ElevatedButton.icon(
                          onPressed: _removing ? null : _removeAll,
                          icon: const Icon(Icons.delete_sweep, size: 16),
                          label: const Text('Remove All Bloatware'),
                          style: ElevatedButton.styleFrom(
                            backgroundColor: _red,
                            foregroundColor: Colors.white,
                            padding: const EdgeInsets.symmetric(
                                horizontal: 16, vertical: 8),
                            shape: RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(8),
                            ),
                          ),
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 16),
                  _buildWarningBanner(),
                  const SizedBox(height: 16),
                  if (_selectedIds.isNotEmpty)
                    Padding(
                      padding: const EdgeInsets.only(bottom: 12),
                      child: SizedBox(
                        width: double.infinity,
                        child: ElevatedButton.icon(
                          onPressed: _removing ? null : _removeSelected,
                          icon: const Icon(Icons.delete_outline, size: 16),
                          label: Text(
                              'Remove Selected (${_selectedIds.length})'),
                          style: ElevatedButton.styleFrom(
                            backgroundColor: _amber,
                            foregroundColor: Colors.black,
                            padding:
                                const EdgeInsets.symmetric(vertical: 12),
                            shape: RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(8),
                            ),
                          ),
                        ),
                      ),
                    ),
                  Expanded(child: _buildBloatList()),
                  if (_protected.isNotEmpty) ...[
                    const SizedBox(height: 12),
                    _buildProtectedPanel(),
                  ],
                ],
              ),
            ),
    );
  }

  Widget _buildWarningBanner() {
    return Container(
      padding: const EdgeInsets.all(14),
      decoration: BoxDecoration(
        color: _amber.withOpacity(0.1),
        borderRadius: BorderRadius.circular(10),
        border: Border.all(color: _amber.withOpacity(0.3)),
      ),
      child: Row(
        children: [
          const Icon(Icons.warning_amber_rounded, color: _amber, size: 22),
          const SizedBox(width: 12),
          Expanded(
            child: Text(
              'Removing bloatware can affect system functionality. Changes are not easily reversible. '
              'Protected system components are shown separately.',
              style: TextStyle(
                fontSize: 12,
                color: _amber.withOpacity(0.9),
                height: 1.4,
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildBloatList() {
    if (_bloat.isEmpty) {
      return Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.check_circle_outline,
                size: 48, color: Colors.grey[600]),
            const SizedBox(height: 12),
            Text(
              'No bloatware detected',
              style: TextStyle(fontSize: 16, color: Colors.grey[500]),
            ),
          ],
        ),
      );
    }

    return ListView.builder(
      itemCount: _bloat.length,
      itemBuilder: (context, index) {
        final app = _bloat[index];
        final selected = _selectedIds.contains(app.id);

        return Card(
          color: _card,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(10),
            side: BorderSide(
              color: selected ? _red.withOpacity(0.5) : _border,
            ),
          ),
          margin: const EdgeInsets.only(bottom: 8),
          child: InkWell(
            borderRadius: BorderRadius.circular(10),
            onTap: () {
              setState(() {
                if (selected) {
                  _selectedIds.remove(app.id);
                } else {
                  _selectedIds.add(app.id);
                }
              });
            },
            child: Padding(
              padding: const EdgeInsets.all(12),
              child: Row(
                children: [
                  Checkbox(
                    value: selected,
                    onChanged: (val) {
                      setState(() {
                        if (val == true) {
                          _selectedIds.add(app.id);
                        } else {
                          _selectedIds.remove(app.id);
                        }
                      });
                    },
                    activeColor: _red,
                    checkColor: Colors.white,
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(4),
                    ),
                    side: const BorderSide(color: _border),
                  ),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          app.name,
                          style: const TextStyle(
                            fontSize: 14,
                            fontWeight: FontWeight.w600,
                            color: Colors.white,
                          ),
                        ),
                        if (app.description.isNotEmpty) ...[
                          const SizedBox(height: 2),
                          Text(
                            app.description,
                            style: TextStyle(
                              fontSize: 12,
                              color: Colors.grey[500],
                            ),
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                          ),
                        ],
                      ],
                    ),
                  ),
                  IconButton(
                    onPressed: () async {
                      try {
                        await RustBridge.removeBloatware(app.id);
                        if (mounted) {
                          _selectedIds.remove(app.id);
                          _loadBloatware();
                        }
                      } catch (_) {}
                    },
                    icon: const Icon(Icons.delete_outline, size: 18),
                    color: _red,
                    tooltip: 'Remove',
                  ),
                ],
              ),
            ),
          ),
        );
      },
    );
  }

  Widget _buildProtectedPanel() {
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
              const Icon(Icons.shield, size: 18, color: _brand),
              const SizedBox(width: 8),
              Text(
                'Protected Apps (${_protected.length})',
                style: const TextStyle(
                  fontSize: 14,
                  fontWeight: FontWeight.w600,
                  color: Colors.white,
                ),
              ),
            ],
          ),
          const SizedBox(height: 4),
          Text(
            'These apps are critical system components and cannot be removed.',
            style: TextStyle(fontSize: 12, color: Colors.grey[500]),
          ),
          const SizedBox(height: 10),
          Wrap(
            spacing: 8,
            runSpacing: 6,
            children: _protected.map((app) {
              return Container(
                padding:
                    const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
                decoration: BoxDecoration(
                  color: _brand.withOpacity(0.1),
                  borderRadius: BorderRadius.circular(6),
                  border: Border.all(color: _border),
                ),
                child: Text(
                  app.name,
                  style: TextStyle(fontSize: 12, color: Colors.grey[300]),
                ),
              );
            }).toList(),
          ),
        ],
      ),
    );
  }
}
