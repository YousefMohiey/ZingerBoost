import 'dart:convert';
import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';

import '../models/metrics.dart';
import '../models/software.dart';
import '../models/tweak.dart';

typedef _InitNative = Int32 Function();
typedef _InitDart = int Function();
typedef _Call0Native = Pointer<Utf8> Function();
typedef _Call0Dart = Pointer<Utf8> Function();
typedef _Call1Native = Pointer<Utf8> Function(Pointer<Utf8>);
typedef _Call1Dart = Pointer<Utf8> Function(Pointer<Utf8>);
typedef _FreeNative = Void Function(Pointer<Utf8>);
typedef _FreeDart = void Function(Pointer<Utf8>);

class RustBridge {
  static DynamicLibrary? _lib;
  static _FreeDart? _free;
  static bool _initialized = false;
  static Object? _loadError;

  static bool get isAvailable => _lib != null && _initialized;
  static Object? get loadError => _loadError;

  static Future<void> init() async {
    if (_initialized) return;

    try {
      _lib = _openLibrary();
      _free = _lib!
          .lookupFunction<_FreeNative, _FreeDart>('zingerboost_free_string');
      final initApp = _lib!.lookupFunction<_InitNative, _InitDart>('init_app');
      final result = initApp();
      if (result != 0) {
        throw StateError('Rust init_app failed with code $result');
      }
      _initialized = true;
      _loadError = null;
    } catch (e) {
      _lib = null;
      _free = null;
      _initialized = false;
      _loadError = e;
    }
  }

  static DynamicLibrary _openLibrary() {
    if (Platform.isWindows) {
      final executableDir = File(Platform.resolvedExecutable).parent.path;
      final candidates = [
        '$executableDir\\zingerboost_core.dll',
        '$executableDir\\lib\\zingerboost_core.dll',
        'zingerboost_core.dll',
      ];
      for (final path in candidates) {
        try {
          return DynamicLibrary.open(path);
        } catch (_) {}
      }
      return DynamicLibrary.open('zingerboost_core.dll');
    }

    if (Platform.isMacOS) {
      return DynamicLibrary.open('libzingerboost_core.dylib');
    }

    return DynamicLibrary.open('libzingerboost_core.so');
  }

  static Future<SystemMetrics> pollMetrics() async {
    final json = _call0('zingerboost_get_metrics');
    if (json == null || json.isEmpty) return const SystemMetrics();
    return SystemMetrics.fromJson(_decodeObject(json));
  }

  static Future<List<TweakMetadata>> listTweaks() async {
    final json = _call0('zingerboost_list_tweaks');
    final list = _decodeList(json);
    return list
        .whereType<Map<String, dynamic>>()
        .map(TweakMetadata.fromJson)
        .toList();
  }

  static Future<void> applyTweak(String id) async {
    _throwOnError(_call1('zingerboost_apply_tweak', id));
  }

  static Future<void> revertTweak(String id) async {
    _throwOnError(_call1('zingerboost_revert_tweak', id));
  }

  static Future<TweakExplanation> explainTweak(String id) async {
    final json = _call1('zingerboost_get_tweak_explanation', id);
    return TweakExplanation.fromJson(_decodeObject(json));
  }

  static Future<List<dynamic>> listSnapshots() async {
    return _decodeList(_call0('zingerboost_list_snapshots'));
  }

  static Future<void> restoreSnapshot(String id) async {
    _throwOnError(_call1('zingerboost_restore_snapshot', id));
  }

  static Future<List<SoftwareInfo>> listBloatware() async {
    final payload = _decodeObject(_call0('zingerboost_list_bloatware'));
    final bloatware = (payload['bloatware'] as List<dynamic>? ?? [])
        .whereType<Map<String, dynamic>>()
        .map(SoftwareInfo.fromJson)
        .toList();
    final protectedApps = (payload['protected'] as List<dynamic>? ?? [])
        .map((name) => SoftwareInfo(
              id: name.toString(),
              name: name.toString(),
              description: 'Protected system component',
              category: 'bloatware',
              isProtected: true,
            ))
        .toList();
    return [...bloatware, ...protectedApps];
  }

  static Future<void> removeBloatware(String id) async {
    _throwOnError(_call1('zingerboost_remove_bloatware', jsonEncode([id])));
  }

  static Future<void> removeAllBloatware() async {
    final apps = await listBloatware();
    final ids = apps.where((app) => !app.isProtected).map((app) => app.id);
    _throwOnError(_call1('zingerboost_remove_bloatware', jsonEncode(ids.toList())));
  }

  static Future<List<SoftwareInfo>> listSoftware() async {
    final list = _decodeList(_call0('zingerboost_list_software'));
    return list
        .whereType<Map<String, dynamic>>()
        .map(SoftwareInfo.fromJson)
        .toList();
  }

  static Future<void> installSoftware(String id) async {
    _throwOnError(_call1('zingerboost_install_software', id));
  }

  static Future<List<Map<String, dynamic>>> listServices() async {
    final list = _decodeList(_call0('zingerboost_list_services'));
    return list.whereType<Map<String, dynamic>>().toList();
  }

  static Future<void> stopService(String name) async {
    _throwOnError(_call1('zingerboost_stop_service', name));
  }

  static Future<void> disableService(String name) async {
    _throwOnError(_call1('zingerboost_disable_service', name));
  }

  static Future<List<Map<String, dynamic>>> scanCleaner() async {
    final list = _decodeList(_call0('zingerboost_scan_cleaner'));
    return list.whereType<Map<String, dynamic>>().toList();
  }

  static Future<void> runCleaner(String category) async {
    _throwOnError(_call1('zingerboost_run_cleaner', category));
  }

  static Future<Map<String, dynamic>> getSettings() async => {};

  static Future<void> saveSetting(String key, dynamic value) async {}

  static Future<String?> pickDirectory() async => null;

  static Future<void> exportSettings() async {}

  static Future<void> importSettings() async {}

  static Future<bool> checkForUpdates() async => false;

  static Future<void> resetAllTweaks() async {}

  static String? _call0(String symbol) {
    final lib = _lib;
    if (lib == null) return null;
    final fn = lib.lookupFunction<_Call0Native, _Call0Dart>(symbol);
    return _readRustString(fn());
  }

  static String? _call1(String symbol, String arg) {
    final lib = _lib;
    if (lib == null) return null;
    final fn = lib.lookupFunction<_Call1Native, _Call1Dart>(symbol);
    final nativeArg = arg.toNativeUtf8();
    try {
      return _readRustString(fn(nativeArg));
    } finally {
      malloc.free(nativeArg);
    }
  }

  static String? _readRustString(Pointer<Utf8> ptr) {
    if (ptr == nullptr) return null;
    try {
      return ptr.toDartString();
    } finally {
      _free?.call(ptr);
    }
  }

  static Map<String, dynamic> _decodeObject(String? raw) {
    if (raw == null || raw.isEmpty) return {};
    final decoded = jsonDecode(raw);
    if (decoded is Map<String, dynamic>) return decoded;
    if (decoded is Map) {
      return decoded.map((key, value) => MapEntry(key.toString(), value));
    }
    return {};
  }

  static List<dynamic> _decodeList(String? raw) {
    if (raw == null || raw.isEmpty) return [];
    final decoded = jsonDecode(raw);
    if (decoded is List) return decoded;
    return [];
  }

  static void _throwOnError(String? raw) {
    final response = _decodeObject(raw);
    if (response['error'] != null) {
      throw StateError(response['error'].toString());
    }
    if (response['success'] == false) {
      throw StateError((response['message'] ?? 'Operation failed').toString());
    }
  }
}
