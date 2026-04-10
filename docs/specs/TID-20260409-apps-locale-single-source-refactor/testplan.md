# Task-ID: TID-20260409-apps-locale-single-source-refactor

## Test Strategy
- Unit:
  - `test/translations.js` 继续校验 `messages/*.json` 全部可解析
  - 新增 `test/desktopBreakCopySource.js`，校验 `ui.breakCopy.*` 存在且 `break-message-copy.*` 已删除
- Integration:
  - `python3 scripts/sync_desktop_locales.py`
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
  - `npm test`
- E2E (if applicable):
  - N/A（本轮不改交互路径，只改文案真源与读取链路）

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `rg -n "break-message-copy" apps/desktop/src test apps/desktop/README.md docs/{RepositoryGuidelines,CodeMap,Architecture,UI,CHANGELOG}.md`（运行时代码应为 0 命中） + `npm --prefix apps/desktop run typecheck`
- AC2 -> `test/desktopBreakCopySource.js` + `python3 scripts/sync_desktop_locales.py`
- AC3 -> `test/desktopBreakCopySource.js` + `rg -n "break-message-copy" apps/desktop/src`
- AC4 -> 文档 grep 审计 + `python3 scripts/validate_workflow_docs.py --mode manual`
- AC5 -> 执行全部命令并记录 PASS

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
- Artifacts path: docs/specs/TID-20260409-apps-locale-single-source-refactor/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: UI 交互不变，不新增可访问性回归
- perf budget: locale lookup 继续复用现有 `i18n.ts`，不新增新的运行时加载链路
- error handling / observability: locale sync、typecheck 与 build 必须能暴露任何残留 import / key 缺失
- security / privacy: 不新增权限、网络请求或敏感数据写入

## Boundary / Invalid Input Cases
- 删除 `break-message-copy.*` 后，运行时代码仍能在 `manualAwaiting` / default prompt / random prompt 三条路径下返回字符串。
- 当随机 prompt 列表为空时，`pickBreakPrompt()` 继续回退到 `defaultPrompt`。
- 非 `desktopReady` 语言缺少 `ui.breakCopy.*` 时，运行时继续沿 fallback 链拿到默认语言文案。

## Concurrency / Race Cases (if applicable)
- N/A。本轮不改并发或计时器逻辑。

## Mocks & Test Data
- 直接使用仓库内真实 `messages/*.json`、`config/*.json` 与生成后的 `registry.generated.json`。

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `npm test`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - locale registry 正常生成
  - `App.tsx` / `break-prompt.ts` 不再引用 `break-message-copy.*`
  - `test/desktopBreakCopySource.js` 通过，证明 `ui.breakCopy.*` 已进入 `messages`
  - docs validator 不再因本任务存在 `TBD/INIT` 占位而失败
- Outputs to keep (10~20 lines snippet):
  - `sync_desktop_locales.py` 的 languages summary
  - `npm test` 的通过统计
  - docs validator 的 PASS 输出
