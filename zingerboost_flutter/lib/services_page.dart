import 'package:flutter/material.dart';

class ServicesPage extends StatelessWidget {
  const ServicesPage({super.key});

  static const _services = [
    _Svc('DPS', 'Diagnostic Policy Service', true),
    _Svc('WdiServiceHost', 'Diagnostic Service Host', false),
    _Svc('WdiSystemHost', 'Diagnostic System Host', true),
    _Svc('SysMain', 'SysMain (Superfetch)', true),
    _Svc('WSearch', 'Windows Search', true),
    _Svc('wuauserv', 'Windows Update', true),
    _Svc('BITS', 'Background Intelligent Transfer', false),
    _Svc('TabletInputService', 'Touch Keyboard & Handwriting', true),
    _Svc('XblAuthManager', 'Xbox Live Auth Manager', true),
    _Svc('XblGameSave', 'Xbox Live Game Save', false),
    _Svc('XboxNetApiSvc', 'Xbox Live Networking', true),
    _Svc('dmwappushservice', 'Device Management WAP Push', true),
  ];

  @override Widget build(BuildContext c) {
    return ListView(padding: const EdgeInsets.all(12), children: [
      Container(
        margin: const EdgeInsets.only(bottom: 12),
        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
        decoration: BoxDecoration(color: const Color(0xFFEF4444).withOpacity(0.12), borderRadius: BorderRadius.circular(10)),
        child: Row(children: [
          Icon(Icons.warning_amber_rounded, color: const Color(0xFFEF4444), size: 18),
          const SizedBox(width: 10),
          const Expanded(child: Text('Disabling critical services may break functionality. Proceed with caution.', style: TextStyle(fontSize: 12, color: Colors.white70))),
        ]),
      ),
      ..._services.map((s) => Card(
        color: const Color(0xFF171717),
        margin: const EdgeInsets.only(bottom: 8),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10), side: const BorderSide(color: Color(0xFF262626))),
        child: Padding(padding: const EdgeInsets.all(14), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Row(children: [
            Expanded(child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              Text(s.name, style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 14)),
              const SizedBox(height: 2),
              Text(s.display, style: TextStyle(color: Colors.grey.shade500, fontSize: 12)),
            ])),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
              decoration: BoxDecoration(
                color: s.running ? const Color(0xFF10B981).withOpacity(0.15) : Colors.grey.withOpacity(0.15),
                borderRadius: BorderRadius.circular(20),
              ),
              child: Text(s.running ? 'Running' : 'Stopped', style: TextStyle(
                color: s.running ? const Color(0xFF10B981) : Colors.grey,
                fontSize: 11, fontWeight: FontWeight.w500)),
            ),
          ]),
          const SizedBox(height: 12),
          Row(mainAxisAlignment: MainAxisAlignment.end, children: [
            OutlinedButton(
              onPressed: () => ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('${s.name} stopped!'))),
              style: OutlinedButton.styleFrom(
                foregroundColor: const Color(0xFFEF4444), side: const BorderSide(color: Color(0xFFEF4444)),
                padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 6), textStyle: const TextStyle(fontSize: 12)),
              child: const Text('Stop')),
            const SizedBox(width: 8),
            OutlinedButton(
              onPressed: () => ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('${s.name} disabled!'))),
              style: OutlinedButton.styleFrom(
                foregroundColor: Colors.grey, side: const BorderSide(color: Color(0xFF262626)),
                padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 6), textStyle: const TextStyle(fontSize: 12)),
              child: const Text('Disable')),
          ]),
        ])),
      )),
    ]);
  }
}

class _Svc {
  final String name, display;
  final bool running;
  const _Svc(this.name, this.display, this.running);
}
