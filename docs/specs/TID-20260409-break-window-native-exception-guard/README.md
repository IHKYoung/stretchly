# Task-ID: TID-20260409-break-window-native-exception-guard

## Meta
- Title: 修复 macOS 休息窗口原生异常闪退
- Date: 2026-04-09
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
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
- 两份 macOS crash report 都显示主线程在 AppKit event loop 中命中 Rust foreign exception abort，栈上同时出现 `_rust_foreign_exception` 与 `objc2::runtime::message_receiver::*`，说明问题集中在 break window 的原生 `NSWindow` selector 调用。
- macOS break window 的 native patch 现在改为 best-effort：`ns_window()` 与后续 `msg_send!` 统一包进 Objective-C exception guard，异常时只记录 `eprintln!` 并跳过 patch，不再直接 abort。
- `configure_break_window_native_behavior()` 与 `present_break_window()` 统一延后到 `window.show()` 之后执行，减少 break 宿主窗口尚未 ready 时抢先访问原生句柄的风险。

## Governance Notes
- Requirement Brief: 用户反馈进入 break 页面前应用直接报 `Rust cannot catch foreign exceptions, aborting`；本任务仅修复 `shell.rs` 的 macOS break window 原生 patch 防崩溃策略与展示时机，不改 React break 页面、调度状态机或 tray 结构。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: frozen；仅修正进入 break 前的宿主原生 patch 时机与异常降级，不改 break 页面布局、CTA 或调度入口。
- Execution Safety Block: service_impact=仅限 macOS break 宿主窗口 patch 与相关文档；touches_running_service=no；backup_required=no；backup_plan=依赖 crash report 对照、`cargo check`、`cargo test`、desktop build 与 workflow validator；rollback_plan=回退 `apps/desktop/src-tauri/src/shell.rs` 和本任务 docs；destructive_operations=none；operator_approval_required=no；rationale=不涉及运行中服务、数据迁移、权限提升或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: BLOCKED: 当前 session 未获用户显式 delegation 授权，遵循上层工具策略不调用 `spawn_agent`；由单 agent 在限定范围内完成崩溃定位、实现、验证、evidence 与文档收口。
- Retention Decision: keep

## Notes
- 证据路径见 `docs/specs/TID-20260409-break-window-native-exception-guard/evidence/README.md`；当前自动化已覆盖 crash report 分析、Rust 编译/单测和 desktop build，但仍保留“需用户本机真实进入一次 break” 的最终现实确认缺口。
