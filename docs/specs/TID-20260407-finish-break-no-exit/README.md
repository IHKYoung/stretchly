# Task-ID: TID-20260407-finish-break-no-exit

## Meta
- Title: 修复完成休息后应用退出
- Date: 2026-04-07
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-07.md
- Log (daily): ../../logs/2026-04-07.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- `close_break_window()` 保留 fullscreen 退出 + destroy 的真实 teardown 语义，不回退到仅 `hide()`
- break CTA 相关命令改为先返回 `DesktopSnapshot`，再异步销毁当前 break webview，避免 `invoke` 回包过程中同步 teardown 当前窗口
- 修复范围限制在 Tauri host 的 break command / shell 生命周期，不扩散到 React 结构、settings schema 或 Electron legacy

## Governance Notes
- Requirement Brief: 用户报告点击“完成休息”后应用会直接退出；本轮只修复 Tauri break 完成链路，确保 break 窗口关闭后应用仍留在后台运行。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: frozen；只修复 break CTA 关闭后的宿主生命周期，不改按钮语义和页面布局。
- Execution Safety Block: service_impact=仅限 break window close timing；touches_running_service=no；backup_required=no；backup_plan=以 Git 差异和编译链为边界；rollback_plan=回退 `commands.rs`、`shell.rs` 与本任务 docs；destructive_operations=替换 break CTA 的关闭时序；operator_approval_required=no；rationale=不涉及数据、权限和外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: BLOCKED: 当前 session 未获得用户显式 delegation 授权，遵循上层策略不 spawn_agent；由单 agent 完成同范围的规划、实现、验证和文档收口。
- Retention Decision: keep

## Notes
- 仍需用户在本机真实点击一次 “完成休息” 做最终现实确认；自动化验证已覆盖 Rust 编译、单测、前端 typecheck/build 与 workflow docs validator。
