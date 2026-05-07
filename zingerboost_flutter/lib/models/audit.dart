class AuditEntry {
  final DateTime timestamp;
  final String level;
  final String category;
  final String message;
  final dynamic details;
  const AuditEntry({required this.timestamp, required this.level, required this.category, required this.message, this.details});
  factory AuditEntry.fromJson(Map<String, dynamic> json) => AuditEntry(timestamp: DateTime.parse(json['timestamp'] ?? DateTime.now().toIso8601String()), level: json['level'] ?? 'Info', category: json['category'] ?? '', message: json['message'] ?? '', details: json['details']);
}
