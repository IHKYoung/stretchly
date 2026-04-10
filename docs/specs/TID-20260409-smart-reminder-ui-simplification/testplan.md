# Task-ID: TID-20260409-smart-reminder-ui-simplification

## Test Strategy
- Unit:
  - 无新增逻辑单测；复用现有 settings / locale 基础测试
- Integration:
  - locale sync、npm test、typecheck、build
- E2E (if applicable):
  - 无

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `npm --prefix apps/desktop run typecheck` + 手动代码审查 `App.tsx`
- AC2 -> `python3 scripts/sync_desktop_locales.py` + `rg -n 'waitForPause|waitForPauseHint'`
- AC3 -> 本轮不改 `state.rs`，由 diff 自证

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
- Artifacts path: docs/specs/TID-20260409-smart-reminder-ui-simplification/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: 减少一个控件后焦点链更简单
- perf budget: 无变化
- error handling / observability: 无变化
- security / privacy: 无新增权限或外部调用

## Boundary / Invalid Input Cases
- 已删除的 locale key 不应残留在 `messages/*.json` 或 registry 中。

## Concurrency / Race Cases (if applicable)
- 无；本轮不改 host 状态机。

## Mocks & Test Data
- 无

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `npm test`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
- Outputs to keep (10~20 lines snippet):
  - `sync_desktop_locales.py` -> `[OK] Built desktop locale registry: 50 languages`
  - `npm test` -> `Test Files  4 passed (4)` / `Tests  58 passed (58)`
  - `typecheck/build/docs validator` 全部 PASS
