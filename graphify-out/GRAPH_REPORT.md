# Graph Report - /home/verhafter/ZingerBoost  (2026-05-07)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 327 nodes · 358 edges · 43 communities (24 shown, 19 thin omitted)
- Extraction: 92% EXTRACTED · 8% INFERRED · 0% AMBIGUOUS · INFERRED: 30 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b7fcb5e3`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 5|Community 5]]
- [[_COMMUNITY_Community 6|Community 6]]
- [[_COMMUNITY_Community 7|Community 7]]
- [[_COMMUNITY_Community 8|Community 8]]
- [[_COMMUNITY_Community 9|Community 9]]
- [[_COMMUNITY_Community 10|Community 10]]
- [[_COMMUNITY_Community 11|Community 11]]
- [[_COMMUNITY_Community 12|Community 12]]
- [[_COMMUNITY_Community 13|Community 13]]
- [[_COMMUNITY_Community 14|Community 14]]
- [[_COMMUNITY_Community 15|Community 15]]
- [[_COMMUNITY_Community 16|Community 16]]
- [[_COMMUNITY_Community 17|Community 17]]
- [[_COMMUNITY_Community 18|Community 18]]
- [[_COMMUNITY_Community 19|Community 19]]
- [[_COMMUNITY_Community 20|Community 20]]
- [[_COMMUNITY_Community 21|Community 21]]
- [[_COMMUNITY_Community 22|Community 22]]
- [[_COMMUNITY_Community 23|Community 23]]
- [[_COMMUNITY_Community 24|Community 24]]
- [[_COMMUNITY_Community 25|Community 25]]
- [[_COMMUNITY_Community 26|Community 26]]
- [[_COMMUNITY_Community 27|Community 27]]
- [[_COMMUNITY_Community 28|Community 28]]

## God Nodes (most connected - your core abstractions)
1. `DisableBackgroundAppsTweak` - 10 edges
2. `DisableTelemetryTweak` - 10 edges
3. `DisableAnimationsTweak` - 10 edges
4. `DisableGameDvrTweak` - 10 edges
5. `DisableStartupDelayTweak` - 10 edges
6. `DisableTransparencyTweak` - 10 edges
7. `DisableStickyKeysTweak` - 10 edges
8. `ShowFileExtensionsTweak` - 10 edges
9. `SqliteRepo` - 9 edges
10. `TweakEngine` - 9 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `init_logging()`  [INFERRED]
  src-tauri/src/main.rs → crates/zb_infrastructure/src/logging.rs
- `main()` --calls--> `init_database()`  [INFERRED]
  src-tauri/src/main.rs → crates/zb_infrastructure/src/persistence/sqlite_repo.rs
- `ToastContainer()` --calls--> `useToastStore`  [EXTRACTED]
  src/components/ui/ToastContainer.tsx → src/store/toast.ts
- `list_software()` --calls--> `get_software_catalog()`  [INFERRED]
  crates/zb_app/src/commands.rs → crates/zb_shared/src/software.rs
- `list_bloatware()` --calls--> `get_bloatware_catalog()`  [INFERRED]
  crates/zb_app/src/commands.rs → crates/zb_shared/src/software.rs

## Communities (43 total, 19 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.07
Nodes (27): api, AuditEntry, SoftwarePackage, SystemMetrics, SystemSnapshot, TweakExplanation, TweakMetadata, TweakResult (+19 more)

### Community 1 - "Community 1"
Cohesion: 0.07
Nodes (3): DisableAnimationsTweak, DisableBackgroundAppsTweak, ShowFileExtensionsTweak

### Community 2 - "Community 2"
Cohesion: 0.11
Nodes (20): [activeCategory, setActiveCategory], [activeTab, setActiveTab], addToast, BloatwareData, categories, categoryIcons, categoryLabels, { data: bloatData } (+12 more)

### Community 3 - "Community 3"
Cohesion: 0.09
Nodes (10): InstallRequest, InstallResult, list_bloatware(), list_software(), RemoveBloatwareRequest, get_bloatware_catalog(), get_protected_apps(), get_software_catalog() (+2 more)

### Community 4 - "Community 4"
Cohesion: 0.12
Nodes (12): AppErrorDto, AuditEntry, AuditLevel, RegRoot, RegValue, RiskLevel, SnapshotData, SystemMetrics (+4 more)

### Community 5 - "Community 5"
Cohesion: 0.14
Nodes (5): init_database(), migrations(), SqliteRepo, init_logging(), main()

### Community 6 - "Community 6"
Cohesion: 0.14
Nodes (7): ApplyRequestDto, AuditLogDto, BatchApplyRequestDto, SystemMetricsDto, TweakExplanationDto, TweakListDto, TweakResultDto

### Community 18 - "Community 18"
Cohesion: 0.33
Nodes (5): BenchmarkError, RegistryError, ServiceError, SnapshotError, TweakError

### Community 19 - "Community 19"
Cohesion: 0.4
Nodes (4): bottomItems, location, navItems, SidebarProps

### Community 23 - "Community 23"
Cohesion: 0.5
Nodes (3): Benchmark, BenchmarkProgress, BenchmarkResult

## Knowledge Gaps
- **75 isolated node(s):** `queryClient`, `[sidebarOpen, setSidebarOpen]`, `icons`, `colors`, `SidebarProps` (+70 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **19 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `RegPath` connect `Community 7` to `Community 1`, `Community 4`?**
  _High betweenness centrality (0.033) - this node is a cross-community bridge._
- **What connects `queryClient`, `[sidebarOpen, setSidebarOpen]`, `icons` to the rest of the system?**
  _75 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Community 0` be split into smaller, more focused modules?**
  _Cohesion score 0.07 - nodes in this community are weakly interconnected._
- **Should `Community 1` be split into smaller, more focused modules?**
  _Cohesion score 0.07 - nodes in this community are weakly interconnected._
- **Should `Community 2` be split into smaller, more focused modules?**
  _Cohesion score 0.11 - nodes in this community are weakly interconnected._
- **Should `Community 3` be split into smaller, more focused modules?**
  _Cohesion score 0.09 - nodes in this community are weakly interconnected._
- **Should `Community 4` be split into smaller, more focused modules?**
  _Cohesion score 0.12 - nodes in this community are weakly interconnected._