# Task-ID: TID-20260411-root-scripts-cleanup

## Test Strategy
- Unit: `node -e` 解析根 `package.json` 并读取 scripts 键集合
- Integration: 运行根级 `npm test`、`npm run typecheck`，验证收敛后的入口仍可用
- E2E (if applicable): N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `node -e "const pkg=require('./package.json'); console.log(Object.keys(pkg.scripts).join(','))"` + `rg -n '"(start|pack|dist|test-single|prepublishOnly|postpublish)"' package.json`
- AC2 -> `rg -n 'npm run dev|npm run build|npm run typecheck|npm run test:coverage|npm run desktop:build' README.md docs/RepositoryGuidelines.md docs/CodeMap.md`
- AC3 -> `rg -n 'npm run test:coverage' .github/workflows/tests.yml`
- AC4 -> `npm test` + `npm run typecheck` + `python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: none
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
- Required: no
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260411-root-scripts-cleanup/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional):

## Quality Gates (Non-functional)
- a11y: N/A
- perf budget: 不新增依赖或构建步骤
- error handling / observability: 脚本缺失/命名错误应通过 `npm` 直接暴露，不允许文档与 CI 继续引用已删除命令
- security / privacy: 不引入新权限或外部网络调用

## Boundary / Invalid Input Cases
- 历史文档仍可保留旧命令作为历史记录，但当前有效入口文档和 workflow 不得继续引用已删除脚本
- `prepare` / `postinstall` 不应被本次清理误删，否则会影响现有安装与 hooks 初始化

## Concurrency / Race Cases (if applicable)
- N/A

## Mocks & Test Data
- N/A

## Commands to Run
- `node -e "const pkg=require('./package.json'); console.log(Object.keys(pkg.scripts).join('\n'))"`
- `rg -n '"(start|pack|dist|test-single|prepublishOnly|postpublish)"' package.json`
- `rg -n 'npm run dev|npm run build|npm run typecheck|npm run test:coverage|npm run desktop:build' README.md docs/RepositoryGuidelines.md docs/CodeMap.md .github/workflows/tests.yml`
- `npm test`
- `npm run typecheck`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 根 scripts 集合中只保留收敛后的命令
  - 当前有效 docs / workflow 不再命中已删除脚本
  - `npm test`、`npm run typecheck`、workflow docs validator 全部通过
- Outputs to keep (10~20 lines snippet):
  - scripts 键集合输出
  - `npm test` / `npm run typecheck` / docs validator 摘要
