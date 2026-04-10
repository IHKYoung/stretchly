# Task-ID: TID-20260409-break-window-macos-fullscreen-coverage

## Test Strategy
- Unit: Rust 单测验证 macOS break window 采用的原生 collection behavior bits 与 level 选择
- Integration: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- E2E (if applicable): `npm --prefix apps/desktop run build` 作为 host + 前台整体构建回归

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `shell::tests::macos_break_window_native_overlay_policy_uses_expected_levels`
- AC2 -> `shell::tests::macos_break_window_native_overlay_policy_uses_expected_levels` + host 编译通过
- AC3 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`; `npm --prefix apps/desktop run build`

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
- Artifacts path: docs/specs/TID-20260409-break-window-macos-fullscreen-coverage/evidence/
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
- `ns_window()` 句柄不可用时 helper 应直接 no-op，不影响现有 Tauri show path

## Concurrency / Race Cases (if applicable)
- 所有原生窗口 patch 必须继续发生在主线程 `run_on_main_thread()` 上

## Mocks & Test Data
- 使用 `shell.rs` 现有 snapshot 测试基座与纯函数级常量断言

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
- Outputs to keep (10~20 lines snippet):
- PASS criteria:
  - Rust 单测全部通过，新增 macOS overlay policy 测试通过
  - desktop build 通过
  - workflow docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - `test shell::tests::macos_break_window_native_overlay_policy_uses_expected_levels ... ok`
  - `test result: ok. 20 passed; 0 failed`
