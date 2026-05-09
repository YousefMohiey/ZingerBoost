// =============================================
// Dashboard
// =============================================
class DashPage extends StatelessWidget {
  const DashPage({super.key});
  static const _recs = [
    'Disable Transparency Effects — reduce GPU load',
    'Disable Game DVR — free CPU while gaming',
    'Show File Extensions — security best practice',
  ];
  Widget _mc(IconData ic, String l, String v, String s, Color clr) {
    return Card(
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
  }
  @override Widget build(BuildContext c) {
    return ListView(padding: const EdgeInsets.all(16), children: [
      GridView.count(crossAxisCount: 2, shrinkWrap: true, physics: const NeverScrollableScrollPhysics(), childAspectRatio: 1.6, mainAxisSpacing: 12, crossAxisSpacing: 12, children: [
        _mc(Icons.computer, 'CPU Usage', '15%', '', const Color(0xFF0EA5E9)),
        _mc(Icons.memory, 'RAM Usage', '42%', '6.9 / 16.3 GB', const Color(0xFF10B981)),
        _mc(Icons.storage, 'Disk Active', '5%', '', const Color(0xFFF59E0B)),
        _mc(Icons.wifi, 'Network', '0.5 Mbps', '↑ 0.1 Mbps', const Color(0xFF8B5CF6)),
      ]),
      const SizedBox(height: 20),
      Card(
        color: const Color(0xFF171717),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12), side: const BorderSide(color: Color(0xFF262626))),
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
        ])),
      ),
    ]);
  }
}