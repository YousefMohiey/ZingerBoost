import '../models/metrics.dart';
import '../models/tweak.dart';
import '../models/software.dart';

class RustBridge {
  static Future<void> init() async {}

  static Future<SystemMetrics> pollMetrics() async => SystemMetrics();

  static Future<List<TweakMetadata>> listTweaks() async => [];

  static Future<void> applyTweak(String id) async {}

  static Future<void> revertTweak(String id) async {}

  static Future<TweakExplanation> explainTweak(String id) async => TweakExplanation(whatItDoes: '', whyItHelps: '', howToRevert: '');

  static Future<List<dynamic>> listSnapshots() async => [];

  static Future<void> restoreSnapshot(String id) async {}

  static Future<List<SoftwareInfo>> listBloatware() async => [];

  static Future<void> removeBloatware(String id) async {}

  static Future<void> removeAllBloatware() async {}

  static Future<List<SoftwareInfo>> listSoftware() async => [];

  static Future<void> installSoftware(String id) async {}

  static Future<Map<String, dynamic>> getSettings() async => {};

  static Future<void> saveSetting(String key, dynamic value) async {}

  static Future<String?> pickDirectory() async => null;

  static Future<void> exportSettings() async {}

  static Future<void> importSettings() async {}

  static Future<bool> checkForUpdates() async => false;

  static Future<void> resetAllTweaks() async {}
}
