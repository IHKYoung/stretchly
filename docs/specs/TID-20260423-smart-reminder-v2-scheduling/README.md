# Task-ID: TID-20260423-smart-reminder-v2-scheduling

## Meta
- Title: 重构智能提醒状态机与恢复结算
- Date: 2026-04-23
- Level: complex  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-23.md
- Log (daily): ../../logs/2026-04-23.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 决定把 pause/focus/DND/app exclusion 从“reset schedule”改成“freeze delivery + 平移 due / waiting timer”，避免 reminder 节奏在 blocker 前后漂移。
- 决定把恢复结算放在 host 内部运行时层完成，不新增设置项；本轮只用本地规则 credit，不引入更复杂的 activity score。
- 决定保持 `smart / forced / naturalBreaks` 用户概念不变，但补齐 `recovery hold` 与“最多再等多久”的状态解释。

## Governance Notes
- Requirement Brief: 重构智能提醒 v2，让 passive blocker 冻结投递而不是重置节奏；让 smart mode 在用户短暂离开时先做恢复结算而不是直接弹 break；补齐可解释状态文案。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 已在本任务 spec 中冻结为“只改运行时状态机与状态文案，不改设置页结构和 break window 视觉”。
- Execution Safety Block: 仅限本地 desktop runtime 状态机、locale 文案与文档；无运行中服务、无破坏性操作、可通过回退 `state.rs/commands.rs/locale/docs` 恢复。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked（当前 session 未获用户显式子 agent 授权，按上层工具策略不可 delegation）
- Escalation Summary: 若 blocker freeze 与 recovery credit 的叠加场景出现未收敛行为，优先通过单测与 docs 回退到上一版状态机。
- Retention Decision: keep（blocker freeze、recovery credit 与更可解释的运行时状态属于长期保留的 reminder v2 资产）

## Notes
- 相关证据与验证结果已回写到 daily logs / plans / evidence。
