# Task-ID: TID-20260409-smart-reminder-wait-guard

## Test Strategy
- Unit:
  - Rust 单测覆盖 smart wait 的等待与超时开始
  - JS 单测覆盖设置页新 locale key
- Integration:
  - locale sync + typecheck + build
- E2E (if applicable):
  - 无

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `state::tests::pending_break_starts_when_idle_opportunity_appears` + `test/desktopSettingsControls.js` + `npm --prefix apps/desktop run typecheck`
- AC2 -> `state::tests::due_break_waits_for_idle_opportunity` + `state::tests::smart_mode_starts_break_after_wait_cap_even_without_idle_gap`
- AC3 -> `test/desktopSettingsControls.js` + `python3 scripts/sync_desktop_locales.py`

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
- Artifacts path: docs/specs/TID-20260409-smart-reminder-wait-guard/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: locale 文案不显示 raw key
- perf budget: 不引入新的高频轮询；继续沿用 1s tick
- error handling / observability: 继续复用现有 runtime state 观察点
- security / privacy: 不新增权限或外部调用

## Boundary / Invalid Input Cases
- `idleOpportunitySeconds` 在前台控件中限制为 `3..60`
- host 侧继续把 `idle_opportunity_seconds` sanitize 到 `3..120`

## Concurrency / Race Cases (if applicable)
- `snapshot()` 使用真实 `now_ms()` 时，等待态显示仍需和 tick 状态保持一致；本轮已通过 `snapshot_status_reflects_waiting_for_opportunity` 守住该边界。

## Mocks & Test Data
- 无新增 mock；继续使用既有 state/unit test setup

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `npm test`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
- Outputs to keep (10~20 lines snippet):
  - `npm test` -> `Test Files  4 passed (4)` / `Tests  59 passed (59)`
  - `cargo test` -> `test result: ok. 17 passed; 0 failed`
  - `sync_desktop_locales.py` -> `[OK] Built desktop locale registry: 50 languages`
