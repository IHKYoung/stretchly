# Task-ID: TID-20260409-settings-runtime-action-clarity

## Meta
- Title: 整理设置页中的恢复提醒与重置节奏
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
- Evidence Report: ./evidence/README.md

## Decision Log
- `恢复提醒` 只保留为暂停态的上下文动作，不再作为常驻顶栏按钮。
- `重置节奏` 保持为独立运行时动作，并显式说明它不会自动解除暂停。
- 运行时动作仍不是“设置项”；本轮只是在设置窗口中提供更清楚的动作入口。

## Governance Notes
- Requirement Brief: 用户指出设置界面顶部把“恢复提醒”和“重置节奏”混在一起会产生语义冲突；本轮需要把前者收敛为暂停态动作，把后者拆成独立节奏控制，并维持 reminder state machine 与宿主命令语义不变。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 已冻结为“顶部只放上下文动作；节奏重置单独成组；不新增新的提醒模式或暂停语义”。
- Execution Safety Block: service_impact=仅限 Tauri 设置页的信息架构、浏览器 preview fallback 与文案；touches_running_service=no；backup_required=no；backup_plan=依赖 Git diff、typecheck/build、docs validator 与 UI 证据；rollback_plan=回退 `App.tsx`、locale overrides/registry 与 docs；destructive_operations=替换设置页当前对运行时动作的表达方式；operator_approval_required=no；rationale=不涉及线上服务、提权、数据迁移或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前 session 未获用户显式授权使用子 agent，且上层运行策略禁止主动 `spawn_agent`；因此本任务在明确边界内由单 agent 兜底完成。
- Retention Decision: keep

## Notes
- Primary source basis: `apps/desktop/src/App.tsx`、`apps/desktop/src/i18n.ts`、`apps/desktop/src-tauri/src/{commands.rs,state.rs}`、`docs/{ReminderScheduling,SettingsInventory,UI}.md`
- 已补齐浏览器 preview 证据：`settings-default.png`、`settings-paused.png`、`settings-focus.png`，用于验证“暂停态才显示恢复提醒”“focus 态改为结束专注”和“重置节奏独立成组”的界面结果。
