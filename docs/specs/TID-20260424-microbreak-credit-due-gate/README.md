# Task-ID: TID-20260424-microbreak-credit-due-gate

## Meta
- Title: 修复恢复结算误提前抵扣微休息
- Date: 2026-04-24
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-24.md
- Log (daily): ../../logs/2026-04-24.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 2026-04-24: 复现后确认根因不在用户配置；本机 `settings.json` 中 `microbreakEnabled=true`、`longBreakEvery=3`。
- 2026-04-24: `tick()` 的 idle return recovery credit 缺少 `pending_due_kind(now)` gate，导致未到点的微休息也会被提前自动抵扣。

## Governance Notes
- Requirement Brief: 将 recovery credit 收紧为“只结算 overdue break”，避免微休息被未到点提前吃掉。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: 仅改本地 desktop host 状态机与测试；无服务影响、无破坏性操作、可直接回退 `state.rs` 与相关 docs。
- Approval Owner: orchestrator
- Warmup Required Roles: orchestrator,architect,coder,tester,scribe
- Warmup Ready Roles: single-agent-fallback
- Warmup Agent IDs: single-agent-fallback（当前 session 未获用户显式 delegation）
- Warmup Verification: BLOCKED: 当前 session 未获用户显式授权调用 `spawn_agent`，按上层工具策略只能走 `single-agent-fallback`。
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前 session 未获用户显式授权调用 `spawn_agent`，按上层工具策略保持单 agent 闭环处理。
- Retention Decision: keep

## Notes
- 设计意图与实现漂移点：`docs/ReminderScheduling.md` 预期 recovery credit 只处理“break 已到点且用户返回”的场景；当前实现对任何 pending break 都会触发结算。
- 已修复：`apply_recovery_credit()` 现在只会处理 `pending_due_kind(now)`，不再提前吃掉未到点的微休息。
- 已补回归测试：
  - `microbreak_is_not_credited_before_due_when_user_returns`
  - `long_break_is_not_deferred_before_due_when_user_returns`
