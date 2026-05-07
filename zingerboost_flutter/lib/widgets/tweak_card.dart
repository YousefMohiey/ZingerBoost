import 'package:flutter/material.dart';

import '../models/tweak.dart';
import 'risk_badge.dart';

const _surface = Color(0xFF171717);
const _border = Color(0xFF262626);
const _brand = Color(0xFF0EA5E9);

class TweakCard extends StatefulWidget {
  final TweakMetadata tweak;
  final VoidCallback? onApply;
  final VoidCallback? onRevert;
  final bool isApplying;
  final bool isReverting;
  final TweakExplanation? explanation;

  const TweakCard({
    super.key,
    required this.tweak,
    this.onApply,
    this.onRevert,
    this.isApplying = false,
    this.isReverting = false,
    this.explanation,
  });

  @override
  State<TweakCard> createState() => _TweakCardState();
}

class _TweakCardState extends State<TweakCard> {
  bool _expanded = false;

  @override
  Widget build(BuildContext context) {
    final tweak = widget.tweak;
    final explanation = widget.explanation;
    final isBusy = widget.isApplying || widget.isReverting;

    return Container(
      margin: const EdgeInsets.symmetric(vertical: 4),
      decoration: BoxDecoration(
        color: _surface,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: _border),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          InkWell(
            borderRadius: BorderRadius.circular(12),
            onTap: () => setState(() => _expanded = !_expanded),
            child: Padding(
              padding: const EdgeInsets.all(14),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Container(
                        width: 36,
                        height: 36,
                        decoration: BoxDecoration(
                          color: _brand.withValues(alpha: 0.12),
                          borderRadius: BorderRadius.circular(8),
                        ),
                        child: const Icon(
                          Icons.tune,
                          size: 20,
                          color: _brand,
                        ),
                      ),
                      const SizedBox(width: 10),
                      Expanded(
                        child: Text(
                          tweak.name,
                          style: const TextStyle(
                            fontSize: 15,
                            fontWeight: FontWeight.w600,
                            color: Colors.white,
                          ),
                        ),
                      ),
                      const SizedBox(width: 8),
                      RiskBadge(risk: tweak.risk),
                    ],
                  ),
                  const SizedBox(height: 8),
                  Text(
                    tweak.description,
                    style: TextStyle(
                      fontSize: 13,
                      color: Colors.grey.shade400,
                      height: 1.4,
                    ),
                    maxLines: _expanded ? null : 2,
                    overflow:
                        _expanded ? TextOverflow.visible : TextOverflow.ellipsis,
                  ),
                  const SizedBox(height: 10),
                  Row(
                    children: [
                      _ActionButton(
                        label: 'Apply',
                        icon: Icons.check_circle_outline,
                        loading: widget.isApplying,
                        disabled: isBusy,
                        onTap: widget.onApply,
                      ),
                      const SizedBox(width: 8),
                      _ActionButton(
                        label: 'Revert',
                        icon: Icons.undo,
                        loading: widget.isReverting,
                        disabled: isBusy,
                        onTap: widget.onRevert,
                      ),
                      const Spacer(),
                      if (explanation != null)
                        Icon(
                          _expanded
                              ? Icons.expand_less
                              : Icons.expand_more,
                          color: Colors.grey.shade600,
                          size: 20,
                        ),
                    ],
                  ),
                ],
              ),
            ),
          ),
          if (_expanded && explanation != null) _buildExplanation(explanation),
        ],
      ),
    );
  }

  Widget _buildExplanation(TweakExplanation explanation) {
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.fromLTRB(14, 0, 14, 14),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Divider(color: _border, height: 1),
          const SizedBox(height: 12),
          _section('What it does', explanation.whatItDoes),
          const SizedBox(height: 10),
          _section('Why it helps', explanation.whyItHelps),
          if (explanation.potentialRisks != null) ...[
            const SizedBox(height: 10),
            _section('Potential risks', explanation.potentialRisks!),
          ],
          const SizedBox(height: 10),
          _section('How to revert', explanation.howToRevert),
        ],
      ),
    );
  }

  Widget _section(String title, String body) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          title,
          style: TextStyle(
            fontSize: 11,
            fontWeight: FontWeight.w700,
            color: Colors.grey.shade500,
            letterSpacing: 0.5,
          ),
        ),
        const SizedBox(height: 3),
        Text(
          body,
          style: TextStyle(fontSize: 13, color: Colors.grey.shade300, height: 1.4),
        ),
      ],
    );
  }
}

class _ActionButton extends StatelessWidget {
  final String label;
  final IconData icon;
  final bool loading;
  final bool disabled;
  final VoidCallback? onTap;

  const _ActionButton({
    required this.label,
    required this.icon,
    this.loading = false,
    this.disabled = false,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final isEnabled = !disabled && !loading;

    return Material(
      color: Colors.transparent,
      child: InkWell(
        borderRadius: BorderRadius.circular(6),
        onTap: isEnabled ? onTap : null,
        child: Container(
          padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 7),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(6),
            border: Border.all(
              color: isEnabled ? _border : _border.withValues(alpha: 0.4),
            ),
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              if (loading)
                const SizedBox(
                  width: 14,
                  height: 14,
                  child: CircularProgressIndicator(
                    strokeWidth: 2,
                    color: _brand,
                  ),
                )
              else
                Icon(icon, size: 16, color: isEnabled ? Colors.white : Colors.grey.shade700),
              const SizedBox(width: 5),
              Text(
                label,
                style: TextStyle(
                  fontSize: 13,
                  fontWeight: FontWeight.w500,
                  color: isEnabled ? Colors.white : Colors.grey.shade700,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
