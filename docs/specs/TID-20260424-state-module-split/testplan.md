# Task-ID: TID-20260424-state-module-split

## Test Strategy
- Unit:
  - 复用现有 `state::tests::*` Rust 单测，确保 settings / persistence / tests 迁移后行为不变
- Integration:
  - 通过 `cargo test` 验证 `commands.rs` / `platform.rs` / `shell.rs` 对 `crate::state` 的编译与链接
  - 通过 `npm run typecheck`、`npm test` 验证前端未受 host 模块整理影响
- E2E (if applicable):
  - N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`（覆盖 `state::tests::*` 与 host 编译）
- AC2 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`（覆盖 legacy strict / idle field / update settings 等持久化相关路径）
- AC3 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`python3 scripts/validate_workflow_docs.py --mode manual`

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
- Artifacts path: docs/specs/TID-20260424-state-module-split/evidence/
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
- legacy `settings.json` 仍带 `microbreakStrictMode` / `idleOpportunitySeconds` 时，迁移逻辑不能丢
- `crate::state` 的 public types 继续能被 `commands.rs` / `platform.rs` / `shell.rs` 使用
- tests 子模块迁移后仍能访问父模块私有 runtime helper 与常量

## Concurrency / Race Cases (if applicable)
- N/A（本轮不改 runtime 并发模型，只做文件边界整理）

## Mocks & Test Data
- 复用现有 Rust tests 中的临时 `settings.json` fixture 与 runtime scheduling fixtures

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run typecheck`
- `npm test`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - Rust tests 全部通过，且 `state::tests::*` 无回归
  - TS typecheck 与 Vitest 维持通过
  - workflow docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - `test result: ok. 28 passed; 0 failed`
  - `Test Files  5 passed (5)`
