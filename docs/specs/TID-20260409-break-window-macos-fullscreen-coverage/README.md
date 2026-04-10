# Task-ID: TID-20260409-break-window-macos-fullscreen-coverage

## Meta
- Title: 修复 macOS 全屏工作区下 break 窗口覆盖当前工作屏幕
- Date: 2026-04-09
- Level: trivial  <!-- trivial | moderate | complex -->
- Lane: fast    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-09.md
- Log (daily): ../../logs/2026-04-09.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 仅依赖 `set_visible_on_all_workspaces(true)` 不能可靠覆盖 macOS 全屏工作区。
- break 窗口在 macOS 上需要额外带上 `CanJoinAllSpaces | MoveToActiveSpace | FullScreenAuxiliary`。
- fullscreen 与 windowed break 都直接使用最高 native level，并在显示后主动 `orderFrontRegardless`。

## Governance Notes
- Requirement Brief: 用户反馈 break 提醒窗口在 macOS 全屏工作时没有覆盖当前工作屏幕；本任务仅修 Tauri host 的 macOS 原生窗口展示策略，不改前端页面布局、调度状态机或多屏选择模型。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限 macOS break 窗口展示层与相关文档；touches_running_service=no；backup_required=no；backup_plan=依赖 cargo test、desktop build 与 docs validator；rollback_plan=回退 `shell.rs` 中的 macOS native helper、相关测试与文档；destructive_operations=none；operator_approval_required=no；rationale=不涉及线上服务、权限、数据迁移或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无
- Retention Decision: keep

## Notes
- 设计依据已同步写入 `docs/Architecture.md`；当前实现只在 macOS 下生效，其他平台保持原行为。
