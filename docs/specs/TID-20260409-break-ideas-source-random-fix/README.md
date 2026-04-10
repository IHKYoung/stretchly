# Task-ID: TID-20260409-break-ideas-source-random-fix

## Meta
- Title: 接回 break ideas 真源并删除过渡 prompts
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
- break 页面交互语不再读取 `ui.breakCopy.prompts.*` 这组过渡数组。
- 当前唯一真源改回 `messages/*.json` 顶层的 `miniBreakIdeas` / `longBreakIdeas`。
- 选词逻辑改成“每次 break 稳定、跨 break 混洗”的索引方式，避免固定 10m/30m 节奏一直命中同一条。

## Governance Notes
- Requirement Brief: 用户指出当前 break 交互语并不是从 `apps/desktop/src/locales/messages` 的 `miniBreakIdeas/longBreakIdeas` 随机得到，而是固定落在同一条。本任务将 break prompt 真源重新接回 locale ideas，并删除 `ui.breakCopy.prompts` 过渡字段与相关代码回退。
- Interaction Impact: none
- Interaction Freeze: N/A
- Execution Safety Block: service_impact=仅限 break prompt 文案读取路径、locale 真源、前端 helper、回归测试与相关 docs；touches_running_service=no；backup_required=no；backup_plan=依赖 locale sync、typecheck、test、build 与 docs validator；rollback_plan=回退 `i18n.ts`、`break-ideas.ts`、`App.tsx`、locale JSON、测试与 docs；destructive_operations=删除过渡 prompt 字段；operator_approval_required=no；rationale=仅调整本地前端文案来源与稳定随机算法，无外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；问题边界清晰，集中在 locale 真源和 prompt 选取算法。
- Retention Decision: keep

## Notes
- `ui.breakCopy.defaultPrompt.*` 继续保留，用于 `breakIdeasEnabled = false` 时的明确兜底文案；`ui.breakCopy.prompts.*` 已移除。
