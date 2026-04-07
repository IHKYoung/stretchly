# Task-ID: TID-20260403-macos-tray-menu-crash

## Meta
- Title: 修复 macOS 托盘右键菜单闪退
- Date: 2026-04-03
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: REVIEW  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-03.md
- Log (daily): ../../logs/2026-04-03.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Evidence: ./evidence/README.md

## Decision Log
- 首轮定位确认了 `muda` 在 macOS 顶层 menu 上的 `Submenu` 约束，因此 `create_tray_menu` 已按平台分支为 macOS 返回 `Submenu<R>`。
- 二次定位发现更直接的用户感知根因在 `engine.rs`：后台 tick 每秒执行 `refresh_tray()`，会在原生菜单刚展开时立刻替换整棵 tray menu，表现为“右键菜单闪一下就消失”。
- 修复改为保留显式动作后的强制刷新，同时为后台 tick 增加 `refresh_tray_if_needed()` 与内容级 refresh key，只在 tray 菜单内容发生有效变化时才重建原生菜单。
- 保持 tray 左键仍只负责显示主窗口，右键菜单修复只收敛在 macOS root context menu 类型，不改动作处理线程和已有调度语义。

## Governance Notes
- Requirement Brief: 修复 `apps/desktop` 的 macOS tray 右键菜单闪退，只调整 Tauri Rust host 的菜单构造方式，不改菜单信息架构、动作 ID 或 Electron legacy 路径。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 已冻结为 `macOS 菜单栏 tray icon -> 右键弹出原生菜单` 与 `左键显示主窗口` 两条链路；本轮不改菜单文案分组、前台 React UI 或休息调度状态机。
- Execution Safety Block: service_impact=仅 macOS 本地 tray 右键菜单行为；touches_running_service=no；backup_required=no；backup_plan=依赖 VCS + 现有 dev runtime；rollback_plan=回退 `apps/desktop/src-tauri/src/shell.rs` 的 macOS tray menu 分支与 docs；destructive_operations=none；operator_approval_required=no；rationale=本轮仅修复本地桌面壳崩溃点，不涉及数据、权限、外部副作用或运行中服务。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无代码层升级；自动化右键复测被 macOS assistive access 权限阻塞，当前状态进入 `REVIEW`，等待用户在已运行 app 上执行最终手工验证。
- Retention Decision: keep

## Notes
- 修复代码位于 `apps/desktop/src-tauri/src/shell.rs`。
- 后台刷新策略调整位于 `apps/desktop/src-tauri/src/engine.rs`。
- 证据说明位于 `docs/specs/TID-20260403-macos-tray-menu-crash/evidence/README.md`。
