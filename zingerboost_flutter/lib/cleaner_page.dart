import 'package:flutter/material.dart';

class CleanerPage extends StatelessWidget {
  const CleanerPage({super.key});

  static const _items = [
    _Cln(Icons.delete_sweep, 'Recycle Bin', 'Safe', 120),
    _Cln(Icons.folder_open, 'Temp Files', 'Safe', 340),
    _Cln(Icons.language, 'Browser Cache', 'Safe', 890),
    _Cln(Icons.window, 'Windows Temp', 'Safe', 210),
    _Cln(Icons.dns, 'DNS Cache', 'Safe', 0),
    _Cln(Icons.image, 'Thumbnail Cache', 'Safe', 45),
    _Cln(Icons.article, 'Windows Logs', 'Safe', 67),
    _Cln(Icons.system_update, 'Update Cache', 'Moderate', 1520),
    _Cln(Icons.speed, 'Prefetch', 'Moderate', 180),
  ];

  @override Widget build(BuildContext c) {
    return ListView(padding: const EdgeInsets.all(12), children: [
      Row(children: [
        Expanded(
          child: ElevatedButton.icon(
            onPressed: () => ScaffoldMessenger.of(c).showSnackBar(const SnackBar(content: Text('Scanning complete!'))),
            icon: const Icon(Icons.scanner, size: 16),
            label: const Text('Scan', style: TextStyle(fontSize: 13)),
            style: ElevatedButton.styleFrom(
              foregroundColor: Colors.white, backgroundColor: const Color(0xFF0EA5E9),
              padding: const EdgeInsets.symmetric(vertical: 12), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)))),
        ),
        const SizedBox(width: 12),
        Expanded(
          child: OutlinedButton.icon(
            onPressed: () => ScaffoldMessenger.of(c).showSnackBar(const SnackBar(content: Text('Safe items cleaned!'))),
            icon: const Icon(Icons.cleaning_services, size: 16),
            label: const Text('Clean All Safe', style: TextStyle(fontSize: 13)),
            style: OutlinedButton.styleFrom(
              foregroundColor: const Color(0xFF10B981), side: const BorderSide(color: Color(0xFF10B981)),
              padding: const EdgeInsets.symmetric(vertical: 12), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)))),
        ),
      ]),
      const SizedBox(height: 16),
      ..._items.map((i) => Card(
        color: const Color(0xFF171717),
        margin: const EdgeInsets.only(bottom: 8),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10), side: const BorderSide(color: Color(0xFF262626))),
        child: Padding(padding: const EdgeInsets.all(12), child: Row(children: [
          Container(
            padding: const EdgeInsets.all(8),
            decoration: BoxDecoration(color: const Color(0xFF0EA5E9).withOpacity(0.12), borderRadius: BorderRadius.circular(8)),
            child: Icon(i.icon, color: const Color(0xFF0EA5E9), size: 20)),
          const SizedBox(width: 12),
          Expanded(child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
            Text(i.name, style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 13)),
            const SizedBox(height: 4),
            Row(children: [
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 1),
                decoration: BoxDecoration(
                  color: i.risk == 'Safe' ? const Color(0xFF10B981).withOpacity(0.15) : const Color(0xFFF59E0B).withOpacity(0.15),
                  borderRadius: BorderRadius.circular(20)),
                child: Text(i.risk, style: TextStyle(
                  color: i.risk == 'Safe' ? const Color(0xFF10B981) : const Color(0xFFF59E0B),
                  fontSize: 10, fontWeight: FontWeight.w500)),
              ),
              const SizedBox(width: 8),
              Text('${i.size} MB', style: TextStyle(color: Colors.grey.shade400, fontSize: 12)),
            ]),
          ])),
          TextButton(
            onPressed: () => ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('${i.name} cleaned!'))),
            style: TextButton.styleFrom(
              foregroundColor: const Color(0xFF0EA5E9), padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6), textStyle: const TextStyle(fontSize: 12)),
            child: const Text('Clean')),
        ])),
      )),
    ]);
  }
}

class _Cln {
  final IconData icon;
  final String name, risk;
  final int size;
  const _Cln(this.icon, this.name, this.risk, this.size);
}
