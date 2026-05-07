import 'package:flutter/material.dart';

const _safe = Color(0xFF10B981);
const _moderate = Color(0xFFF59E0B);
const _advanced = Color(0xFFEF4444);

class RiskBadge extends StatelessWidget {
  final String risk;

  const RiskBadge({super.key, required this.risk});

  Color _color() {
    switch (risk.toLowerCase()) {
      case 'safe':
        return _safe;
      case 'moderate':
        return _moderate;
      case 'advanced':
        return _advanced;
      default:
        return Colors.grey;
    }
  }

  double _opacity() {
    switch (risk.toLowerCase()) {
      case 'safe':
        return 0.20;
      case 'moderate':
        return 0.18;
      case 'advanced':
        return 0.18;
      default:
        return 0.15;
    }
  }

  @override
  Widget build(BuildContext context) {
    final color = _color();
    final bgOpacity = _opacity();

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
      decoration: BoxDecoration(
        color: color.withValues(alpha: bgOpacity),
        borderRadius: BorderRadius.circular(6),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: 7,
            height: 7,
            decoration: BoxDecoration(color: color, shape: BoxShape.circle),
          ),
          const SizedBox(width: 5),
          Text(
            risk,
            style: TextStyle(
              fontSize: 12,
              fontWeight: FontWeight.w600,
              color: color,
            ),
          ),
        ],
      ),
    );
  }
}
