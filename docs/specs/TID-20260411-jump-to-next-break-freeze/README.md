# Task-ID: TID-20260411-jump-to-next-break-freeze

## Meta
- Title: 修复顶部菜单跳到下一次休息导致卡死
- Date: 2026-04-11
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-11.md
- Log (daily): ../../logs/2026-04-11.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 初步定位显示，tray 菜单点击后仍会立即走 `refresh_tray()`，而该路径内部会同步 `tray.set_menu()`。
- `set_menu()` 与 `MenuItem::set_text()` 都经 Tauri 的主线程阻塞桥接执行；这与 `TID-20260403-macos-tray-menu-crash` 已记录的 macOS 原生菜单生命周期风险重合。
- 当前最优先修复方向是不再在 tray 菜单点击路径上立刻重建整棵菜单，而是把刷新延后到菜单关闭后再按需执行。
- 最终实现为：tray `on_menu_event` 在点击后先完成状态变更，再等待 `150ms`，随后只调用 `refresh_tray_if_needed()`；`open` / `hide` / `quit` 这类无需刷新菜单的动作直接跳过刷新。

## Governance Notes
- Requirement Brief: 用户反馈当前在顶部菜单选择“跳到下一次休息”时应用会直接卡死；本任务聚焦 tray 菜单动作后的宿主刷新时机，不改 break 调度语义、settings schema 或前台文案。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限 macOS menubar/tray 动作后的刷新时机与本地宿主行为；touches_running_service=no；backup_required=no；backup_plan=依赖 Rust/前端构建、workflow docs validator 与既有 tray 生命周期源码审查；rollback_plan=回退 `apps/desktop/src-tauri/src/shell.rs` 与本任务 docs；destructive_operations=none；operator_approval_required=no；rationale=纯本地桌面端宿主修复，不涉及数据、权限、外部副作用或线上服务。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前 session 未获用户显式 delegation 授权，遵循上层工具策略不调用 `spawn_agent`；因此在限定于 tray 宿主刷新链路、测试与文档闭环的范围内执行单 agent fallback。
- Retention Decision: keep

## Notes
- 重点源码集中在 `apps/desktop/src-tauri/src/shell.rs` 的 tray `on_menu_event`、`refresh_tray()` 与 `refresh_tray_if_needed()`。
- 关键参考为 `docs/specs/TID-20260403-macos-tray-menu-crash/*` 以及本地 Tauri 2 / muda menu 实现。
- 自动化已覆盖 Rust tests、前端 build 和 workflow docs validator；最终 menubar 点击现实检查仍建议用户在本机实际点一次确认。
