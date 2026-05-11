# Graph Report - ZingerBoost  (2026-05-11)

## Corpus Check
- 69 files · ~22,852 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 592 nodes · 746 edges · 61 communities (28 shown, 33 thin omitted)
- Extraction: 88% EXTRACTED · 12% INFERRED · 0% AMBIGUOUS · INFERRED: 87 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `156411fc`
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
- [[_COMMUNITY_Community 29|Community 29]]
- [[_COMMUNITY_Community 30|Community 30]]
- [[_COMMUNITY_Community 31|Community 31]]
- [[_COMMUNITY_Community 32|Community 32]]
- [[_COMMUNITY_Community 33|Community 33]]
- [[_COMMUNITY_Community 34|Community 34]]
- [[_COMMUNITY_Community 35|Community 35]]
- [[_COMMUNITY_Community 36|Community 36]]
- [[_COMMUNITY_Community 37|Community 37]]
- [[_COMMUNITY_Community 38|Community 38]]
- [[_COMMUNITY_Community 39|Community 39]]
- [[_COMMUNITY_Community 40|Community 40]]
- [[_COMMUNITY_Community 41|Community 41]]
- [[_COMMUNITY_Community 42|Community 42]]
- [[_COMMUNITY_Community 43|Community 43]]
- [[_COMMUNITY_Community 44|Community 44]]
- [[_COMMUNITY_Community 45|Community 45]]
- [[_COMMUNITY_Community 46|Community 46]]
- [[_COMMUNITY_Community 47|Community 47]]

## God Nodes (most connected - your core abstractions)
1. `SystemCleaner` - 22 edges
2. `dir_size()` - 17 edges
3. `DebloatEngine` - 13 edges
4. `DisableCursorShadowTweak` - 10 edges
5. `DisableBackgroundAppsTweak` - 10 edges
6. `DisableComboAnimationTweak` - 10 edges
7. `DisableDropShadowsTweak` - 10 edges
8. `DisableTelemetryTweak` - 10 edges
9. `DisableMenuDelayTweak` - 10 edges
10. `DisableAnimationsTweak` - 10 edges

## Surprising Connections (you probably didn't know these)
- `run()` --calls--> `init_logging()`  [INFERRED]
  crates/zb_app/src/lib.rs → crates/zb_infrastructure/src/logging.rs
- `main()` --calls--> `run()`  [INFERRED]
  crates/zb_app/src/main.rs → crates/zb_app/src/lib.rs

## Communities (61 total, 33 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.06
Nodes (4): DisableCursorShadowTweak, DisableLockScreenAdsTweak, DisablePeekTweak, DisableTransparencyTweak

### Community 1 - "Community 1"
Cohesion: 0.09
Nodes (13): App, make_all_tweaks(), Message, run(), Tab, init_logging(), App, main() (+5 more)

### Community 2 - "Community 2"
Cohesion: 0.2
Nodes (6): CleanCategory, CleanResult, dir_size(), get_local_appdata(), remove_dir_contents(), SystemCleaner

### Community 3 - "Community 3"
Cohesion: 0.1
Nodes (3): DisableGameDvrTweak, DisableTelemetryTweak, RegPath

### Community 4 - "Community 4"
Cohesion: 0.08
Nodes (24): 1. Install Flutter, 2. Verify Flutter, 3. Install Visual Studio Build Tools, 4. Clone or pull the latest code, 5. Go to the Flutter folder, 6. Generate Windows platform files, 7. Install Flutter packages, 8. Build the app (+16 more)

### Community 5 - "Community 5"
Cohesion: 0.1
Nodes (19): 🔧 Adding a New Tweak, 📜 Code of Conduct, 📝 Code Style, code:bash (# Formatting), code:bash (cd zingerboost_flutter), code:bash (cargo check -p zb_shared -p zb_domain -p zb_application), code:bash (cargo check --workspace), code:block5 (ZingerBoost/) (+11 more)

### Community 6 - "Community 6"
Cohesion: 0.12
Nodes (12): AppErrorDto, AuditEntry, AuditLevel, RegRoot, RegValue, RiskLevel, SnapshotData, SystemMetrics (+4 more)

### Community 8 - "Community 8"
Cohesion: 0.14
Nodes (7): ApplyRequestDto, AuditLogDto, BatchApplyRequestDto, SystemMetricsDto, TweakExplanationDto, TweakListDto, TweakResultDto

### Community 9 - "Community 9"
Cohesion: 0.28
Nodes (4): get_safe_to_disable_list(), ServiceController, to_wide(), WindowsService

### Community 10 - "Community 10"
Cohesion: 0.2
Nodes (3): init_database(), migrations(), SqliteRepo

### Community 11 - "Community 11"
Cohesion: 0.17
Nodes (11): Adding a Tweak, Architecture, CI, code:bash (cargo check -p zb_shared -p zb_domain -p zb_application), code:block2 (crates/), code:bash (git tag v0.0.6 && git push origin v0.0.6), Critical Gotchas for Iced 0.13, Quick Check (Linux) (+3 more)

### Community 37 - "Community 37"
Cohesion: 0.22
Nodes (8): Author, Build, code:powershell (# Download from releases), code:powershell (git clone https://github.com/YousefMohiey/ZingerBoost.git), Download, Safe, Reversible Windows Optimization — Pure Rust, Tech Stack, ZingerBoost v0.0.5

### Community 38 - "Community 38"
Cohesion: 0.43
Nodes (4): MetricsCollector, read_cpu_counter(), read_disk_counter(), read_ram()

### Community 41 - "Community 41"
Cohesion: 0.33
Nodes (5): BenchmarkError, RegistryError, ServiceError, SnapshotError, TweakError

### Community 43 - "Community 43"
Cohesion: 0.5
Nodes (3): Benchmark, BenchmarkProgress, BenchmarkResult

## Knowledge Gaps
- **67 isolated node(s):** `SoftwarePackage`, `RegValue`, `TweakMetadata`, `TweakResult`, `TweakExplanation` (+62 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **33 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `RegPath` connect `Community 3` to `Community 0`, `Community 6`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **What connects `SoftwarePackage`, `RegValue`, `TweakMetadata` to the rest of the system?**
  _67 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Community 0` be split into smaller, more focused modules?**
  _Cohesion score 0.06 - nodes in this community are weakly interconnected._
- **Should `Community 1` be split into smaller, more focused modules?**
  _Cohesion score 0.09 - nodes in this community are weakly interconnected._
- **Should `Community 3` be split into smaller, more focused modules?**
  _Cohesion score 0.1 - nodes in this community are weakly interconnected._
- **Should `Community 4` be split into smaller, more focused modules?**
  _Cohesion score 0.08 - nodes in this community are weakly interconnected._
- **Should `Community 5` be split into smaller, more focused modules?**
  _Cohesion score 0.1 - nodes in this community are weakly interconnected._