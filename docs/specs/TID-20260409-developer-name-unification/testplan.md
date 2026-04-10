# Task-ID: TID-20260409-developer-name-unification

## Test Strategy
- Unit:
  - `test/translations.js` 覆盖 locale JSON 可解析性
- Integration:
  - `python3 scripts/sync_desktop_locales.py`
  - `npm test`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
- E2E (if applicable):
  - N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `python3 scripts/sync_desktop_locales.py` + `rg -n "Jan Hovancik|Clarke Young" apps/desktop/src/locales/messages apps/desktop/src/locales/registry.generated.json`
- AC2 -> `rg -n "Jan Hovancik|Clarke Young" app/locales`
- AC3 -> `rg -n "Jan Hovancik|Clarke Young" package.json README.md net.hovancik.Pauza.metainfo.xml`
- AC4 -> `rg -n "Jan Hovancik|Clarke Young" LICENSE`

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
- Artifacts path: docs/specs/TID-20260409-developer-name-unification/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y:
  - 无新增交互
- perf budget:
  - 不新增依赖或运行时代码
- error handling / observability:
  - locale sync 可显式暴露生成问题
- security / privacy:
  - 不改密钥、权限或外部服务

## Boundary / Invalid Input Cases
- Turkish locale 的 `developedBy / clarkeY` 顺序需保持可读，不得替换后变成重复或错位短语
- archived locale 与 desktop locale 不得出现一边已替换、一边未替换的漂移

## Concurrency / Race Cases (if applicable)
- N/A

## Mocks & Test Data
- 直接使用仓库内现有 locale JSON 与元数据文件

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `npm test`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - desktop locale source、registry 和仓库元数据中的当前开发者显示名已统一为 `Clarke Young`
  - `LICENSE` 未被修改
- Outputs to keep (10~20 lines snippet):
  - locale sync success
  - test PASS 摘要
  - docs validator PASS
