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
    id: json['id'] ?? '',
    name: json['name'] ?? '',
    description: json['description'] ?? '',
    category: json['category'] ?? '',
    isProtected: json['is_protected'] ?? false,
  );
}
