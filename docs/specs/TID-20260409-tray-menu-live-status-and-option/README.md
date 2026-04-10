# Task-ID: TID-20260409-tray-menu-live-status-and-option

## Meta
- Title: 修复托盘菜单实时状态并开放倒计时开关
- Date: 2026-04-09
- Level: moderate
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Links
- Plan (daily): ../../plans/2026-04-09.md
- Log (daily): ../../logs/2026-04-09.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 右键 tray 菜单中的 `status` / `status-detail` 改为原地 `set_text()` 更新，不再依赖菜单重建。
- 顶部 tray 倒计时增加用户设置开关，直接复用既有 `showTimeToBreakInTray` 文案和字段命名。
- 缺失的英文 `showTimeToBreakInTray` 文案补进 locale 真源并重生成 registry。

## Governance Notes
- Requirement Brief: 用户指出右键打开 tray 菜单后，里面的时间仍然卡住；同时希望顶部倒计时可作为一个设置项开关。本任务只修复 tray 菜单里的实时状态更新并开放倒计时开关，不改调度状态机和 tray 动作语义。
- Interaction Impact: none
- Interaction Freeze: N/A
- Execution Safety Block: service_impact=仅限本地 tray 菜单信息项与设置 schema/设置页开关；touches_running_service=no；backup_required=no；backup_plan=以 locale sync、TypeScript、Rust tests、desktop build 与 docs validator 为边界；rollback_plan=回退 `shell.rs`、`state.rs`、`App.tsx`、locale 变更与本任务 docs；destructive_operations=none；operator_approval_required=no；rationale=宿主展示与本地设置修复，不涉及外部系统、权限或数据迁移。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；问题边界清晰，集中在 tray 菜单刷新方式与设置模型曝光。
- Retention Decision: keep

## Notes
- 顶部 title 受 `showTimeToBreakInTray` 控制，右键菜单状态文案始终实时更新。
