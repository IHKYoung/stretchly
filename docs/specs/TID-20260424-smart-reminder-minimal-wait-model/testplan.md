# Task-ID: TID-20260424-smart-reminder-minimal-wait-model

## Test Strategy
- Unit:
  - 调整 `state.rs` 单测，覆盖固定阈值、最大等待、forced 立即开始、natural break full reset 与 blocker freeze 保持不变。
- Integration:
  - N/A
- E2E (if applicable):
  - N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `smart_mode_uses_fixed_microbreak_idle_threshold`、`long_break_uses_fixed_idle_threshold`
- AC2 -> `pending_break_starts_when_idle_opportunity_appears`、`smart_mode_starts_break_after_max_wait_even_without_idle_gap`
- AC3 -> `forced_mode_starts_break_immediately`、`active_break_is_not_closed_by_passive_blockers`、`dnd_freezes_waiting_timer_without_reset`
- AC4 -> locale / docs 文案检查 + `python3 scripts/validate_workflow_docs.py --mode manual`

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
- Artifacts path: docs/specs/TID-20260424-smart-reminder-minimal-wait-model/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional):

## Quality Gates (Non-functional)
- a11y: N/A
- perf budget: 本轮不引入额外状态机层级；tick 只保留固定阈值和最大等待判断。
- error handling / observability: `last_action` 不应再出现 recovery hold / recovery credit 路径；DND 与 app exclusion clear 文案应体现“恢复”而不是“重置”。
- security / privacy: N/A

## Boundary / Invalid Input Cases
- break 已到点但 `idle_ms < threshold` 时，smart 应继续等待。
- break 已到点且 `idle_ms >= threshold` 时，应立即开始。
- break 已到点但没有空档，超过 `max_wait` 后也必须开始。
- `natural_break_threshold` 达标并返回输入后，节奏应从当前时间重排。

## Concurrency / Race Cases (if applicable)
- `BreakActive` 优先级必须高于 delivery blocker。
- blocker clear 只平移 pending schedule，不应让 waiting timer 丢失。

## Mocks & Test Data
- 使用 `PauzaState::default()` 与显式 `schedule_specific_break` / `schedule_next_slot` 构造待投递 break。

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - Rust tests 通过，且 fixed threshold / max wait 行为可由单测证明。
  - Workflow docs 校验通过。
- Outputs to keep (10~20 lines snippet):
  - `cargo test` PASS 片段
  - workflow docs validator PASS 片段
