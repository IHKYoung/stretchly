# Task-ID: TID-20260409-break-ideas-source-random-fix

## Test Strategy
- Unit:
  - `tBreakIdeaList()` 从 locale ideas 真源读出 `.text` 列表。
  - `pickBreakPromptIndex()` 不再对固定 10m/30m 周期退化成同一索引。
  - `pickBreakPrompt()` 对同一 `startedAtMs` 保持稳定。
- Integration:
  - `desktopBreakCopySource` 守住 `ui.breakCopy.prompts` 已被删除。
- E2E (if applicable):
  - 不需要。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `test/desktopBreakIdeas.js`：locale ideas 真源读取 + 稳定选择
- AC2 -> `test/desktopBreakCopySource.js`：`ui.breakCopy.prompts` 不存在
- AC3 -> `test/desktopBreakIdeas.js`：固定 10m/30m 周期不再锁死同一索引
- AC4 -> `App.tsx` 代码审查 + typecheck/build：`pickBreakPrompt() || defaultPrompt`

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
- Artifacts path: docs/specs/TID-20260409-break-ideas-source-random-fix/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: N/A（无交互结构变化）
- perf budget: prompt 选择为纯函数，不引入新异步或重计算瓶颈
- error handling / observability: ideas 列表为空时必须仍能回退到 default prompt
- security / privacy: 仅本地 locale 文案读取，无新风险

## Boundary / Invalid Input Cases
- locale ideas 缺失或为空
- `startedAtMs` 固定间隔递增（10m / 30m）

## Concurrency / Race Cases (if applicable)
- break 页每秒刷新时，同一 break 不会切换 prompt

## Mocks & Test Data
- 直接使用 `apps/desktop/src/locales/messages/{en,zh-CN}.json` 生成的 locale registry

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `npm test`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - `desktopBreakIdeas.js` 3 tests passed
  - `desktopBreakCopySource.js` 证明 `prompts` 已删除
  - locale sync / typecheck / build / docs validator 全绿
- Outputs to keep (10~20 lines snippet):
  - `Test Files  4 passed (4)`
  - `Tests  58 passed (58)`
