import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../models/metrics.dart';
import '../services/rust_bridge.dart';

class DashboardPage extends ConsumerStatefulWidget {
  const DashboardPage({super.key});

  @override
  ConsumerState<DashboardPage> createState() => _DashboardPageState();
}

class _DashboardPageState extends ConsumerState<DashboardPage> {
  late Timer _timer;
  late SystemMetrics _metrics;
  bool _loading = true;

  @override
  void initState() {
    super.initState();
    _metrics = SystemMetrics();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _poll();
      _timer = Timer.periodic(const Duration(seconds: 2), (_) => _poll());
    });
  }

  Future<void> _poll() async {
    try {
      final m = await RustBridge.pollMetrics();
      if (mounted) {
        setState(() {
          _metrics = m;
          _loading = false;
        });
      }
    } catch (_) {
      if (mounted) setState(() => _loading = false);
    }
  }

  @override
  void dispose() {
    _timer.cancel();
    super.dispose();
  }

  static const _bg = Color(0xFF0A0A0A);
  static const _card = Color(0xFF171717);
  static const _border = Color(0xFF262626);
  static const _brand = Color(0xFF0EA5E9);
  static const _red = Color(0xFFEF4444);
  static const _amber = Color(0xFFF59E0B);
  static const _green = Color(0xFF10B981);

  Color _valueColor(double pct) {
    if (pct > 80) return _red;
    if (pct > 50) return _amber;
    return _brand;
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
              child: ListView(
                children: [
                  const Text(
                    'Dashboard',
                    style: TextStyle(
                      fontSize: 28,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                  ),
                  const SizedBox(height: 24),
                  GridView.count(
                    crossAxisCount: 2,
                    shrinkWrap: true,
                    physics: const NeverScrollableScrollPhysics(),
                    crossAxisSpacing: 16,
                    mainAxisSpacing: 16,
                    childAspectRatio: 1.6,
                    children: [
                      _MetricCard(
                        icon: Icons.memory,
                        label: 'CPU',
                        value: '${_metrics.cpuPercent.toStringAsFixed(1)}%',
                        color: _valueColor(_metrics.cpuPercent),
                      ),
                      _MetricCard(
                        icon: Icons.storage,
                        label: 'RAM',
                        value:
                            '${_metrics.ramPercent.toStringAsFixed(1)}%\n${_metrics.ramUsedMb} / ${_metrics.ramTotalMb} MB',
                        color: _valueColor(_metrics.ramPercent),
                      ),
                      _MetricCard(
                        icon: Icons.disc_full,
                        label: 'Disk',
                        value:
                            '${_metrics.diskActivePercent.toStringAsFixed(1)}% active',
                        color: _valueColor(_metrics.diskActivePercent),
                      ),
                      _MetricCard(
                        icon: Icons.wifi,
                        label: 'Network',
                        value:
                            '↓ ${_metrics.networkDownMbps.toStringAsFixed(1)} Mbps\n↑ ${_metrics.networkUpMbps.toStringAsFixed(1)} Mbps',
                        color: _brand,
                      ),
                    ],
                  ),
                  const SizedBox(height: 24),
                  Container(
                    padding: const EdgeInsets.all(20),
                    decoration: BoxDecoration(
                      color: _card,
                      borderRadius: BorderRadius.circular(12),
                      border: Border.all(color: _border),
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            const Icon(Icons.lightbulb_outline,
                                color: _amber, size: 22),
                            const SizedBox(width: 10),
                            const Text(
                              'Recommended Actions',
                              style: TextStyle(
                                fontSize: 18,
                                fontWeight: FontWeight.w600,
                                color: Colors.white,
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 16),
                        if (_metrics.cpuPercent > 80)
                          _ActionItem(
                            icon: Icons.warning_amber_rounded,
                            color: _red,
                            text:
                                'High CPU usage detected. Consider closing resource-heavy applications or running a system cleanup.',
                          ),
                        if (_metrics.ramPercent > 80)
                          _ActionItem(
                            icon: Icons.memory,
                            color: _amber,
                            text:
                                'Memory usage is high. Try applying RAM-optimization tweaks or closing unused applications.',
                          ),
                        if (_metrics.cpuPercent <= 80 &&
                            _metrics.ramPercent <= 80)
                          const _ActionItem(
                            icon: Icons.check_circle,
                            color: _green,
                            text:
                                'System is running smoothly. No urgent actions needed.',
                          ),
                        const SizedBox(height: 12),
                        SizedBox(
                          width: double.infinity,
                          child: ElevatedButton.icon(
                            onPressed: () {},
                            icon: const Icon(Icons.auto_fix_high, size: 18),
                            label: const Text('Optimize Now'),
                            style: ElevatedButton.styleFrom(
                              backgroundColor: _brand,
                              foregroundColor: Colors.white,
                              padding:
                                  const EdgeInsets.symmetric(vertical: 12),
                              shape: RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(8),
                              ),
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
    );
  }
}

class _MetricCard extends StatelessWidget {
  final IconData icon;
  final String label;
  final String value;
  final Color color;

  const _MetricCard({
    required this.icon,
    required this.label,
    required this.value,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF171717),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: const Color(0xFF262626)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Row(
            children: [
              Icon(icon, color: color, size: 22),
              const SizedBox(width: 8),
              Text(
                label,
                style: TextStyle(
                  fontSize: 13,
                  fontWeight: FontWeight.w500,
                  color: Colors.grey[400],
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            value,
            style: TextStyle(
              fontSize: 22,
              fontWeight: FontWeight.bold,
              color: color,
              height: 1.3,
            ),
          ),
        ],
      ),
    );
  }
}

class _ActionItem extends StatelessWidget {
  final IconData icon;
  final Color color;
  final String text;

  const _ActionItem({
    required this.icon,
    required this.color,
    required this.text,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Icon(icon, color: color, size: 18),
          const SizedBox(width: 10),
          Expanded(
            child: Text(
              text,
              style: TextStyle(
                fontSize: 13,
                color: Colors.grey[300],
                height: 1.4,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
