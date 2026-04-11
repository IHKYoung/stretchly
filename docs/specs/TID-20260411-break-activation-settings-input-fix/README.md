# Task-ID: TID-20260411-break-activation-settings-input-fix

## Meta
- Title: 修复全屏场景 break 不浮出与设置手输卡死
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
- Evidence Report: ./evidence/README.md

## Decision Log
- 维持现有 `PauzaSettings` / `update_settings` / 调度状态机契约不变，只修复前台数字输入交互和 macOS break 浮出链路。
- 对 macOS break 问题，优先补“显式激活 app/当前 Space”的宿主层修复，而不是重写 `tick()` 或 break planner。
- 对设置输入问题，优先补“本地草稿 -> 显式提交”的组件内修复，而不是增加新的 save debounce 或改 settings schema。

## Governance Notes
- Requirement Brief: 修复 break prompt 在当前全屏工作区不浮出的宿主行为，以及设置页数字输入手动编辑时的回弹/卡住问题；范围只含 `App.tsx` 与 `shell.rs` 相关链路，不扩到调度状态机、locale 或 schema。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 已冻结为“全屏 Space 可见的 break 浮出 + 数字草稿输入 blur/Enter 提交”两条交互边界，不扩到 break 页面视觉或 settings 信息架构。
- Execution Safety Block: service_impact=仅限 desktop 前台数字输入与 macOS break 宿主激活链路；touches_running_service=no；backup_required=no；backup_plan=依赖前端/Rust 构建测试与 workflow docs validator；rollback_plan=回退 `App.tsx`、`shell.rs` 与对应 docs；destructive_operations=none；operator_approval_required=no；rationale=纯本地桌面端交互修复。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前 session 未获用户显式 delegation 授权，遵循上层工具策略不调用 `spawn_agent`；因此由单 agent 在限定范围内完成 spec、实现、验证与 docs 闭环。
- Retention Decision: keep

## Notes
- 本任务与 `TID-20260409-break-window-macos-fullscreen-coverage`、`TID-20260409-break-schedule-input-redesign`、`TID-20260410-settings-language-switch-freeze`、`TID-20260411-jump-to-next-break-freeze` 连续相关。
