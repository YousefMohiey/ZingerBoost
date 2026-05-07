import 'package:flutter/material.dart';

class AppSidebar extends StatefulWidget {
  const AppSidebar({super.key});

  @override
  State<AppSidebar> createState() => _AppSidebarState();
}

class _AppSidebarState extends State<AppSidebar> {
  int _selected = 0;

  void _navigate(int index) {
    setState(() => _selected = index);
    switch (index) {
      case 0:
        Navigator.pushNamedAndRemoveUntil(context, '/', (_) => false);
      case 1:
        Navigator.pushNamed(context, '/tweaks');
      case 2:
        Navigator.pushNamed(context, '/snapshots');
      case 3:
        Navigator.pushNamed(context, '/debloat');
      case 4:
        Navigator.pushNamed(context, '/software');
      case 5:
        Navigator.pushNamed(context, '/settings');
    }
  }

  static const _brand = Color(0xFF0EA5E9);

  static const _items = [
    _NavItem(Icons.dashboard, 'Dashboard'),
    _NavItem(Icons.tune, 'Tweaks'),
    _NavItem(Icons.history, 'Snapshots'),
    _NavItem(Icons.cleaning_services, 'Debloat'),
    _NavItem(Icons.apps, 'Software'),
    _NavItem(Icons.settings, 'Settings'),
  ];

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 220,
      color: const Color(0xFF0D0D0D),
      child: Column(
        children: [
          const SizedBox(height: 24),
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Container(
                width: 32,
                height: 32,
                decoration: BoxDecoration(
                  color: _brand.withOpacity(0.1),
                  borderRadius: BorderRadius.circular(8),
                ),
                child: const Icon(Icons.bolt, color: _brand, size: 20),
              ),
              const SizedBox(width: 10),
              const Text(
                'ZingerBoost',
                style: TextStyle(
                  fontSize: 17,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                ),
              ),
            ],
          ),
          const SizedBox(height: 32),
          ...List.generate(_items.length, (i) {
            final item = _items[i];
            final active = _selected == i;
            return Padding(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
              child: InkWell(
                borderRadius: BorderRadius.circular(8),
                onTap: () => _navigate(i),
                child: Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
                  decoration: BoxDecoration(
                    color: active ? _brand.withOpacity(0.1) : null,
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Row(
                    children: [
                      Icon(
                        item.icon,
                        size: 18,
                        color: active ? _brand : Colors.grey,
                      ),
                      const SizedBox(width: 12),
                      Text(
                        item.label,
                        style: TextStyle(
                          fontSize: 13,
                          fontWeight: active ? FontWeight.w600 : FontWeight.normal,
                          color: active ? Colors.white : Colors.grey,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            );
          }),
        ],
      ),
    );
  }
}

class _NavItem {
  final IconData icon;
  final String label;
  const _NavItem(this.icon, this.label);
}
