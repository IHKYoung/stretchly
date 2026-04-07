# Task-ID: TID-20260403-break-surface-mode-restore

## Meta
- Title: 恢复休息窗口/全屏设置
- Date: 2026-04-03
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-03.md
- Log (daily): ../../logs/2026-04-03.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Evidence Report: ./evidence/README.md

## Decision Log
- 旧 Electron 的 `fullscreen` 设置是独立于 `breakPromptStyle` 的另一根轴，当前 Tauri 端需要恢复这根轴。
- Tauri 设置模型继续使用 `fullscreen: boolean`，与旧实现保持兼容，而不是新造另一套枚举字段。
- `window` 模式下，`immersive` long break 不再被强制全屏，而是恢复为大尺寸居中窗口；`fullscreen` 模式下才真正铺满显示器。

## Governance Notes
- Requirement Brief: 根据用户反馈，恢复休息窗口的“窗口 / 全屏”设置，让 Tauri 端重新具备 break surface mode 这根独立配置轴，并在前端设置页重新可见。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 设置页中“提醒与打断”分类重新包含 `显示方式：窗口 / 全屏`；该设置保存后直接影响 Tauri break window 的尺寸与全屏行为。
- Execution Safety Block: service_impact=仅改 Tauri 设置模型、break window profile 和设置页入口，不改调度算法；touches_running_service=no；backup_required=no；backup_plan=依赖 VCS、cargo/typecheck/build 与 DOM snapshot 旁证；rollback_plan=回退 `state.rs`、`shell.rs`、`App.tsx`、locale 与本任务 docs；destructive_operations=替换当前 break window surface mode 逻辑；operator_approval_required=no；rationale=用户明确要求恢复功能，本轮无数据/外部副作用风险。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无
- Retention Decision: keep

## Notes
- 证据落盘在 `./evidence/settings-break-surface-snapshot-1440x810.md` 与 `./evidence/browser-console.log`
- Playwright screenshot 在本页上持续卡在 fonts wait，因此本轮可视旁证以 DOM snapshot 为主，并以构建/编译链补强
