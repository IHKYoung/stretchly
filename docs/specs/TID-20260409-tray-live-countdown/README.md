# Task-ID: TID-20260409-tray-live-countdown

## Meta
- Title: 修复托盘实时倒计时
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
- tray 菜单继续按 `TrayRefreshKey` 的分钟级/状态级变化刷新，不改成每秒重建。
- 顶部 tray 文本单独通过 `tray.set_title()` 每秒同步真实倒计时。
- tooltip 同步状态标题和详情，避免顶部纯数字缺少语义。
- 新增 Rust 纯函数测试，覆盖倒计时格式、当前 break 优先级和阻塞态隐藏倒计时。

## Governance Notes
- Requirement Brief: 用户反馈顶部 tray 的倒计时像卡住一样不友好；本任务只修复 tray 标题的实时倒计时与相关 tooltip/测试，不改调度状态机和菜单动作。
- Interaction Impact: none
- Interaction Freeze: N/A
- Execution Safety Block: service_impact=仅限本地 tray 标题/tooltip 刷新链路；touches_running_service=no；backup_required=no；backup_plan=以 Rust tests、desktop build 和 docs validator 为边界；rollback_plan=回退 `shell.rs` 与本任务 docs；destructive_operations=none；operator_approval_required=no；rationale=宿主展示层修复，不涉及外部系统、权限或数据迁移。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；问题边界清晰，集中在 tray 展示层。
- Retention Decision: keep

## Notes
- 关键实现位于 `apps/desktop/src-tauri/src/shell.rs`，未改动设置 schema 或前端页面。
