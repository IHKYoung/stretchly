# Task-ID: TID-20260409-root-readme-run-build-guide

## Test Strategy
- Unit: N/A（纯文档任务）
- Integration: 通过命令 grep / 文件存在性校验 README 与脚本真源一致
- E2E (if applicable): N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `test -f README.md` + `rg -n 'npm start|npm run desktop:build|universal-apple-darwin|target/release/bundle' README.md`
- AC2 -> `rg -n 'apps/desktop' README.md`
- AC3 -> `rg -n 'README.md' docs/RepositoryGuidelines.md docs/CodeMap.md`
- AC4 -> `python3 scripts/validate_workflow_docs.py --mode manual`

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
- Artifacts path: docs/specs/TID-20260409-root-readme-run-build-guide/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional):

## Quality Gates (Non-functional)
- a11y: 文档命令块可直接复制，无过长段落掩盖核心命令
- perf budget: N/A
- error handling / observability: README 中应明确哪些命令只是转发，哪些产物路径是实际输出位置
- security / privacy: 不暴露具体签名密钥值，只描述环境变量前提

## Boundary / Invalid Input Cases
- README 不应重新引用 `app/` 作为默认运行或打包入口。
- README 不应写入仓库中不存在的命令，如旧 Electron 打包链。

## Concurrency / Race Cases (if applicable)
- N/A

## Mocks & Test Data
- N/A

## Commands to Run
- `test -f README.md`
- `rg -n 'npm start|npm run dev|npm run desktop:build|npm run pack|universal-apple-darwin|target/release/bundle' README.md`
- `rg -n 'README.md' docs/RepositoryGuidelines.md docs/CodeMap.md`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - README 存在且命令完整
  - 索引文档已同步 README 职责
  - workflow docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - README grep 命中关键命令
  - validator `[OK]` 输出
