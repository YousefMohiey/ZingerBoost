import 'package:flutter/material.dart';

import '../services/rust_bridge.dart';

class SnapshotInfo {
  final String id;
  final DateTime date;
  final String description;
  final int tweakCount;

  const SnapshotInfo({
    required this.id,
    required this.date,
    required this.description,
    required this.tweakCount,
  });

  factory SnapshotInfo.fromJson(Map<String, dynamic> json) {
    return SnapshotInfo(
      id: json['id'] ?? '',
      date: DateTime.tryParse(json['date'] ?? '') ?? DateTime.now(),
      description: json['description'] ?? '',
      tweakCount: json['tweak_count'] ?? 0,
    );
  }
}

class SnapshotsPage extends StatefulWidget {
  const SnapshotsPage({super.key});

  @override
  State<SnapshotsPage> createState() => _SnapshotsPageState();
}

class _SnapshotsPageState extends State<SnapshotsPage> {
  late List<SnapshotInfo> _snapshots;
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
    _snapshots = [];
    _loading = true;
    WidgetsBinding.instance.addPostFrameCallback((_) => _loadSnapshots());
  }

  Future<void> _loadSnapshots() async {
    try {
      final snapshots = await RustBridge.listSnapshots();
      if (mounted) {
        setState(() {
          _snapshots = snapshots.map((s) => s as SnapshotInfo).toList();
          _loading = false;
        });
      }
    } catch (_) {
      if (mounted) setState(() => _loading = false);
    }
  }

  Future<void> _restoreSnapshot(String id) async {
    try {
      await RustBridge.restoreSnapshot(id);
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Snapshot restored successfully'),
            backgroundColor: _green,
          ),
        );
        _loadSnapshots();
      }
    } catch (_) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Failed to restore snapshot'),
            backgroundColor: _red,
          ),
        );
      }
    }
  }

  String _formatDate(DateTime dt) {
    return '${dt.year}-${dt.month.toString().padLeft(2, '0')}-${dt.day.toString().padLeft(2, '0')}  '
        '${dt.hour.toString().padLeft(2, '0')}:${dt.minute.toString().padLeft(2, '0')}';
  }

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
                  const Text(
                    'Snapshots',
                    style: TextStyle(
                      fontSize: 28,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                  ),
                  const SizedBox(height: 8),
                  Text(
                    'Restore your system to a previous state',
                    style: TextStyle(fontSize: 14, color: Colors.grey[500]),
                  ),
                  const SizedBox(height: 24),
                  Expanded(child: _buildTimeline()),
                ],
              ),
            ),
    );
  }

  Widget _buildTimeline() {
    if (_snapshots.isEmpty) {
      return Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Icon(Icons.history, size: 64, color: _brand),
            const SizedBox(height: 16),
            const Text(
              'No Snapshots Yet',
              style: TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.w600,
                color: Colors.white,
              ),
            ),
            const SizedBox(height: 8),
            Text(
              'Create a snapshot after applying tweaks to save\nyour system state for easy restoration.',
              textAlign: TextAlign.center,
              style: TextStyle(fontSize: 13, color: Colors.grey[500], height: 1.5),
            ),
          ],
        ),
      );
    }

    return ListView.builder(
      itemCount: _snapshots.length,
      itemBuilder: (context, index) {
        final snap = _snapshots[index];
        final isLast = index == _snapshots.length - 1;

        return IntrinsicHeight(
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              SizedBox(
                width: 40,
                child: Column(
                  children: [
                    Container(
                      width: 14,
                      height: 14,
                      decoration: BoxDecoration(
                        shape: BoxShape.circle,
                        color: isLast ? _brand : _brand.withOpacity(0.4),
                        border: Border.all(
                          color: _brand,
                          width: 2,
                        ),
                      ),
                    ),
                    if (!isLast)
                      Expanded(
                        child: Container(
                          width: 2,
                          color: _border,
                        ),
                      ),
                  ],
                ),
              ),
              Expanded(
                child: Container(
                  margin: const EdgeInsets.only(bottom: 16),
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
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Expanded(
                            child: Text(
                              _formatDate(snap.date),
                              style: const TextStyle(
                                fontSize: 15,
                                fontWeight: FontWeight.w600,
                                color: Colors.white,
                              ),
                            ),
                          ),
                          Container(
                            padding: const EdgeInsets.symmetric(
                                horizontal: 8, vertical: 4),
                            decoration: BoxDecoration(
                              color: _brand.withOpacity(0.1),
                              borderRadius: BorderRadius.circular(6),
                            ),
                            child: Text(
                              '${snap.tweakCount} tweaks',
                              style: const TextStyle(
                                fontSize: 11,
                                fontWeight: FontWeight.w500,
                                color: _brand,
                              ),
                            ),
                          ),
                        ],
                      ),
                      if (snap.description.isNotEmpty) ...[
                        const SizedBox(height: 8),
                        Text(
                          snap.description,
                          style: TextStyle(
                              fontSize: 13, color: Colors.grey[400]),
                        ),
                      ],
                      const SizedBox(height: 12),
                      SizedBox(
                        width: double.infinity,
                        child: OutlinedButton.icon(
                          onPressed: () => _restoreSnapshot(snap.id),
                          icon: const Icon(Icons.restore, size: 16),
                          label: const Text('Restore'),
                          style: OutlinedButton.styleFrom(
                            foregroundColor: _brand,
                            side: const BorderSide(color: _border),
                            padding: const EdgeInsets.symmetric(vertical: 10),
                            shape: RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(8),
                            ),
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
