# Task-ID: TID-20260404-fullscreen-break-close-fix

## Meta
- Title: 修复全屏休息跳过后黑屏
- Date: 2026-04-04
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-04.md
- Log (daily): ../../logs/2026-04-04.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Evidence Report: ./evidence/README.md

## Decision Log
- 根因判断为 Tauri `close_break_window()` 在 fullscreen break 上只做 `hide()`，没有完整 teardown。
- 修复策略对齐旧 Electron：fullscreen break 在关闭链路里先退出 fullscreen，再销毁窗口，不复用黑壳窗口实例。

## Governance Notes
- Requirement Brief: 根据用户反馈，修复 fullscreen 休息时点击“跳过”后留下黑屏的问题；本轮只动 break window 宿主关闭链路，不改调度逻辑或前端按钮语义。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 用户在 fullscreen break 点击 `跳过` / `完成` / `稍后` 时，应正确退出 fullscreen 并销毁 break 窗口，不再残留黑屏空间。
- Execution Safety Block: service_impact=仅改 Tauri break window 关闭路径；touches_running_service=no；backup_required=no；backup_plan=依赖 VCS、cargo/typecheck/build 以及代码路径对照旧 Electron；rollback_plan=回退 `shell.rs` 与本任务 docs；destructive_operations=替换当前 break window 关闭策略；operator_approval_required=no；rationale=用户明确指出高优先级宿主 bug，本轮无数据或外部副作用风险。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无
- Retention Decision: keep

## Notes
- 本轮没有拿到“修复后 fullscreen skip 不再黑屏”的新鲜真实截图；最终现实确认仍需你本机手点一次。
- 编译链与代码路径已收口：`close_break_window()` 现在先退出 fullscreen，再 `destroy()`。
