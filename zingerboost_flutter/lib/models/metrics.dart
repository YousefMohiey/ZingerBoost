class SystemMetrics {
  final double cpuPercent;
  final double ramPercent;
  final int ramUsedMb;
  final int ramTotalMb;
  final double diskActivePercent;
  final double networkDownMbps;
  final double networkUpMbps;
  const SystemMetrics({this.cpuPercent = 0, this.ramPercent = 0, this.ramUsedMb = 0, this.ramTotalMb = 0, this.diskActivePercent = 0, this.networkDownMbps = 0, this.networkUpMbps = 0});
  factory SystemMetrics.fromJson(Map<String, dynamic> json) => SystemMetrics(cpuPercent: (json['cpu_percent'] ?? 0).toDouble(), ramPercent: (json['ram_percent'] ?? 0).toDouble(), ramUsedMb: json['ram_used_mb'] ?? 0, ramTotalMb: json['ram_total_mb'] ?? 0, diskActivePercent: (json['disk_active_percent'] ?? 0).toDouble(), networkDownMbps: (json['network_down_mbps'] ?? 0).toDouble(), networkUpMbps: (json['network_up_mbps'] ?? 0).toDouble());
}
