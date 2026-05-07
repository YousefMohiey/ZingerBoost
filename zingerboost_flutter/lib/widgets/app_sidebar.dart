import 'package:flutter/material.dart';

class AppSidebar extends StatelessWidget {
  final int selectedIndex;
  final Function(int) onSelect;

  const AppSidebar({super.key, required this.selectedIndex, required this.onSelect});

  static const _items = [
    ('Dashboard', Icons.dashboard),
    ('Tweaks', Icons.tune),
    ('Snapshots', Icons.history),
    ('Debloat', Icons.delete),
    ('Software', Icons.download),
    ('Services', Icons.settings),
    ('Cleaner', Icons.cleaning_services),
    ('Settings', Icons.settings_applications),
  ];

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 220,
      color: const Color(0xFF171717),
      child: Column(
        children: [
          const SizedBox(height: 16),
          Row(children: [
            const SizedBox(width: 16),
            Container(
              padding: const EdgeInsets.all(8),
              decoration: BoxDecoration(
                color: const Color(0xFF0EA5E9).withOpacity(0.15),
                borderRadius: BorderRadius.circular(8),
              ),
              child: const Icon(Icons.shield, color: Color(0xFF0EA5E9), size: 22),
            ),
            const SizedBox(width: 12),
            const Text('ZingerBoost', style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 16)),
          ]),
          const SizedBox(height: 20),
          Expanded(
            child: ListView(
              children: List.generate(_items.length, (i) {
                final selected = i == selectedIndex;
                return Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                  child: Material(
                    color: selected ? const Color(0xFF0EA5E9).withOpacity(0.15) : Colors.transparent,
                    borderRadius: BorderRadius.circular(8),
                    child: InkWell(
                      borderRadius: BorderRadius.circular(8),
                      onTap: () => onSelect(i),
                      child: Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
                        child: Row(
                          children: [
                            Icon(_items[i].$2, size: 20, color: selected ? const Color(0xFF0EA5E9) : Colors.grey),
                            const SizedBox(width: 12),
                            Text(_items[i].$1, style: TextStyle(fontSize: 13, fontWeight: FontWeight.w500, color: selected ? const Color(0xFF0EA5E9) : Colors.grey)),
                          ],
                        ),
                      ),
                    ),
                  ),
                );
              }),
            ),
          ),
          const SizedBox(height: 8),
          Padding(
            padding: const EdgeInsets.all(16),
            child: Row(
              children: [
                Icon(Icons.admin_panel_settings, size: 14, color: Colors.green.withOpacity(0.7)),
                const SizedBox(width: 8),
                Text('Admin', style: TextStyle(fontSize: 11, color: Colors.green.withOpacity(0.7))),
              ],
            ),
          ),
          const SizedBox(height: 8),
        ],
      ),
    );
  }
}
