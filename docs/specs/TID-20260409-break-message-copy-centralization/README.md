# Task-ID: TID-20260409-break-message-copy-centralization

## Meta
- Title: 统一管理消息页提示语
- Date: 2026-04-09
- Level: trivial
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
- 桌面端 locale 真源改为 `apps/desktop/src/locales/{messages,config,overrides}`，不再从 `app/locales` 或 `app/preferences.html` 回灌。
- break 消息页提示语单独收口到 `apps/desktop/src/locales/break-message-copy.ts`，避免继续散落在组件和通用 locale override 中。

## Governance Notes
- Requirement Brief: 用户要求消息页提示语能单独管理且不要继续混用 legacy 设计；本任务将桌面端 locale 真源切到 `apps/desktop/src/locales/**`，并把 break 消息页文案独立成单文件入口。
- Interaction Impact: none
- Interaction Freeze: N/A（本任务不改变页面结构或交互语义，只调整文案来源与维护入口）
- Execution Safety Block: service_impact=仅限桌面端 locale 构建链路与 break 消息页文案读取；touches_running_service=no；backup_required=no；backup_plan=以 Git diff、`python3 scripts/sync_desktop_locales.py`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build` 为回退边界；rollback_plan=回退 `apps/desktop/src/locales/**`、`App.tsx`、`break-prompt.ts` 与 `scripts/sync_desktop_locales.py`；destructive_operations=none；operator_approval_required=no；rationale=不涉及线上服务、提权或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前 session 未获用户显式 delegation，且上层运行策略不允许因“想更稳妥”自行调用 `spawn_agent`，因此以受限单 agent 完成本次局部重构。
- Retention Decision: keep

## Notes
- 后续若要改 break 页面主提示语、默认提示语或按钮文案，直接编辑 `apps/desktop/src/locales/break-message-copy.ts`。
