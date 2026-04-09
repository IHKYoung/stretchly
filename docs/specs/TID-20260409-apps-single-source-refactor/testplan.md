# Task-ID: TID-20260409-apps-single-source-refactor

## Test Strategy
- Unit:
  - `npm test` 覆盖根级 JS helper、translations 与迁移后的 `apps/desktop/legacy-utils` 路径
- Integration:
  - `python3 scripts/sync_desktop_locales.py`
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
- E2E (if applicable):
  - N/A，本任务不改变交互契约

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `rg -n "from '../app|from \"../app|app/locales|app/main\\.js"` 审计 + `npm test`
- AC2 -> `python3 scripts/sync_desktop_locales.py` + `rg -n "overrides|app/locales" apps/desktop/src scripts package.json test README.md`
- AC3 -> 检查 `apps/desktop/src/locales/break-message-copy.json` / `break-message-copy.ts` / `App.tsx` / `break-prompt.ts` 引用链
- AC4 -> 审阅 `README.md`、`apps/desktop/README.md` 与 `docs/{RepositoryGuidelines,CodeMap,Architecture,SettingsInventory,UI}.md`
- AC5 -> `npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`npm test`、`python3 scripts/validate_workflow_docs.py --mode manual`

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
- Artifacts path: docs/specs/TID-20260409-apps-single-source-refactor/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y:
  - 无新增 UI 契约，本轮不引入新的键盘或可访问性回归面
- perf budget:
  - 不新增依赖，不扩大前端运行时代码面
- error handling / observability:
  - locale generator 需对 message/config 漏配给出显式失败
- security / privacy:
  - 不新增外部能力或数据处理路径

## Boundary / Invalid Input Cases
- locale `config/*.json` 与 `messages/*.json` 数量或语言码不匹配
- `break-message-copy.json` 缺失非默认语言时，helper 必须回退到 `zh-CN`
- 根测试仍有旧的 `app/**` import 漏网时，`npm test` 应直接失败而不是静默通过

## Concurrency / Race Cases (if applicable)
- N/A，本任务不改运行时并发模型

## Mocks & Test Data
- 使用仓库内现有 locale JSON、根级测试 fixtures 和迁移后的 `apps/desktop/legacy-utils` 作为测试数据

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `npm test`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 所有验证命令通过
  - 不再存在默认链路对 `app/**` 或 `overrides/` 的真实依赖
- Outputs to keep (10~20 lines snippet):
  - locale sync 成功输出
  - typecheck / build / test / docs validator 的 PASS 摘要
