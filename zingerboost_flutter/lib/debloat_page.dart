import 'package:flutter/material.dart';

class DebloatPage extends StatefulWidget {
  const DebloatPage({super.key});
  @override State<DebloatPage> createState() => _DebloatPageState();
}
class _DebloatPageState extends State<DebloatPage> {
  static const _apps = [
    'Candy Crush', 'Solitaire', 'Xbox', 'Bing Weather', 'Bing News',
    'Bing Sports', 'Bing Finance', 'Get Help', 'Tips', 'Feedback Hub',
    'Office Hub', 'Mixed Reality', '3D Viewer', 'Paint 3D', 'Skype',
    'Mail & Calendar', 'People', 'Groove Music', 'Movies & TV', 'OneNote',
  ];
  static const _protected = [
    'Microsoft Edge', 'Windows Store', 'Calculator', 'Notepad', 'Paint',
    'Snipping Tool', 'Photos', 'Camera',
  ];

  final Set<String> _selected = {};

  void _toggle(String a) {
    setState(() {
      if (_selected.contains(a)) _selected.remove(a); else _selected.add(a);
    });
  }

  @override Widget build(BuildContext c) {
    return ListView(padding: const EdgeInsets.all(12), children: [
      Container(
        margin: const EdgeInsets.only(bottom: 12),
        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
        decoration: BoxDecoration(color: const Color(0xFFEF4444).withOpacity(0.12), borderRadius: BorderRadius.circular(10)),
        child: Row(children: [
          Icon(Icons.warning_amber_rounded, color: const Color(0xFFEF4444), size: 18),
          const SizedBox(width: 10),
          const Expanded(child: Text('Removing apps is irreversible without a clean Windows install. Some apps are protected.', style: TextStyle(fontSize: 12, color: Colors.white70))),
        ]),
      ),
      Row(children: [
        Expanded(child: OutlinedButton(
          onPressed: () => setState(() => _selected.addAll(_apps)),
          style: OutlinedButton.styleFrom(foregroundColor: Colors.white, side: const BorderSide(color: Color(0xFF262626)), padding: const EdgeInsets.symmetric(vertical: 10), textStyle: const TextStyle(fontSize: 12)),
          child: const Text('Select All'))),
        const SizedBox(width: 8),
        Expanded(child: OutlinedButton(
          onPressed: () => setState(() => _selected.clear()),
          style: OutlinedButton.styleFrom(foregroundColor: Colors.white, side: const BorderSide(color: Color(0xFF262626)), padding: const EdgeInsets.symmetric(vertical: 10), textStyle: const TextStyle(fontSize: 12)),
          child: const Text('Deselect All'))),
      ]),
      const SizedBox(height: 8),
      Row(children: [
        Expanded(child: ElevatedButton(
          onPressed: _selected.isEmpty ? null : () {
            ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('Removed ${_selected.length} app(s)!')));
            setState(() => _selected.clear());
          },
          style: ElevatedButton.styleFrom(foregroundColor: Colors.white, backgroundColor: const Color(0xFFEF4444), padding: const EdgeInsets.symmetric(vertical: 10), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)), textStyle: const TextStyle(fontSize: 12)),
          child: const Text('Remove Selected'))),
        const SizedBox(width: 8),
        Expanded(child: ElevatedButton(
          onPressed: () {
            ScaffoldMessenger.of(c).showSnackBar(const SnackBar(content: Text('All bloatware removed!')));
            setState(() => _selected.clear());
          },
          style: ElevatedButton.styleFrom(foregroundColor: Colors.white, backgroundColor: const Color(0xFFEF4444), padding: const EdgeInsets.symmetric(vertical: 10), shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)), textStyle: const TextStyle(fontSize: 12)),
          child: const Text('Remove All'))),
      ]),
      const SizedBox(height: 12),
      Card(
        color: const Color(0xFF171717),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10), side: const BorderSide(color: Color(0xFF262626))),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Padding(padding: const EdgeInsets.fromLTRB(14, 12, 14, 0), child: Text('Bloatware Apps', style: TextStyle(color: Colors.grey.shade400, fontSize: 11, fontWeight: FontWeight.w600))),
          ..._apps.map((a) => CheckboxListTile(
            dense: true,
            title: Text(a, style: const TextStyle(fontSize: 13)),
            value: _selected.contains(a),
            onChanged: (_) => _toggle(a),
            activeColor: const Color(0xFF0EA5E9),
            checkColor: Colors.white,
            contentPadding: const EdgeInsets.symmetric(horizontal: 8),
          )),
        ]),
      ),
      const SizedBox(height: 16),
      Card(
        color: const Color(0xFF171717),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10), side: BorderSide(color: const Color(0xFF10B981).withOpacity(0.3))),
        child: Padding(
          padding: const EdgeInsets.all(14),
          child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
            Row(children: [
              Icon(Icons.shield, color: const Color(0xFF10B981), size: 16),
              const SizedBox(width: 8),
              const Text('Protected Apps', style: TextStyle(fontWeight: FontWeight.w600, fontSize: 13)),
            ]),
            const SizedBox(height: 8),
            Wrap(spacing: 6, runSpacing: 6, children: _protected.map((p) => Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              decoration: BoxDecoration(color: const Color(0xFF10B981).withOpacity(0.1), borderRadius: BorderRadius.circular(20), border: Border.all(color: const Color(0xFF10B981).withOpacity(0.3))),
              child: Text(p, style: TextStyle(color: const Color(0xFF10B981), fontSize: 11)),
            )).toList()),
          ]),
        ),
      ),
    ]);
  }
}
