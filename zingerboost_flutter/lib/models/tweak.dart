class TweakMetadata {
  final String id;
  final String name;
  final String description;
  final String category;
  final String risk;
  final bool requiresReboot;
  final bool requiresAdmin;
  final List<String> affectedKeys;
  final String? sourceUrl;

  const TweakMetadata({
    required this.id,
    required this.name,
    required this.description,
    required this.category,
    required this.risk,
    required this.requiresReboot,
    required this.requiresAdmin,
    required this.affectedKeys,
    this.sourceUrl,
  });

  factory TweakMetadata.fromJson(Map<String, dynamic> json) => TweakMetadata(
    id: json['id'] ?? '',
    name: json['name'] ?? '',
    description: json['description'] ?? '',
    category: json['category'] ?? '',
    risk: json['risk'] ?? 'Safe',
    requiresReboot: json['requires_reboot'] ?? false,
    requiresAdmin: json['requires_admin'] ?? false,
    affectedKeys: (json['affected_keys'] as List<dynamic>?)?.map((e) => e.toString()).toList() ?? [],
    sourceUrl: json['source_url'],
  );
}

class TweakResult {
  final bool rebootRequired;
  final String message;
  const TweakResult({required this.rebootRequired, required this.message});
  factory TweakResult.fromJson(Map<String, dynamic> json) => TweakResult(rebootRequired: json['reboot_required'] ?? false, message: json['message'] ?? '');
}

class TweakExplanation {
  final String whatItDoes;
  final String whyItHelps;
  final String? potentialRisks;
  final String howToRevert;
  const TweakExplanation({required this.whatItDoes, required this.whyItHelps, this.potentialRisks, required this.howToRevert});
  factory TweakExplanation.fromJson(Map<String, dynamic> json) => TweakExplanation(whatItDoes: json['what_it_does'] ?? '', whyItHelps: json['why_it_helps'] ?? '', potentialRisks: json['potential_risks'], howToRevert: json['how_to_revert'] ?? '');
}
