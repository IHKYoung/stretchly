# Task-ID: TID-20260403-break-window-scale-fix

## Meta
- Title: 修复休息窗口过小与双层卡片
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
- break window 的核心问题不是单一尺寸值，而是“宿主窗口 + React 内层小卡片”的双层缩放。
- break 页面改为直接吃满宿主窗口，不再在窗口里再套一张固定宽度卡片。
- window 模式下三档 break profile 的默认尺寸整体拉大，优先消除“像个小弹窗”的观感。

## Governance Notes
- Requirement Brief: 用户贴图指出当前微休息窗口“太扯淡”，本轮直接修正 break 页面结构和 Tauri host 的默认窗口尺寸，去掉窗口里的小卡片套娃感。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: break 页保留现有倒计时、标题、说明和操作按钮，但视觉结构改为填满宿主窗口；主设置页不变。
- Execution Safety Block: service_impact=仅改 break window React 页面和 Tauri host 默认尺寸，不改调度逻辑；touches_running_service=no；backup_required=no；backup_plan=依赖 VCS、cargo/typecheck/build 和 break preview snapshot；rollback_plan=回退 `App.tsx`、`shell.rs` 与本任务 docs；destructive_operations=替换 break window 现有双层容器结构与尺寸策略；operator_approval_required=no；rationale=用户明确要求修正 break 窗口观感，本轮无数据/外部副作用风险。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无
- Retention Decision: keep

## Notes
- 关键证据：`./evidence/break-window-preview-snapshot.md`
- Playwright screenshot 仍卡在 fonts wait，本轮以 preview DOM snapshot 和编译链作为 UI 旁证
