#[cfg(test)]
mod integration_tests {
    use std::sync::Arc;

    #[test]
    fn test_software_catalog_not_empty() {
        let catalog = zb_shared::software::get_software_catalog();
        assert!(!catalog.is_empty(), "Software catalog should not be empty");
        for pkg in &catalog {
            assert!(!pkg.name.is_empty(), "Package name should not be empty");
        }
    }

    #[test]
    fn test_bloatware_catalog_not_empty() {
        let catalog = zb_shared::software::get_bloatware_catalog();
        assert!(!catalog.is_empty(), "Bloatware catalog should not be empty");
        for pkg in &catalog {
            assert!(!pkg.name.is_empty(), "Package name should not be empty");
        }
    }

    #[test]
    fn test_protected_apps_not_empty() {
        let apps = zb_shared::software::get_protected_apps();
        assert!(!apps.is_empty(), "Protected apps should not be empty");
    }

    #[test]
    fn test_make_all_tweaks() {
        let rp = zb_infrastructure::registry::WinRegistryProvider::new();
        let tweaks: Vec<Arc<dyn zb_domain::tweaks::Tweak>> = vec![
            Arc::new(
                zb_domain::tweaks::definitions::DisableGameDvrTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableTransparencyTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableAnimationsTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::ShowFileExtensionsTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableStickyKeysTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableStartupDelayTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableBackgroundAppsTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableTelemetryTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableMenuDelayTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableCursorShadowTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableFontSmoothingTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableTaskbarAnimationsTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableAeroShakeTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableAeroSnapTweak::with_provider(rp.clone()),
            ),
            Arc::new(zb_domain::tweaks::definitions::DisablePeekTweak::with_provider(rp.clone())),
            Arc::new(
                zb_domain::tweaks::definitions::DisableSmoothScrollTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableComboAnimationTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableTaskbarBadgesTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableAllVisualEffectsTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableDropShadowsTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableThumbnailsTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableMinMaxAnimTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableLockScreenAdsTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableStartSuggestionsTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableExplorerAdsTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableAdvertisingIdTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableMeetNowTweak::with_provider(rp.clone()),
            ),
            Arc::new(zb_domain::tweaks::definitions::DisableHibernationTweak::new()),
            Arc::new(zb_domain::tweaks::definitions::SetHighPerformanceTweak::new()),
            Arc::new(
                zb_domain::tweaks::definitions::DisableNaglesAlgorithmTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableNetworkThrottlingTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(zb_domain::tweaks::definitions::SetTcpAutotuningNormalTweak::new()),
            Arc::new(
                zb_domain::tweaks::definitions::DisableWifiSenseTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::EnableHwGpuSchedulingTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableFullscreenOptimizationsTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableMemoryCompressionTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableCortanaRegistryTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableLocationServicesTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableActivityHistoryTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableTailoredExperiencesTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableFeedbackFrequencyTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableAutoDriverUpdatesTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableWUAutoRebootTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableDeliveryOptimizationTweak::with_provider(
                    rp.clone(),
                ),
            ),
        ];
        assert_eq!(tweaks.len(), 44, "Should have 44 tweaks");
        for t in &tweaks {
            let m = t.metadata();
            assert!(!m.id.is_empty(), "Tweak ID should not be empty");
            assert!(!m.name.is_empty(), "Tweak name should not be empty");
        }
    }

    #[test]
    fn test_registry_provider_new() {
        let provider = zb_infrastructure::registry::WinRegistryProvider::new();
        assert!(Arc::strong_count(&provider) > 0);
    }

    #[test]
    fn test_service_controller_new() {
        let sc = zb_infrastructure::services::ServiceController::new();
        let services = sc.query_services();
        assert_eq!(services.len(), 19, "Should query 19 services");
        for svc in &services {
            assert!(!svc.name.is_empty());
        }
    }

    #[test]
    fn test_system_cleaner_scan() {
        let cleaner = zb_infrastructure::windows_api::system_cleaner::SystemCleaner::new();
        let categories = cleaner.scan_categories();
        assert_eq!(categories.len(), 9, "Should have 9 cleaner categories");
        for cat in &categories {
            assert!(!cat.id.is_empty());
        }
    }

    #[test]
    fn test_winget_installer_new() {
        let w = zb_infrastructure::windows_api::winget::WingetInstaller::new();
        let _available = w.is_available();
    }

    #[tokio::test]
    async fn test_sqlite_persistence() {
        let conn = zb_infrastructure::persistence::sqlite_repo::init_database();
        assert!(conn.is_ok(), "Database initialization should work");
    }

    #[tokio::test]
    async fn test_sqlite_repo_crud() {
        let repo = zb_infrastructure::persistence::sqlite_repo::SqliteRepo::new_in_memory();
        assert!(repo.is_ok(), "In-memory repo should be creatable");
        let repo = repo.unwrap();

        let snapshot = zb_domain::snapshots::SystemSnapshot::new(String::from("test snapshot"));
        let snap_id = snapshot.id.to_string();
        let result = repo.save_snapshot(snapshot).await;
        assert!(result.is_ok(), "Should save snapshot");

        let snapshots = repo.list_snapshots().await;
        assert!(snapshots.is_ok());
        let snapshots = snapshots.unwrap();
        assert_eq!(snapshots.len(), 1);

        let data = zb_shared::types::SnapshotData::Other("test".into());
        let result = repo.save_applied("test_tweak", data.clone()).await;
        assert!(result.is_ok());

        let result = repo.get_last_snapshot_data("test_tweak").await;
        assert!(result.is_ok());

        let result = repo.restore_snapshot(&snap_id).await;
        assert!(result.is_ok(), "restore_snapshot failed: {:?}", result);

        let result = repo.restore_snapshot("nonexistent").await;
        assert!(result.is_err(), "Should error on nonexistent snapshot");
    }

    #[test]
    fn test_registry_value_types() {
        use zb_shared::types::RegValue;
        let dword = RegValue::Dword(42);
        let qword = RegValue::Qword(12345);
        let sz = RegValue::Sz("hello".into());
        let expand = RegValue::ExpandSz("%PATH%".into());
        let binary = RegValue::Binary(vec![1, 2, 3]);
        let absent = RegValue::Absent;
        let multi = RegValue::MultiSz(vec!["a".into(), "b".into()]);

        assert!(matches!(dword, RegValue::Dword(_)));
        assert!(matches!(qword, RegValue::Qword(_)));
        assert!(matches!(sz, RegValue::Sz(_)));
        assert!(matches!(expand, RegValue::ExpandSz(_)));
        assert!(matches!(binary, RegValue::Binary(_)));
        assert!(matches!(absent, RegValue::Absent));
        assert!(matches!(multi, RegValue::MultiSz(_)));
    }

    #[test]
    fn test_reg_path_creation() {
        use zb_shared::types::{RegPath, RegRoot};
        let path = RegPath::hkcu(r"Software\Test");
        assert_eq!(path.root, RegRoot::Hkcu);
        assert_eq!(path.path, r"Software\Test");

        let path = RegPath::hklm(r"SOFTWARE\Test");
        assert_eq!(path.root, RegRoot::Hklm);
        assert_eq!(path.path, r"SOFTWARE\Test");
    }

    #[test]
    fn test_reg_root_display() {
        use zb_shared::types::RegRoot;
        assert_eq!(RegRoot::Hkcu.to_string(), "HKEY_CURRENT_USER");
        assert_eq!(RegRoot::Hklm.to_string(), "HKEY_LOCAL_MACHINE");
        assert_eq!(RegRoot::Hkcr.to_string(), "HKEY_CLASSES_ROOT");
        assert_eq!(RegRoot::Hku.to_string(), "HKEY_USERS");
        assert_eq!(RegRoot::Hkcc.to_string(), "HKEY_CURRENT_CONFIG");
    }

    #[test]
    fn test_tweak_metadata() {
        let rp = zb_infrastructure::registry::WinRegistryProvider::new();
        let tweaks: Vec<Arc<dyn zb_domain::tweaks::Tweak>> = vec![
            Arc::new(
                zb_domain::tweaks::definitions::DisableGameDvrTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableTransparencyTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableAnimationsTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::ShowFileExtensionsTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableStickyKeysTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableStartupDelayTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableBackgroundAppsTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableTelemetryTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableMenuDelayTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableCursorShadowTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableFontSmoothingTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableTaskbarAnimationsTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableAeroShakeTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableAeroSnapTweak::with_provider(rp.clone()),
            ),
            Arc::new(zb_domain::tweaks::definitions::DisablePeekTweak::with_provider(rp.clone())),
            Arc::new(
                zb_domain::tweaks::definitions::DisableSmoothScrollTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableComboAnimationTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableTaskbarBadgesTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableAllVisualEffectsTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableDropShadowsTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableThumbnailsTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableMinMaxAnimTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableLockScreenAdsTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableStartSuggestionsTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableExplorerAdsTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableAdvertisingIdTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableMeetNowTweak::with_provider(rp.clone()),
            ),
            Arc::new(zb_domain::tweaks::definitions::DisableHibernationTweak::new()),
            Arc::new(zb_domain::tweaks::definitions::SetHighPerformanceTweak::new()),
            Arc::new(
                zb_domain::tweaks::definitions::DisableNaglesAlgorithmTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableNetworkThrottlingTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(zb_domain::tweaks::definitions::SetTcpAutotuningNormalTweak::new()),
            Arc::new(
                zb_domain::tweaks::definitions::DisableWifiSenseTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::EnableHwGpuSchedulingTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableFullscreenOptimizationsTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableMemoryCompressionTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableCortanaRegistryTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableLocationServicesTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableActivityHistoryTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableTailoredExperiencesTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableFeedbackFrequencyTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableAutoDriverUpdatesTweak::with_provider(
                    rp.clone(),
                ),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableWUAutoRebootTweak::with_provider(rp.clone()),
            ),
            Arc::new(
                zb_domain::tweaks::definitions::DisableDeliveryOptimizationTweak::with_provider(
                    rp.clone(),
                ),
            ),
        ];
        for t in &tweaks {
            let m = t.metadata();
            assert!(!m.id.is_empty(), "Tweak {}: id empty", m.name);
            assert!(!m.name.is_empty(), "Tweak {}: name empty", m.id);
            assert!(
                !m.description.is_empty(),
                "Tweak {}: description empty",
                m.id
            );
            assert!(!m.category.to_string().is_empty());
            assert!(!m.risk.to_string().is_empty());
        }
    }

    #[tokio::test]
    async fn test_metrics_collector() {
        let mc = zb_infrastructure::windows_api::metrics_collector::MetricsCollector::new();
        // Wait for the sampler thread to initialize and collect its first sample
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        let metrics = mc.current().await;
        assert!(metrics.cpu_percent >= 0.0);
        assert!(metrics.ram_percent >= 0.0);
        assert!(metrics.ram_total_mb > 0);
        assert!(metrics.disk_active_percent >= 0.0);
    }
}
