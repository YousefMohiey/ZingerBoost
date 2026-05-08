import 'package:flutter/material.dart';

import '../models/tweak.dart';
import '../services/rust_bridge.dart';

class TweaksPage extends StatefulWidget {
  const TweaksPage({super.key});

  @override
  State<TweaksPage> createState() => _TweaksPageState();
}

class _TweaksPageState extends State<TweaksPage> {
  late List<TweakMetadata> _tweaks;
  late List<TweakMetadata> _filteredTweaks;
  late Set<String> _appliedIds;
  late Set<String> _expandedIds;
  late TextEditingController _searchCtrl;
  late Map<String, TweakExplanation> _explanations;
  String _selectedCategory = 'All';
  bool _loading = true;

  static const _bg = Color(0xFF0A0A0A);
  static const _card = Color(0xFF171717);
  static const _border = Color(0xFF262626);
  static const _brand = Color(0xFF0EA5E9);
  static const _green = Color(0xFF10B981);
  static const _amber = Color(0xFFF59E0B);
  static const _red = Color(0xFFEF4444);

  static const _categories = [
    'All',
    'Performance',
    'Privacy',
    'Visual',
    'Startup',
    'Debloat',
    'Gaming',
    'Network',
  ];

  @override
  void initState() {
    super.initState();
    _tweaks = [];
    _filteredTweaks = [];
    _appliedIds = {};
    _expandedIds = {};
    _explanations = {};
    _searchCtrl = TextEditingController();
    _searchCtrl.addListener(_applyFilters);
    WidgetsBinding.instance.addPostFrameCallback((_) => _loadTweaks());
  }

  Future<void> _loadTweaks() async {
    try {
      final tweaks = await RustBridge.listTweaks();
      if (mounted) {
        setState(() {
          _tweaks = tweaks;
          _loading = false;
        });
        _applyFilters();
      }
    } catch (_) {
      if (mounted) setState(() => _loading = false);
    }
  }

  void _applyFilters() {
    final query = _searchCtrl.text.toLowerCase();
    setState(() {
      _filteredTweaks = _tweaks.where((t) {
        final matchCat =
            _selectedCategory == 'All' || t.category == _selectedCategory;
        final matchSearch = t.name.toLowerCase().contains(query) ||
            t.description.toLowerCase().contains(query);
        return matchCat && matchSearch;
      }).toList();
    });
  }

  Color _riskColor(String risk) {
    switch (risk) {
      case 'Safe':
        return _green;
      case 'Moderate':
        return _amber;
      case 'Advanced':
        return _red;
      default:
        return _green;
    }
  }

  Future<void> _applyTweak(String id) async {
    try {
      await RustBridge.applyTweak(id);
      if (mounted) {
        setState(() => _appliedIds.add(id));
      }
    } catch (_) {}
  }

  Future<void> _revertTweak(String id) async {
    try {
      await RustBridge.revertTweak(id);
      if (mounted) {
        setState(() => _appliedIds.remove(id));
      }
    } catch (_) {}
  }

  Future<void> _toggleDetails(String id) async {
    if (_expandedIds.contains(id)) {
      setState(() => _expandedIds.remove(id));
    } else {
      setState(() => _expandedIds.add(id));
      if (!_explanations.containsKey(id)) {
        try {
          final exp = await RustBridge.explainTweak(id);
          if (mounted) {
            setState(() => _explanations[id] = exp);
          }
        } catch (_) {}
      }
    }
  }

  IconData _tweakIcon(String category) {
    switch (category) {
      case 'Performance':
        return Icons.speed;
      case 'Privacy':
        return Icons.shield;
      case 'Visual':
        return Icons.palette;
      case 'Startup':
        return Icons.rocket_launch;
      case 'Debloat':
        return Icons.delete_sweep;
      case 'Gaming':
        return Icons.sports_esports;
      case 'Network':
        return Icons.language;
      case 'Security':
        return Icons.lock;
      default:
        return Icons.tune;
    }
  }

  @override
  void dispose() {
    _searchCtrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: _bg,
      body: _loading
          ? const Center(
              child: CircularProgressIndicator(color: _brand),
            )
          : Column(
              children: [
                _buildSearchBar(),
                _buildCategoryChips(),
                Expanded(child: _buildTweakList()),
              ],
            ),
    );
  }

  Widget _buildSearchBar() {
    return Padding(
      padding: const EdgeInsets.fromLTRB(24, 24, 24, 12),
      child: Row(
        children: [
          const Expanded(
            child: Text(
              'Tweaks',
              style: TextStyle(
                fontSize: 28,
                fontWeight: FontWeight.bold,
                color: Colors.white,
              ),
            ),
          ),
          SizedBox(
            width: 280,
            child: TextField(
              controller: _searchCtrl,
              style: const TextStyle(color: Colors.white, fontSize: 14),
              decoration: InputDecoration(
                hintText: 'Search tweaks...',
                hintStyle: TextStyle(color: Colors.grey[500]),
                prefixIcon:
                    const Icon(Icons.search, color: _brand, size: 20),
                filled: true,
                fillColor: _card,
                contentPadding:
                    const EdgeInsets.symmetric(vertical: 10, horizontal: 16),
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(10),
                  borderSide: const BorderSide(color: _border),
                ),
                enabledBorder: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(10),
                  borderSide: const BorderSide(color: _border),
                ),
                focusedBorder: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(10),
                  borderSide: const BorderSide(color: _brand),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildCategoryChips() {
    return SizedBox(
      height: 40,
      child: ListView.separated(
        scrollDirection: Axis.horizontal,
        padding: const EdgeInsets.symmetric(horizontal: 24),
        itemCount: _categories.length,
        separatorBuilder: (_, __) => const SizedBox(width: 8),
        itemBuilder: (context, index) {
          final cat = _categories[index];
          final selected = _selectedCategory == cat;
          return ChoiceChip(
            label: Text(cat),
            selected: selected,
            onSelected: (_) {
              setState(() => _selectedCategory = cat);
              _applyFilters();
            },
            backgroundColor: _card,
            selectedColor: _brand,
            labelStyle: TextStyle(
              color: selected ? Colors.white : Colors.grey[400],
              fontSize: 12,
              fontWeight: FontWeight.w500,
            ),
            side: BorderSide(
              color: selected ? _brand : _border,
            ),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
          );
        },
      ),
    );
  }

  Widget _buildTweakList() {
    if (_filteredTweaks.isEmpty) {
      return Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.search_off, size: 48, color: Colors.grey[600]),
            const SizedBox(height: 12),
            Text(
              'No tweaks found',
              style: TextStyle(fontSize: 16, color: Colors.grey[500]),
            ),
          ],
        ),
      );
    }

    return ListView.builder(
      padding: const EdgeInsets.all(24),
      itemCount: _filteredTweaks.length,
      itemBuilder: (context, index) {
        final tweak = _filteredTweaks[index];
        final isApplied = _appliedIds.contains(tweak.id);
        final isExpanded = _expandedIds.contains(tweak.id);
        final explanation = _explanations[tweak.id];

        return Card(
          color: _card,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
            side: const BorderSide(color: _border),
          ),
          margin: const EdgeInsets.only(bottom: 12),
          child: Padding(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Container(
                      width: 40,
                      height: 40,
                      decoration: BoxDecoration(
                        color: _brand.withOpacity(0.1),
                        borderRadius: BorderRadius.circular(10),
                      ),
                      child: Icon(
                        _tweakIcon(tweak.category),
                        color: _brand,
                        size: 20,
                      ),
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            tweak.name,
                            style: const TextStyle(
                              fontSize: 15,
                              fontWeight: FontWeight.w600,
                              color: Colors.white,
                            ),
                          ),
                          const SizedBox(height: 2),
                          Text(
                            tweak.description,
                            style: TextStyle(
                              fontSize: 12,
                              color: Colors.grey[400],
                            ),
                            maxLines: 2,
                            overflow: TextOverflow.ellipsis,
                          ),
                        ],
                      ),
                    ),
                    Container(
                      padding: const EdgeInsets.symmetric(
                          horizontal: 10, vertical: 4),
                      decoration: BoxDecoration(
                        color: _riskColor(tweak.risk).withOpacity(0.15),
                        borderRadius: BorderRadius.circular(6),
                      ),
                      child: Text(
                        tweak.risk,
                        style: TextStyle(
                          fontSize: 11,
                          fontWeight: FontWeight.w600,
                          color: _riskColor(tweak.risk),
                        ),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 12),
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    TextButton.icon(
                      onPressed: () => _toggleDetails(tweak.id),
                      icon: Icon(
                        isExpanded
                            ? Icons.expand_less
                            : Icons.expand_more,
                        size: 18,
                      ),
                      label: Text(
                          isExpanded ? 'Hide Details' : 'Details'),
                      style: TextButton.styleFrom(
                        foregroundColor: Colors.grey[400],
                        padding: const EdgeInsets.symmetric(
                            horizontal: 12, vertical: 6),
                      ),
                    ),
                    const SizedBox(width: 8),
                    if (isApplied)
                      ElevatedButton.icon(
                        onPressed: () => _revertTweak(tweak.id),
                        icon: const Icon(Icons.undo, size: 16),
                        label: const Text('Revert'),
                        style: ElevatedButton.styleFrom(
                          backgroundColor: _amber,
                          foregroundColor: Colors.black,
                          padding: const EdgeInsets.symmetric(
                              horizontal: 16, vertical: 8),
                          shape: RoundedRectangleBorder(
                            borderRadius: BorderRadius.circular(8),
                          ),
                        ),
                      )
                    else
                      ElevatedButton.icon(
                        onPressed: () => _applyTweak(tweak.id),
                        icon: const Icon(Icons.check, size: 16),
                        label: const Text('Apply'),
                        style: ElevatedButton.styleFrom(
                          backgroundColor: _green,
                          foregroundColor: Colors.white,
                          padding: const EdgeInsets.symmetric(
                              horizontal: 16, vertical: 8),
                          shape: RoundedRectangleBorder(
                            borderRadius: BorderRadius.circular(8),
                          ),
                        ),
                      ),
                  ],
                ),
                if (isExpanded) ...[
                  const Divider(color: _border, height: 24),
                  if (explanation != null)
                    _buildExplanation(explanation)
                  else
                    const Padding(
                      padding: EdgeInsets.symmetric(vertical: 12),
                      child: SizedBox(
                        width: 20,
                        height: 20,
                        child: CircularProgressIndicator(
                          strokeWidth: 2,
                          color: _brand,
                        ),
                      ),
                    ),
                ],
              ],
            ),
          ),
        );
      },
    );
  }

  Widget _buildExplanation(TweakExplanation exp) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _explanationRow(Icons.info_outline, 'What it does:', exp.whatItDoes),
        const SizedBox(height: 8),
        _explanationRow(Icons.trending_up, 'Why it helps:', exp.whyItHelps),
        if (exp.potentialRisks != null) ...[
          const SizedBox(height: 8),
          _explanationRow(
              Icons.warning_amber_outlined, 'Risks:', exp.potentialRisks!),
        ],
        const SizedBox(height: 8),
        _explanationRow(Icons.restore, 'How to revert:', exp.howToRevert),
      ],
    );
  }

  Widget _explanationRow(IconData icon, String label, String text) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Icon(icon, size: 14, color: _brand),
        const SizedBox(width: 6),
        Text(
          label,
          style: const TextStyle(
            fontSize: 11,
            fontWeight: FontWeight.w600,
            color: Colors.white70,
          ),
        ),
        const SizedBox(width: 6),
        Expanded(
          child: Text(
            text,
            style: TextStyle(fontSize: 11, color: Colors.grey[400]),
          ),
        ),
      ],
    );
  }
}
