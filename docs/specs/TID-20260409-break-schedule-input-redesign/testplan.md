# Task-ID: TID-20260409-break-schedule-input-redesign

## Test Strategy
- Unit: `test/desktopSettingsControls.js` 覆盖 preset 分布与草稿提交 helper
- Integration: 前端 typecheck 与 build
- E2E (if applicable): N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `test/desktopSettingsControls.js`
- AC2 -> `test/desktopSettingsControls.js`
- AC3 -> `npm test` + `npm --prefix apps/desktop run typecheck` + `npm --prefix apps/desktop run build`

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
- Artifacts path: docs/specs/TID-20260409-break-schedule-input-redesign/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional):

## Quality Gates (Non-functional)
- a11y: 自定义输入支持键盘提交与撤销
- perf budget: N/A
- error handling / observability: 非数字输入不应污染状态
- security / privacy: 不涉及

## Boundary / Invalid Input Cases
- 空字符串只应作为编辑态存在，不应被提交成 `NaN`。
- 超出范围的值提交后仍应 clamp 到 min/max。

## Concurrency / Race Cases (if applicable)
- N/A

## Mocks & Test Data
- 直接使用 helper 输出与 preset 常量

## Commands to Run
- `npm test`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`

## Expected Results
- PASS criteria:
  - preset 与草稿提交测试通过
  - 前端 typecheck/build 通过
- Outputs to keep (10~20 lines snippet):
  - `test/desktopSettingsControls.js` PASS
