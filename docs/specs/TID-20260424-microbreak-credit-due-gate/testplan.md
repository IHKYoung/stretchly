# Task-ID: TID-20260424-microbreak-credit-due-gate

## Test Strategy
- Unit:
  - 在 `state.rs` 增加 idle return 恢复结算的 due gate 回归测试。
- Integration:
  - N/A
- E2E (if applicable):
  - N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 新增单测：未到点从 `45s+` idle 返回，不触发 recovery credit。
- AC2 -> 新增单测：当前 pending break 未到点时，`next_break_kind / next_break_due_ms / cycle_index` 保持不变。
- AC3 -> 复用现有单测：`microbreak_is_credited_after_user_returns`、`long_break_is_deferred_after_user_returns`。

## Interaction Contract Coverage
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: N/A
- Fallback / secondary flow -> tests/evidence: N/A
- Visible states / transitions -> tests/evidence: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节三项与 Evidence Capture 的 `Required` 不得继续保留 `N/A/no/TBD`

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- `DONE` 任务对应的 spec 仍保留 `TBD/INIT` 占位。
- `orchestrator` 未显式使用 `sandbox_mode = "danger-full-access"` 与 `approval_policy = "never"`，却仍宣称当前仓库运行在 aggressive 基线。
- 代码变更前未记录 `Source Basis`，导致实现依据不可追溯。
- 子 agent 未显式 `approval_policy = "never"`。
- `moderate/complex` 任务通过缩小 `Required Roles` 伪装为 trivial fallback。
- 已使用 `single-agent-fallback`，但 logs/plans 没有单独记录 `Execution Mode` / `Fallback Scope` / `Fallback Reason Code`。
- 需要受角色边界约束的文件系统写命令没有经过 `run_role_guard.py`，只在结案时补跑范围校验。
- `git add -- <explicit paths...>` 仍被包进 wrapper / helper script，导致运行时看不到裸命令前缀。
- `interaction_impact != none`，但 plan/testplan/ui spec 没有定义 primary flow / fallback flow / visible states / evidence coverage。

## Evidence Capture (UI / E2E)
- Required: no   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260424-microbreak-credit-due-gate/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional):

## Quality Gates (Non-functional)
- a11y: N/A
- perf budget: 本轮不增加额外 tick 成本，判定仅新增一个 due 检查。
- error handling / observability: `last_action` 不应在未到点返回时误出现 recovery credit 文案。
- security / privacy: N/A

## Boundary / Invalid Input Cases
- pending break 存在但 `due_at > now`，返回不应触发 credit。
- pending break 已到点且 `idle_gap >= 45s`，返回仍应触发 credit / defer。
- `natural_break_threshold` 达标时，full reset 仍保留。

## Concurrency / Race Cases (if applicable)
- 同一 tick 中“idle return -> delivery blocker -> natural break”按既有优先级运行，本轮只收紧 recovery credit 的准入条件。

## Mocks & Test Data
- 使用 `PauzaState::default()` 与显式 `schedule_specific_break` / `schedule_next_slot` 构造 pending break。

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - Rust tests 通过，新增回归测试能证明未到点不会提前抵扣微休息。
  - Workflow docs 校验通过。
- Outputs to keep (10~20 lines snippet):
  - `cargo test` 中新增回归测试 PASS 片段
  - workflow docs validator PASS 片段
