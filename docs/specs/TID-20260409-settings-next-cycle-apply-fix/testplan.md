# Task-ID: TID-20260409-settings-next-cycle-apply-fix

## Test Strategy
- Unit: Rust 单测覆盖当前 break 与已排队 next break
- Integration: `npm test` 与前端 build，确保联动未破坏
- E2E (if applicable): N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `state::tests::update_settings_keeps_current_break_running`
- AC2 -> `state::tests::update_settings_keeps_already_scheduled_next_break`
- AC3 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` + `npm test` + `npm --prefix apps/desktop run build`

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
- Artifacts path: docs/specs/TID-20260409-settings-next-cycle-apply-fix/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional):

## Quality Gates (Non-functional)
- a11y: N/A
- perf budget: 不新增 tick 频率或额外后台线程
- error handling / observability: 继续保留 `settingsUpdated` action 记录
- security / privacy: 不涉及

## Boundary / Invalid Input Cases
- 当不存在 current/next break 且未阻塞时，settings 更新后仍应重新生成 future schedule。

## Concurrency / Race Cases (if applicable)
- `update_settings()` 与 tick 都通过 `runtime` mutex 串行化，避免 schedule 状态竞争。

## Mocks & Test Data
- 测试内临时 `settings.json` 路径 + 默认 settings

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm test`
- `npm --prefix apps/desktop run build`

## Expected Results
- PASS criteria:
  - 两条新增 Rust 单测通过
  - 前端 tests/build 不回归
- Outputs to keep (10~20 lines snippet):
  - `update_settings_keeps_current_break_running ... ok`
  - `update_settings_keeps_already_scheduled_next_break ... ok`
