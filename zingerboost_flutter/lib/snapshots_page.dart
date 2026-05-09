import 'package:flutter/material.dart';

class SnapshotsPage extends StatelessWidget {
  const SnapshotsPage({super.key});

  static const _snapshots = [
    _Snap('2025-04-15 14:30', 'Applied 3 tweaks batch', 3),
    _Snap('2025-04-14 09:15', 'Applied tweak visual_disable_transparency', 1),
    _Snap('2025-04-13 18:45', 'Applied tweak gaming_disable_dvr', 1),
  ];

  void _confirmRestore(BuildContext c, _Snap s) {
    showDialog(
      context: c,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFF171717),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
        title: const Text('Restore Snapshot', style: TextStyle(color: Colors.white, fontSize: 16)),
        content: Text('Restore system state to "${s.desc}" from ${s.date}?', style: const TextStyle(color: Colors.white70, fontSize: 13)),
        actions: [
          TextButton(onPressed: () => Navigator.pop(ctx), child: const Text('Cancel', style: TextStyle(color: Colors.grey))),
          ElevatedButton(
            onPressed: () {
              Navigator.pop(ctx);
              ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('Restored: ${s.desc}')));
            },
            style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFF0EA5E9), foregroundColor: Colors.white),
            child: const Text('Restore'),
          ),
        ],
      ),
    );
  }

  @override Widget build(BuildContext c) {
    return ListView(padding: const EdgeInsets.all(12), children: [
      ..._snapshots.map((s) => Card(
        color: const Color(0xFF171717),
        margin: const EdgeInsets.only(bottom: 10),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12), side: const BorderSide(color: Color(0xFF262626))),
        child: Padding(padding: const EdgeInsets.all(16), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Row(children: [
            Container(
              padding: const EdgeInsets.all(6),
              decoration: BoxDecoration(color: const Color(0xFF0EA5E9).withOpacity(0.12), borderRadius: BorderRadius.circular(8)),
              child: const Icon(Icons.restore, color: Color(0xFF0EA5E9), size: 18)),
            const SizedBox(width: 10),
            Expanded(child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              Text(s.date, style: TextStyle(color: Colors.grey.shade500, fontSize: 12)),
              const SizedBox(height: 2),
              Text(s.desc, style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 13)),
            ])),
          ]),
          const SizedBox(height: 12),
          Row(children: [
            Icon(Icons.list_alt, color: Colors.grey.shade500, size: 14),
            const SizedBox(width: 6),
            Text('${s.records} record(s)', style: TextStyle(color: Colors.grey.shade400, fontSize: 12)),
            const Spacer(),
            OutlinedButton(
              onPressed: () => _confirmRestore(c, s),
              style: OutlinedButton.styleFrom(
                foregroundColor: const Color(0xFF0EA5E9), side: const BorderSide(color: Color(0xFF0EA5E9)),
                padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 6), textStyle: const TextStyle(fontSize: 12)),
              child: const Text('Restore')),
          ]),
        ])),
      )),
    ]);
  }
}

class _Snap {
  final String date, desc;
  final int records;
  const _Snap(this.date, this.desc, this.records);
}
