# Task-ID: TID-20260402-codex-init-understanding

## Test Strategy
- Unit: 复用现有 Vitest 测试集确认核心领域逻辑未因初始化过程受影响。
- Integration: 运行 workflow validator，确认 specs、plans、logs 与 hooks 配置符合仓库门禁。
- E2E (if applicable): 本任务不改 UI 行为，不做浏览器或 Electron 端到端取证。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `python3 ~/.codex/scripts/ensure_workflow_ready.py --target . --hooks required` + `python3 scripts/validate_workflow_docs.py`
- AC2 -> 人工核对 `docs/RepositoryGuidelines.md`、`docs/CodeMap.md` 与仓库现状
- AC3 -> 人工核对 `docs/Architecture.md`、`docs/UI.md` 与已阅读源码的一致性
- AC4 -> `npm run lint`、`npm test`、`python3 scripts/validate_agent_configs.py`

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
- Artifacts path: docs/specs/TID-20260402-codex-init-understanding/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 命令行输出记录在 logs 中即可

## Quality Gates (Non-functional)
- a11y: N/A，本任务不改界面
- perf budget: N/A，本任务不改运行时逻辑
- error handling / observability: workflow 命令需全部成功，且日志中保留关键结果
- security / privacy: 不新增依赖、不扩大 Electron 能力暴露、不接触用户数据

## Boundary / Invalid Input Cases
- 缺失 workflow 模板时，`ensure_workflow_ready.py` 应能自动补齐。
- 脏工作树存在时，本任务仍需保持只写 workflow/docs 相关文件。

## Concurrency / Race Cases (if applicable)
- N/A；本任务为静态阅读与文档初始化，无并发行为变更。

## Mocks & Test Data
- 直接复用仓库现有测试数据与 fixture，无新增 mock。

## Commands to Run
- `python3 ~/.codex/scripts/ensure_workflow_ready.py --target . --hooks required`
- `npm run lint`
- `npm test`
- `python3 scripts/validate_agent_configs.py`
- `python3 scripts/validate_workflow_docs.py`

## Expected Results
- PASS criteria: lint 通过；Vitest 全绿；workflow docs 校验通过；agent config 校验无错误或明确 skip。
- Outputs to keep (10~20 lines snippet): `Test Files 17 passed (17)`、`Tests 315 passed (315)`、`workflow docs validation PASSED`、`Agent config validation skipped: no multi-agent config detected`。
