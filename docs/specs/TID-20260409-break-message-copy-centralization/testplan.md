# Task-ID: TID-20260409-break-message-copy-centralization

## Test Strategy
- Unit:
  - `npm --prefix apps/desktop run typecheck`
- Integration:
  - `python3 scripts/sync_desktop_locales.py`
  - `npm --prefix apps/desktop run build`
- E2E (if applicable):
  - 不涉及；本任务不改交互流程，只改文案来源与 locale 构建链路。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `App.tsx` / `break-prompt.ts` 代码审查 + `npm --prefix apps/desktop run typecheck`
- AC2 -> `python3 scripts/sync_desktop_locales.py` + `scripts/sync_desktop_locales.py` 代码审查
- AC3 -> `npm --prefix apps/desktop run typecheck` + `npm --prefix apps/desktop run build`
- AC4 -> docs diff 审查

## Interaction Contract Coverage
- Interaction impact: none
- Primary flow -> tests/evidence: N/A
- Fallback / secondary flow -> tests/evidence: N/A
- Visible states / transitions -> tests/evidence: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节三项与 Evidence Capture 的 `Required` 不得继续保留未填写占位。

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- `DONE` 任务对应的 spec 仍保留未填写占位。
- `orchestrator` 未显式使用 `sandbox_mode = "danger-full-access"` 与 `approval_policy = "never"`，却仍宣称当前仓库运行在 aggressive 基线。
- 代码变更前未记录 `Source Basis`，导致实现依据不可追溯。
- 子 agent 未显式 `approval_policy = "never"`。
- `moderate/complex` 任务通过缩小 `Required Roles` 伪装为 trivial fallback。
- 已使用 `single-agent-fallback`，但 logs/plans 没有单独记录 `Execution Mode` / `Fallback Scope` / `Fallback Reason Code`。
- 需要受角色边界约束的文件系统写命令没有经过 `run_role_guard.py`，只在结案时补跑范围校验。
- `git add -- <explicit paths...>` 仍被包进 wrapper / helper script，导致运行时看不到裸命令前缀。
- `interaction_impact != none`，但 plan/testplan/ui spec 没有定义 primary flow / fallback flow / visible states / evidence coverage。

## Evidence Capture (UI / E2E)
- Required: no
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260409-break-message-copy-centralization/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: 不改变现有焦点与键盘路径
- perf budget: 不新增依赖，不新增运行时网络请求
- error handling / observability: locale generator 对目录不一致直接失败并输出 language code
- security / privacy: 不新增数据读写范围

## Boundary / Invalid Input Cases
- `messages/*.json` 缺少对应 `config/*.json`
- `overrides/*.json` 存在孤儿语言文件
- break 随机提示语列表为空时回退 default prompt

## Concurrency / Race Cases (if applicable)
- 不涉及。

## Mocks & Test Data
- 使用仓库现有 desktop locale 文件与 build 产物。

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`（预期仍可能被无关任务占位阻塞）

## Expected Results
- PASS criteria:
  - `python3 scripts/sync_desktop_locales.py` 成功生成 registry
  - `npm --prefix apps/desktop run typecheck` 通过
  - `npm --prefix apps/desktop run build` 通过
- Outputs to keep (10~20 lines snippet):
  - locale registry build 摘要
  - typecheck / build PASS 摘要
