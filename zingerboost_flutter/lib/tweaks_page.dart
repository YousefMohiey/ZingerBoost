import 'package:flutter/material.dart';

class TweaksPage extends StatefulWidget {
  const TweaksPage({super.key});
  @override State<TweaksPage> createState() => _TweaksPageState();
}
class _TweaksPageState extends State<TweaksPage> {
  String _filter = 'All', _search = '';
  static const _items = [
    _Tk('Disable Transparency Effects','Turns off acrylic effects','Visual','Safe'),
    _Tk('Disable Game DVR','Stops background recording','Gaming','Safe'),
    _Tk('Show File Extensions','Show extensions in Explorer','Visual','Safe'),
    _Tk('Disable Telemetry','Minimize diagnostic data','Privacy','Safe'),
    _Tk('Disable Startup Delay','Remove 10s boot delay','Performance','Safe'),
    _Tk('Disable Sticky Keys Popup','No Shift x5 interruptions','Visual','Safe'),
    _Tk('Disable Background Apps','Stop UWP background processes','Privacy','Safe'),
    _Tk('High Performance Power Plan','Prevent CPU downclocking','Performance','Safe'),
  ];
  static const _cats = ['All','Visual','Privacy','Performance','Gaming'];

  @override Widget build(BuildContext c) {
    final filtered = _items.where((t) =>
      (_filter == 'All' || t.cat == _filter) &&
      (_search.isEmpty || t.name.toLowerCase().contains(_search.toLowerCase()))
    ).toList();
    return Column(children: [
      Padding(padding: const EdgeInsets.fromLTRB(16, 0, 16, 8), child: TextField(
        decoration: InputDecoration(
          hintText: 'Search tweaks...', prefixIcon: const Icon(Icons.search),
          filled: true, fillColor: const Color(0xFF171717),
          border: OutlineInputBorder(borderRadius: BorderRadius.circular(10), borderSide: const BorderSide(color: Color(0xFF262626))),
        ),
        style: const TextStyle(fontSize: 13),
        onChanged: (v) => setState(() => _search = v),
      )),
      SizedBox(height: 36, child: ListView(scrollDirection: Axis.horizontal, padding: const EdgeInsets.symmetric(horizontal: 12), children: [
        ..._cats.map((c) => Padding(padding: const EdgeInsets.only(right: 6), child: ChoiceChip(
          label: Text(c, style: const TextStyle(fontSize: 12)),
          selected: _filter == c,
          onSelected: (_) => setState(() => _filter = c),
          selectedColor: const Color(0xFF0EA5E9), backgroundColor: const Color(0xFF171717),
          labelStyle: TextStyle(color: _filter == c ? Colors.white : Colors.grey),
          side: const BorderSide(color: Color(0xFF262626)),
        ))),
      ])),
      Expanded(child: ListView.builder(padding: const EdgeInsets.all(12), itemCount: filtered.length,
        itemBuilder: (_, i) {
          final t = filtered[i];
          final rc = t.risk == 'Safe' ? const Color(0xFF10B981) : const Color(0xFFEF4444);
          return Card(color: const Color(0xFF171717), margin: const EdgeInsets.only(bottom: 8),
            shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10), side: const BorderSide(color: Color(0xFF262626))),
            child: Padding(padding: const EdgeInsets.all(14), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              Row(children: [
                Expanded(child: Text(t.name, style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 14))),
                Container(padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                  decoration: BoxDecoration(color: rc.withOpacity(0.15), borderRadius: BorderRadius.circular(20)),
                  child: Text(t.risk, style: TextStyle(color: rc, fontSize: 11, fontWeight: FontWeight.w500))),
              ]),
              const SizedBox(height: 4), Text(t.desc, style: TextStyle(color: Colors.grey.shade400, fontSize: 12)),
              const SizedBox(height: 10),
              Row(mainAxisAlignment: MainAxisAlignment.end, children: [
                OutlinedButton(onPressed: () => ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('${t.name} applied!'))),
                  style: OutlinedButton.styleFrom(foregroundColor: const Color(0xFF0EA5E9), side: const BorderSide(color: Color(0xFF0EA5E9)),
                    padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 6), textStyle: const TextStyle(fontSize: 12)),
                  child: const Text('Apply')),
                const SizedBox(width: 8),
                TextButton(onPressed: () => ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('${t.name} reverted!'))),
                  style: TextButton.styleFrom(foregroundColor: Colors.grey, padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 6), textStyle: const TextStyle(fontSize: 12)),
                  child: const Text('Revert')),
              ]),
            ])));
        }),
      ),
    ]);
  }
}
class _Tk { final String name, desc, cat, risk; const _Tk(this.name, this.desc, this.cat, this.risk); }
