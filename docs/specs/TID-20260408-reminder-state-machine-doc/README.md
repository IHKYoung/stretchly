# Task-ID: TID-20260408-reminder-state-machine-doc

## Meta
- Title: 整理提醒状态机与调度逻辑文档
- Date: 2026-04-08
- Level: trivial  <!-- trivial | moderate | complex -->
- Lane: fast    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-08.md
- Log (daily): ../../logs/2026-04-08.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 本轮先做减法，只保留 `智能提醒 / 强制提醒 / 自然休息` 三个用户概念。
- `强制提醒` 直接吸收原先 `严格模式` 的语义，定义为严格的定时打断，不再作为并列设置项存在。
- `breakPromptStyle` 明确剔出调度决策，只能保留为 UI 呈现层或后续直接移除。
- 第一版目标状态机不再包含 `soft nudge`，先把“到点后等空档再提醒”的主路径做清晰。
- 正式设计文档落盘到 `docs/ReminderScheduling.md`，作为后续实现的直接依据。

## Governance Notes
- Requirement Brief: 用户要求先不要继续改代码，而是先把下一轮要收敛的提醒状态机和整体调度逻辑写成一份正式文档，明确智能提醒、强制提醒与自然休息的职责边界，并把原先的严格模式折叠进强制提醒。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅新增/更新文档，不改运行时代码；touches_running_service=no；backup_required=no；backup_plan=以 Git diff 和 workflow docs validator 为边界；rollback_plan=回退本任务新增/更新的 docs；destructive_operations=none；operator_approval_required=no；rationale=纯文档任务，无服务、副作用、提权或数据风险。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: trivial-scribe-only（仅 fallback 时填写）
- Escalation Summary: N/A
- Retention Decision: keep

## Notes
- Primary artifact: `docs/ReminderScheduling.md`
- 同步更新：`docs/CodeMap.md`、当日 `docs/plans/2026-04-08.md`、`docs/logs/2026-04-08.md`
