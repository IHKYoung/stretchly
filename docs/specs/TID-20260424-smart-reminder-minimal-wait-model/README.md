# Task-ID: TID-20260424-smart-reminder-minimal-wait-model

## Meta
- Title: 收缩智能提醒为固定阈值与最长等待
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
- 2026-04-24: 用户确认当前 smart reminder v2 复杂度过高，应收回到最小状态机，只解决“到点后先等空档、不要用递减阈值硬凑机会”。
- 2026-04-24: 保留 `BreakActive` 优先级与 passive blocker freeze 语义；回滚 `RecoveryHold`、recovery credit / defer 与 staged threshold。
- 2026-04-24: 固定内置阈值定为微休息 `8s / 90s`、长休息 `12s / 180s`。

## Governance Notes
- Requirement Brief: 将 smart reminder 收缩为固定阈值 + 最长等待模型，避免当前恢复补偿和递减阈值把状态机复杂度推高。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: 仅调整本地 desktop host reminder 调度逻辑、文案与文档；无服务影响、无破坏性操作，可直接回退 `state.rs`、locale 与 docs。
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
- 本轮目标不是引入更复杂的“可打断性推理”，而是把 smart/forced 两种模式重新拉回可解释的最小差异。
- 用户侧设置不增加新开关；`idle_threshold` 与 `max_wait` 保持内置常量。
- 已验证：
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` PASS（27 passed）
  - `python3 scripts/validate_workflow_docs.py --mode manual` PASS
