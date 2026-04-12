# Task-ID: TID-20260412-settings-lock-inversion-deadlock-doc-commit

## Meta
- Title: 沉淀设置页锁反转死锁根因并提交
- Date: 2026-04-12
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: merge-gate  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-12.md
- Log (daily): ../../logs/2026-04-12.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 将设置页彩球卡死的真正根因收口为 `LAST_TRAY_MENU_TEXT_UPDATER` 上的锁反转死锁，而不是继续停留在“autosave 并发 + host 全量刷新”这个不完整解释。
- `shell.rs` 中的 tray 文本热更新改为“锁内 clone updater，锁外执行 `MenuItem::set_text()`”，显式打破后台 tick 与主线程 tray rebuild 的互锁条件。
- 历史任务与旧日志不回写重写；新的根因纠偏、并发不变量与回滚边界统一沉淀到本 Task-ID、`docs/Architecture.md` 与 `docs/CHANGELOG.md`。

## Governance Notes
- Requirement Brief: 用户提供了已定位出的死锁链，要求把这次真正的根因、修复方式和工程教训做成详细文档，并以单独 commit 收口。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限本地 `apps/desktop/src-tauri/src/shell.rs` 并发修正、文档沉淀与 git commit；touches_running_service=no；backup_required=no；backup_plan=`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` + `python3 scripts/validate_workflow_docs.py --mode manual` + `git diff --check`；rollback_plan=若后续发现 tray 文本无法实时更新，可基于本次 commit 直接 `git revert`；destructive_operations=git commit（用户已明确要求）；operator_approval_required=no；rationale=仅本地代码与文档收口，无线上副作用、无提权、无额外成本。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无。用户已直接给出根因链路，当前任务只需补齐代码意图、文档追溯与 merge-gate commit。
- Retention Decision: keep

## Notes
- 真正的死锁序列是：后台引擎线程在 `sync_tray_menu_text()` 中持有 `LAST_TRAY_MENU_TEXT_UPDATER` 锁，并在锁内调用 `status_item.set_text()` / `detail_item.set_text()`；Tauri 会把这两个更新投递到主线程并阻塞等待。与此同时，设置保存触发的 tray rebuild 会在主线程 `create_tray_menu() -> register_tray_menu_text_updater()` 路径再次尝试获取同一把锁，主线程与后台线程互相等待，表现为 macOS 彩球。
- 之前的 autosave 串行化与 `update_settings` 差异刷新仍然是有价值的减压措施，但它们解决的是“高频触发更容易撞上宿主刷新”的放大器，不是这次 beachball 的最终并发根因。
