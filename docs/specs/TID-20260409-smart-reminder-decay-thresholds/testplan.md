# Task-ID: TID-20260409-smart-reminder-decay-thresholds

## Test Strategy
- Unit:
  - Rust 单测覆盖微休息放宽、长休息首段更宽阈值、最终 deadline 触发
- Integration:
  - locale sync、npm test、typecheck、build
- E2E (if applicable):
  - 无

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `state::tests::microbreak_wait_threshold_relaxes_after_first_stage` + `state::tests::long_break_uses_wider_idle_window_before_relaxing`
- AC2 -> `state::tests::smart_mode_starts_break_after_final_stage_even_without_idle_gap`
- AC3 -> 文档 diff + `python3 scripts/validate_workflow_docs.py --mode manual`

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
- Artifacts path: docs/specs/TID-20260409-smart-reminder-decay-thresholds/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: 无新增 UI 控件
- perf budget: 继续沿用 1s tick，不新增高频轮询
- error handling / observability: waiting 阶段应仍能通过 status 文案解释当前状态
- security / privacy: 不新增权限或外部调用

## Boundary / Invalid Input Cases
- 微休息在第二阶段应能接受 `3s` idle，但在第一阶段不能提前放宽。
- 休息在第一阶段即使 `idle_ms = 6s` 也不应提前开始。

## Concurrency / Race Cases (if applicable)
- `snapshot()` 使用真实 `now_ms()` 时，waiting 状态仍需在 due break 未真正开始前保持一致。

## Mocks & Test Data
- 无新增 mock；继续使用现有 `PauzaState::default()` 测试 setup

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm test`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
- Outputs to keep (10~20 lines snippet):
  - `cargo test` -> `test result: ok. 19 passed; 0 failed`
  - `npm test` -> `Test Files  4 passed (4)` / `Tests  58 passed (58)`
  - `sync_desktop_locales.py` -> `[OK] Built desktop locale registry: 50 languages`
