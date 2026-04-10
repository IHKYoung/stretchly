# Task-ID: TID-20260409-apps-reduction-cleanup

## Test Strategy
- Unit: 无新增单元测试；本任务以现有 desktop 真源测试和引用审计为主。
- Integration: 通过 `npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`npm test` 验证清理未破坏当前默认桌面端链路。
- E2E (if applicable): 不适用；无产品 UI 或运行时交互变更。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `rg -n "legacy-utils" package.json apps/desktop test --glob '!apps/desktop/node_modules/**'` 应不再出现默认链路命中；`npm test`
- AC2 -> `test ! -e coverage && test ! -e examples && test ! -e output`；`git diff --name-status`
- AC3 -> `test -f docs/RootMetadataArchive.md && test ! -e README.md && test ! -e CONTRIBUTING.md && test ! -e CODE_OF_CONDUCT.md && test ! -e LICENSE && test ! -e net.hovancik.Pauza.desktop && test ! -e net.hovancik.Pauza.metainfo.xml`
- AC4 -> `npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`npm test`、`python3 scripts/validate_workflow_docs.py --mode manual`

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
- Artifacts path: docs/specs/TID-20260409-apps-reduction-cleanup/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: N/A，本任务不改产品 UI
- perf budget: 构建通过且不新增运行时代码路径
- error handling / observability: 文档和引用链必须与当前目录一致
- security / privacy: 不新增权限、依赖、外部调用或数据迁移

## Boundary / Invalid Input Cases
- 删除根展示素材后，README/CONTRIBUTING/docs 不能继续保留旧图片引用。
- 删除 legacy-utils 后，根测试不能继续导入已删除模块。
- 删除根级社区/发布元数据后，`docs/RootMetadataArchive.md` 必须能提供恢复所需的核心信息。

## Concurrency / Race Cases (if applicable)
- N/A

## Mocks & Test Data
- N/A

## Commands to Run
- `rg -n "legacy-utils" package.json apps/desktop test --glob '!apps/desktop/node_modules/**'`
- `test ! -e coverage && test ! -e examples && test ! -e output`
- `test -f docs/RootMetadataArchive.md && test ! -e README.md && test ! -e CONTRIBUTING.md && test ! -e CODE_OF_CONDUCT.md && test ! -e LICENSE && test ! -e net.hovancik.Pauza.desktop && test ! -e net.hovancik.Pauza.metainfo.xml`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `npm test`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 当前默认桌面端构建与测试全部通过
  - 仓库根目录不再保留被清理目录与旧 README 素材
  - 当前 spec / plans / logs 不保留占位字段
- Outputs to keep (10~20 lines snippet):
  - `npm test` 的文件数与测试总数摘要
  - typecheck/build/validator 的 PASS 摘要
