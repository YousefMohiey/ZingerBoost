class SoftwareInfo {
  final String id;
  final String name;
  final String description;
  final String category;
  final bool isProtected;

  const SoftwareInfo({
    required this.id,
    required this.name,
    required this.description,
    required this.category,
    this.isProtected = false,
  });

  factory SoftwareInfo.fromJson(Map<String, dynamic> json) => SoftwareInfo(
        id: (json['winget_id'] ?? json['id'] ?? '').toString(),
        name: json['name'] ?? '',
        description: json['description'] ?? '',
        category: _normalizeCategory(json['category']),
        isProtected: json['is_protected'] ?? false,
      );

  static String _normalizeCategory(dynamic value) {
    final raw = (value ?? '').toString();
    switch (raw) {
      case 'Browsers':
        return 'browsers';
      case 'MediaPlayers':
        return 'media_players';
      case 'CloudStorage':
        return 'cloud_storage';
      case 'Bloatware':
        return 'bloatware';
      default:
        return raw
            .replaceAllMapped(
              RegExp(r'([a-z0-9])([A-Z])'),
              (m) => '${m[1]}_${m[2]}',
            )
            .toLowerCase();
    }
  }
}
