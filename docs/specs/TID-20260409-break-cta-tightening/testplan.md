# Task-ID: TID-20260409-break-cta-tightening

## Test Strategy
- Unit: Rust 单测覆盖“前 10 秒内允许延后”和“非 manualAwaiting 不能 finish”
- Integration: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- E2E (if applicable): `npm test` 与 `npm --prefix apps/desktop run build`

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `state::tests::finish_current_break_requires_manual_awaiting`
- AC2 -> `state::tests::postpone_is_only_available_in_first_ten_seconds`
- AC3 -> `App.tsx` CTA 条件渲染 + `npm --prefix apps/desktop run build`
- AC4 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`; `npm test`; `npm --prefix apps/desktop run build`; `python3 scripts/validate_workflow_docs.py --mode manual`

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
- Artifacts path: docs/specs/TID-20260409-break-cta-tightening/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional):

## Quality Gates (Non-functional)
- a11y:
- perf budget:
- error handling / observability:
- security / privacy:

## Boundary / Invalid Input Cases
- break 正在进行但尚未到 manualAwaiting 时，直接命令 finish 应失败且保留当前 break

## Concurrency / Race Cases (if applicable)
- `tick()` 自动结束 break 与用户 CTA 接近同时发生时，不应出现提前 finish 跳过计时

## Mocks & Test Data
- 使用 `state.rs` 现有 `CurrentBreak` / `PauzaState` 测试基座

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm test`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
- Outputs to keep (10~20 lines snippet):
- PASS criteria:
  - Rust 新增两条 CTA 时机测试通过
  - 前端测试与 desktop build 通过
  - docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - `test state::tests::postpone_is_only_available_in_first_ten_seconds ... ok`
  - `test state::tests::finish_current_break_requires_manual_awaiting ... ok`
