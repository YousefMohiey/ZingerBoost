import 'package:flutter/material.dart';
import '../services/rust_bridge.dart';

class MaintenancePage extends StatefulWidget {
  const MaintenancePage({super.key});

  @override
  State<MaintenancePage> createState() => _MaintenancePageState();
}

class _MaintenancePageState extends State<MaintenancePage> with SingleTickerProviderStateMixin {
  late TabController _tabController;
  List<Map<String, dynamic>> _services = [];
  List<Map<String, dynamic>> _cleanCategories = [];
  bool _loadingServices = true;
  bool _loadingCleaner = true;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 2, vsync: this);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _loadServices();
      _loadCleaner();
    });
  }

  Future<void> _loadServices() async {
    try {
      final services = await RustBridge.listServices();
      if (mounted) {
        setState(() {
          _services = services;
          _loadingServices = false;
        });
      }
    } catch (_) {
      if (mounted) setState(() => _loadingServices = false);
    }
  }

  Future<void> _loadCleaner() async {
    try {
      final categories = await RustBridge.scanCleaner();
      if (mounted) {
        setState(() {
          _cleanCategories = categories;
          _loadingCleaner = false;
        });
      }
    } catch (_) {
      if (mounted) setState(() => _loadingCleaner = false);
    }
  }

  Future<void> _stopService(String name) async {
    try {
      await RustBridge.stopService(name);
      await _loadServices();
    } catch (_) {}
  }

  Future<void> _disableService(String name) async {
    try {
      await RustBridge.disableService(name);
      await _loadServices();
    } catch (_) {}
  }

  Future<void> _runCleaner(String id) async {
    try {
      await RustBridge.runCleaner(id);
      await _loadCleaner();
    } catch (_) {}
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final surfaceColor = Color(0xFF171717);
    final borderColor = Color(0xFF262626);

    return Scaffold(
      backgroundColor: Color(0xFF0A0A0A),
      appBar: AppBar(
        title: Text('Maintenance', style: TextStyle(color: Colors.white, fontSize: 20, fontWeight: FontWeight.bold)),
        backgroundColor: Color(0xFF0A0A0A),
        bottom: TabBar(
          controller: _tabController,
          labelColor: Color(0xFF0EA5E9),
          unselectedLabelColor: Colors.grey,
          indicatorColor: Color(0xFF0EA5E9),
          tabs: const [
            Tab(text: 'Services', icon: Icon(Icons.settings, size: 18)),
            Tab(text: 'Cleaner', icon: Icon(Icons.cleaning_services, size: 18)),
          ],
        ),
      ),
      body: TabBarView(
        controller: _tabController,
        children: [
          _buildServicesTab(surfaceColor, borderColor),
          _buildCleanerTab(surfaceColor, borderColor),
        ],
      ),
    );
  }

  Widget _buildServicesTab(Color surface, Color border) {
    return Column(
      children: [
        _buildWarningBanner('Services shown here are safe to disable. System-critical services are hidden.'),
        Expanded(
          child: _loadingServices
              ? Center(child: CircularProgressIndicator(color: Color(0xFF0EA5E9)))
              : _services.isEmpty
                  ? _buildEmpty('No services to display', 'Services data will appear here', Icons.settings)
                  : ListView.builder(
                      padding: EdgeInsets.all(12),
                      itemCount: _services.length,
                      itemBuilder: (_, i) => _buildServiceCard(_services[i], surface, border),
                    ),
        ),
      ],
    );
  }

  Widget _buildServiceCard(Map<String, dynamic> svc, Color surface, Color border) {
    return Container(
      margin: EdgeInsets.only(bottom: 8),
      padding: EdgeInsets.all(14),
      decoration: BoxDecoration(color: surface, borderRadius: BorderRadius.circular(10), border: Border.all(color: border)),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Expanded(child: Text(svc['display_name'] ?? svc['name'] ?? '', style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 14))),
              Container(padding: EdgeInsets.symmetric(horizontal: 8, vertical: 3), decoration: BoxDecoration(color: svc['status'] == 'Running' ? Color(0xFF10B981).withOpacity(0.2) : Colors.grey.withOpacity(0.2), borderRadius: BorderRadius.circular(20)), child: Text(svc['status'] ?? '', style: TextStyle(fontSize: 11, color: svc['status'] == 'Running' ? Color(0xFF10B981) : Colors.grey))),
            ],
          ),
          SizedBox(height: 4),
          Text(svc['description'] ?? '', style: TextStyle(color: Colors.grey, fontSize: 12)),
          SizedBox(height: 8),
          Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              if (svc['status'] == 'Running')
                TextButton.icon(
                  onPressed: () => _stopService(svc['name'] ?? ''),
                  icon: Icon(Icons.stop_circle_outlined, size: 16, color: Colors.amber),
                  label: Text('Stop', style: TextStyle(color: Colors.amber, fontSize: 12)),
                ),
              SizedBox(width: 8),
              TextButton.icon(
                onPressed: () => _disableService(svc['name'] ?? ''),
                icon: Icon(Icons.block, size: 16, color: Colors.red.withOpacity(0.8)),
                label: Text('Disable', style: TextStyle(color: Colors.red.withOpacity(0.8), fontSize: 12)),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildCleanerTab(Color surface, Color border) {
    return Column(
      children: [
        _buildWarningBanner('Safe categories shown. Moderate items and risky operations require confirmation.'),
        SizedBox(
          height: 50,
          child: Padding(
            padding: EdgeInsets.symmetric(horizontal: 12, vertical: 8),
            child: Row(
              children: [
                Expanded(
                  child: ElevatedButton.icon(
                    onPressed: () async {
                      setState(() => _loadingCleaner = true);
                      await _loadCleaner();
                    },
                    icon: Icon(Icons.search, size: 16),
                    label: Text('Scan'),
                    style: ElevatedButton.styleFrom(backgroundColor: Color(0xFF0EA5E9), foregroundColor: Colors.white),
                  ),
                ),
                SizedBox(width: 8),
                Expanded(
                  child: ElevatedButton.icon(
                    onPressed: () async {
                      final safeCategories = _cleanCategories
                          .where((cat) => cat['risk'] == 'safe')
                          .map((cat) => cat['id']?.toString() ?? '')
                          .where((id) => id.isNotEmpty)
                          .toList();
                      for (final id in safeCategories) {
                        await _runCleaner(id);
                      }
                    },
                    icon: Icon(Icons.cleaning_services, size: 16),
                    label: Text('Clean All Safe'),
                    style: ElevatedButton.styleFrom(backgroundColor: Color(0xFF10B981), foregroundColor: Colors.white),
                  ),
                ),
              ],
            ),
          ),
        ),
        Expanded(
          child: _loadingCleaner
              ? Center(child: CircularProgressIndicator(color: Color(0xFF0EA5E9)))
              : _cleanCategories.isEmpty
                  ? _buildEmpty('No scan results', 'Tap Scan to analyze your system', Icons.cleaning_services)
                  : ListView.builder(
                      padding: EdgeInsets.all(12),
                      itemCount: _cleanCategories.length,
                      itemBuilder: (_, i) => _buildCleanerCard(_cleanCategories[i], surface, border),
                    ),
        ),
      ],
    );
  }

  Widget _buildCleanerCard(Map<String, dynamic> cat, Color surface, Color border) {
    final riskColor = cat['risk'] == 'safe' ? Color(0xFF10B981) : Color(0xFFF59E0B);
    final mb = (cat['size_bytes'] ?? 0) / (1024 * 1024);
    return Container(
      margin: EdgeInsets.only(bottom: 8),
      padding: EdgeInsets.all(14),
      decoration: BoxDecoration(color: surface, borderRadius: BorderRadius.circular(10), border: Border.all(color: border)),
      child: Row(
        children: [
          Container(padding: EdgeInsets.all(8), decoration: BoxDecoration(color: riskColor.withOpacity(0.1), borderRadius: BorderRadius.circular(8)), child: Icon(Icons.folder_delete, color: riskColor, size: 20)),
          SizedBox(width: 12),
          Expanded(
            child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              Text(cat['name'] ?? '', style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 13)),
              SizedBox(height: 2),
              Text(cat['description'] ?? '', style: TextStyle(color: Colors.grey, fontSize: 11)),
            ]),
          ),
          Column(
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              Text('${mb.toStringAsFixed(1)} MB', style: TextStyle(color: Colors.grey, fontSize: 12, fontWeight: FontWeight.w600)),
              SizedBox(height: 4),
              SizedBox(
                height: 28,
                child: ElevatedButton(
                  onPressed: () => _runCleaner(cat['id'] ?? ''),
                  style: ElevatedButton.styleFrom(backgroundColor: riskColor, foregroundColor: Colors.white, padding: EdgeInsets.symmetric(horizontal: 12), textStyle: TextStyle(fontSize: 11)),
                  child: Text('Clean'),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildWarningBanner(String text) {
    return Container(
      width: double.infinity,
      margin: EdgeInsets.fromLTRB(12, 8, 12, 0),
      padding: EdgeInsets.all(10),
      decoration: BoxDecoration(color: Color(0xFFF59E0B).withOpacity(0.1), borderRadius: BorderRadius.circular(8), border: Border.all(color: Color(0xFFF59E0B).withOpacity(0.2))),
      child: Row(
        children: [
          Icon(Icons.info_outline, color: Color(0xFFF59E0B), size: 16),
          SizedBox(width: 8),
          Expanded(child: Text(text, style: TextStyle(color: Color(0xFFF59E0B), fontSize: 11))),
        ],
      ),
    );
  }

  Widget _buildEmpty(String title, String subtitle, IconData icon) {
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, size: 48, color: Colors.grey.shade700),
          SizedBox(height: 12),
          Text(title, style: TextStyle(color: Colors.grey.shade400, fontSize: 16)),
          SizedBox(height: 4),
          Text(subtitle, style: TextStyle(color: Colors.grey.shade600, fontSize: 12)),
        ],
      ),
    );
  }
}
