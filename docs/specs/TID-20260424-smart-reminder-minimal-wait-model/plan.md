# Task-ID: TID-20260424-smart-reminder-minimal-wait-model

## Summary
- Title: 收缩智能提醒为固定阈值与最长等待
- Date: 2026-04-24
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 将当前复杂的 smart reminder v2 收回到最小状态机，只保留“到点后先等空档，超过最长等待后开始”的核心逻辑，避免 recovery hold / recovery credit / 递减阈值带来的复杂性和体验漂移。
- In-scope:
  - 调整 `apps/desktop/src-tauri/src/state.rs` 的 smart/forced 到点后投递逻辑。
  - 移除 recovery hold / recovery credit / defer / staged threshold。
  - 更新 locale 与 `docs/ReminderScheduling.md`，使文案和设计描述与实现一致。
  - 补充或调整 Rust 单测覆盖固定阈值与 max wait 语义。
- Out-of-scope:
  - 不新增设置项。
  - 不引入新的 activity score 或更复杂输入信号。
  - 不变更 break planner 的整体节奏来源。
- Assumptions:
  - 用户当前主要痛点是“递减阈值 + 恢复补偿把状态机搞乱了”，而不是需要更多智能性。
  - 继续保留 blocker freeze 与 active break 优先级，能避免回退到更差的旧语义。
- Risks:
  - 固定阈值如果不合适，可能导致等待过长或仍显得打断感太强。
  - 如果删减 recovery 逻辑不彻底，文案/测试/文档会继续漂移。
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 把 smart reminder 收敛为可解释、可测试、可维护的最小模型。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/locales/messages/zh-CN.json`
  - `apps/desktop/src/locales/messages/en.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `docs/ReminderScheduling.md`
  - `docs/CHANGELOG.md`
  - `docs/specs/TID-20260424-smart-reminder-minimal-wait-model/*`
  - `docs/plans/2026-04-24.md`
  - `docs/logs/2026-04-24.md`
- Out-of-scope:
  - `apps/desktop/src/App.tsx`
  - 打包 / 提交 / 发布流程
  - 新的提醒模式和高级设置

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/engine.rs`
  - `apps/desktop/src/locales/messages/zh-CN.json`
  - `apps/desktop/src/locales/messages/en.json`
- Related docs/specs/logs reviewed:
  - `docs/ReminderScheduling.md`
  - `docs/CHANGELOG.md`
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/logs/2026-04-23.md`
  - `docs/specs/TID-20260424-microbreak-credit-due-gate/plan.md`
- Why these are sufficient:
  - 当前问题集中在 desktop host reminder 状态机；`App.tsx` 只用于确认用户侧模式语义没有新增入口，locale/doc 用于对齐提示与设计说明。

## Acceptance Criteria (AC)
- AC1: `Smart` 只保留 `idle_threshold` 与 `max_wait` 两条开始条件，不再进入 recovery hold，也不再执行 recovery credit / defer。
- AC2: `Smart` 固定阈值为微休息 `8s / 90s`、长休息 `12s / 180s`，且不再递减。
- AC3: `Forced` 到点立即开始；已修复的 `BreakActive` 优先级与 blocker freeze 语义保持不变。
- AC4: locale 与 `docs/ReminderScheduling.md` 不再描述“逐步放宽阈值”“recovery hold”“schedule reset on DND/app exclusion clear”等旧语义。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: N/A
- Primary flow: N/A
- Fallback / secondary flow: N/A
- Interaction authority / ownership boundary: N/A
- Visible entrypoints / handoff cues: N/A
- In-scope interactions: N/A
- Out-of-scope interactions: N/A
- Interaction acceptance criteria: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节与 Requirement Brief 中的交互字段不得继续保留 `N/A/TBD`

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅影响本地 desktop runtime 的 reminder 调度逻辑、运行时文案与相关文档。
- touches_running_service: no
- backup_required: no
- backup_plan: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`python3 scripts/validate_workflow_docs.py --mode manual`
- rollback_plan: 回退 `state.rs`、locale 与本任务相关 docs 改动。
- destructive_operations: none
- operator_approval_required: no
- rationale: 无线上服务、数据迁移、权限提升或外部副作用；属于本地调度逻辑收敛。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收缩 smart 等待逻辑
  - DoD: `state.rs` 中 recovery hold / recovery credit / staged threshold 路径移除，改成 fixed threshold + max wait。
- [x] Task-2: 调整回归测试
  - DoD: Rust 单测覆盖固定阈值与最大等待行为，旧恢复补偿测试移除或重写。
- [x] Task-3: 同步文案与设计文档
  - DoD: locale 与 `ReminderScheduling.md` 不再描述旧模型。
- [x] Task-4: 运行验证并结案
  - DoD: `cargo test` 与 workflow docs validator 通过，spec/logs/plans 无占位残留。

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260424-smart-reminder-minimal-wait-model/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs:
  - 复用现有 `last_action`，验证 `waitingForOpportunity`、`breakStarted`、`naturalBreakFinished`、`dndEnded`、`appExclusionCleared` 的语义是否仍然成立。
- Error codes:
  - N/A
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - N/A

## Risks & Rollback
- Risks:
  - fixed threshold 若过高会让 smart 等待时间显著变长。
  - 最大等待若计算错位，可能造成 break 永远不开始或开始过早。
- Rollback plan:
  - 回退本任务在 `state.rs`、locale 与 docs 的改动，恢复到本轮收敛前版本。

## Outcome
- 已将 smart reminder 收缩为固定阈值 + 最大等待模型。
- 已删除 recovery hold / recovery credit / defer 与 staged threshold 相关调度路径。
- 已同步更新 locale、`apps/desktop/src/locales/registry.generated.json`、`docs/ReminderScheduling.md` 与 `docs/CHANGELOG.md`。
- 已通过 `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`（27 passed）与 `python3 scripts/validate_workflow_docs.py --mode manual`。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 移除 recovery hold / recovery credit / staged threshold 相关分支与辅助函数。
  2. 将 smart mode 改为 `idle_threshold + max_wait` 固定模型。
  3. 调整测试、locale 和 `ReminderScheduling.md`。
  4. 运行 Rust 测试与 workflow docs 校验。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户在当前会话中明确确认“可以，改吧”）
