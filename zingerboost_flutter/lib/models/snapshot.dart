class SystemSnapshot {
  final String id;
  final DateTime createdAt;
  final String description;
  final List<AppliedTweakRecord> tweakRecords;
  const SystemSnapshot({required this.id, required this.createdAt, required this.description, required this.tweakRecords});
  factory SystemSnapshot.fromJson(Map<String, dynamic> json) => SystemSnapshot(id: json['id'] ?? '', createdAt: DateTime.parse(json['created_at'] ?? DateTime.now().toIso8601String()), description: json['description'] ?? '', tweakRecords: (json['tweak_records'] as List<dynamic>?)?.map((e) => AppliedTweakRecord.fromJson(e)).toList() ?? []);
}

class AppliedTweakRecord {
  final String tweakId;
  final dynamic snapshotData;
  const AppliedTweakRecord({required this.tweakId, required this.snapshotData});
  factory AppliedTweakRecord.fromJson(Map<String, dynamic> json) => AppliedTweakRecord(tweakId: json['tweak_id'] ?? '', snapshotData: json['snapshot_data']);
}
