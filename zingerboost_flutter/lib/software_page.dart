import 'package:flutter/material.dart';

class SoftwarePage extends StatelessWidget {
  const SoftwarePage({super.key});

  static const _cats = ['All', 'Browsers', 'Media Players', 'Music', 'Gaming', 'Utilities', 'Development', 'Communication'];
  static const _apps = [
    _App(Icons.language, 'Chrome', 'Browsers'),
    _App(Icons.language, 'Brave', 'Browsers'),
    _App(Icons.language, 'Zen', 'Browsers'),
    _App(Icons.language, 'Arc', 'Browsers'),
    _App(Icons.language, 'Vivaldi', 'Browsers'),
    _App(Icons.language, 'Edge', 'Browsers'),
    _App(Icons.movie, 'VLC', 'Media Players'),
    _App(Icons.movie, 'Screenbox', 'Media Players'),
    _App(Icons.movie, 'PotPlayer', 'Media Players'),
    _App(Icons.music_note, 'Spotify', 'Music'),
    _App(Icons.music_note, 'Anghami', 'Music'),
    _App(Icons.music_note, 'Windows Media Player', 'Music'),
    _App(Icons.sports_esports, 'Steam', 'Gaming'),
    _App(Icons.sports_esports, 'Epic Games', 'Gaming'),
    _App(Icons.headset_mic, 'Discord', 'Communication'),
    _App(Icons.archive, '7-Zip', 'Utilities'),
    _App(Icons.code, 'Notepad++', 'Development'),
    _App(Icons.send, 'Telegram', 'Communication'),
  ];

  @override Widget build(BuildContext c) {
    return Column(children: [
      SizedBox(height: 38, child: ListView(scrollDirection: Axis.horizontal, padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4), children: [
        ..._cats.map((cat) => Padding(padding: const EdgeInsets.only(right: 6), child: ChoiceChip(
          label: Text(cat, style: const TextStyle(fontSize: 12)),
          selected: cat == 'All',
          onSelected: (_) => ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('Filter: $cat'))),
          selectedColor: const Color(0xFF0EA5E9), backgroundColor: const Color(0xFF171717),
          labelStyle: TextStyle(color: cat == 'All' ? Colors.white : Colors.grey),
          side: const BorderSide(color: Color(0xFF262626)),
        ))),
      ])),
      Expanded(child: Padding(
        padding: const EdgeInsets.all(12),
        child: GridView.builder(
          gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(crossAxisCount: 3, childAspectRatio: 0.85, crossAxisSpacing: 8, mainAxisSpacing: 8),
          itemCount: _apps.length,
          itemBuilder: (_, i) {
            final a = _apps[i];
            return Card(
              color: const Color(0xFF171717),
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12), side: const BorderSide(color: Color(0xFF262626))),
              child: Padding(padding: const EdgeInsets.all(10), child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
                Container(
                  padding: const EdgeInsets.all(8),
                  decoration: BoxDecoration(color: const Color(0xFF0EA5E9).withOpacity(0.1), borderRadius: BorderRadius.circular(10)),
                  child: Icon(a.icon, color: const Color(0xFF0EA5E9), size: 24)),
                const SizedBox(height: 6),
                Text(a.name, style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 11), textAlign: TextAlign.center, maxLines: 1, overflow: TextOverflow.ellipsis),
                const SizedBox(height: 2),
                Text(a.cat, style: TextStyle(color: Colors.grey.shade500, fontSize: 9)),
                const Spacer(),
                SizedBox(width: double.infinity, child: OutlinedButton(
                  onPressed: () => ScaffoldMessenger.of(c).showSnackBar(SnackBar(content: Text('Installing ${a.name}...'))),
                  style: OutlinedButton.styleFrom(
                    foregroundColor: const Color(0xFF0EA5E9), side: const BorderSide(color: Color(0xFF0EA5E9)),
                    padding: const EdgeInsets.symmetric(vertical: 4), textStyle: const TextStyle(fontSize: 10)),
                  child: const Text('Install')),
                ),
              ])),
            );
          },
        ),
      )),
    ]);
  }
}

class _App {
  final IconData icon;
  final String name, cat;
  const _App(this.icon, this.name, this.cat);
}
