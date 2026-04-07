# Task-ID: TID-20260403-repo-cleanup-foundation

## Test Strategy
- Unit: 不涉及运行时单元逻辑变更，不新增单测。
- Integration: 用引用搜索与 `git diff` 验证删除边界。
- E2E (if applicable): 不适用。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `rg` 检查当前产品主体对删除文件无直接引用；`git diff -- <files>` 确认删除范围。
- AC2 -> `python3 scripts/validate_workflow_docs.py --mode manual` 确认文档同步完成。

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
- Artifacts path: docs/specs/TID-20260403-repo-cleanup-foundation/evidence/
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
- 不删除 `app/**`、`test/**`、`package.json`、`build/**` 等仍在当前运行路径上的文件。

## Concurrency / Race Cases (if applicable)
- N/A

## Mocks & Test Data
- N/A

## Commands to Run
- `rg -n "danger-color-tester|publish-snap|stretchly.service|pauza.service|net\\.hovancik\\.(Stretchly|Pauza)\\.(desktop|metainfo)|graphics/v0|build/appx|installerSidebar|icon\\.ico|icon\\.icns" . -g '!node_modules' -g '!.git'`
- `git diff -- Dockerfile docker-compose.yml publish-snap.md danger-color-tester.html docs/CodeMap.md docs/RepositoryGuidelines.md docs/CHANGELOG.md`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria: 删除边界与计划一致；workflow docs 校验通过；无核心代码引用被破坏。
- Outputs to keep (10~20 lines snippet): 保存 `git diff` 删除片段与 workflow docs validator 的 `[OK]` 输出。
