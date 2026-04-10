# Task-ID: TID-20260410-settings-language-switch-freeze

## Test Strategy
- Unit:
  - 为 `resolveUiLanguage()` 增加回归测试，验证“有已持久化语言时先跟 snapshot 走；无 snapshot 时再回退草稿语言”
- Integration:
  - 运行根级 `npm test`
  - 运行 `npm --prefix apps/desktop run typecheck`
  - 运行 `npm --prefix apps/desktop run build`
- E2E (if applicable):
  - 本轮不新增；当前以 helper 单测 + 构建验证前台行为边界

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `test/desktopSettingsControls.js` 中 `keeps the current UI locale until the persisted language save completes`
- AC2 -> `apps/desktop/src/App.tsx` 代码审查 + `npm --prefix apps/desktop run build`，确认轮询 effect 不再依赖语言
- AC3 -> `npm test`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`

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
- Artifacts path: docs/specs/TID-20260410-settings-language-switch-freeze/evidence/
- What to capture:
  - Screenshots:
  - Video/trace (optional):
  - HAR/console logs (optional):

## Quality Gates (Non-functional)
- a11y:
  - 语言下拉键盘行为不回退
- perf budget:
  - 不新增依赖；减少一次因草稿语言触发的轮询 effect 重建
- error handling / observability:
  - 继续沿用现有 load/save error 路径
- security / privacy:
  - 无新增外部输入面

## Boundary / Invalid Input Cases
- `persistedLanguage` 缺失时，仍需从草稿语言归一化出合法 locale
- `zh` 这类别名仍需继续归一化到 `zh-CN`

## Concurrency / Race Cases (if applicable)
- 语言下拉选中后的草稿状态与宿主保存回包并发时，页面可见语言必须继续跟 snapshot，而不是草稿值
- 轮询 effect 不应因为语言草稿变化被销毁/重建

## Mocks & Test Data
- 直接复用仓库内 `registry.generated.json`
- 不需要额外 mock host

## Commands to Run
- `npm test`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`

## Expected Results
- PASS criteria:
  - 相关 59 条 Vitest 用例全部通过
  - `typecheck` 无 TS 错误
  - `build` 成功，允许保留既有 chunk size warning
- Outputs to keep (10~20 lines snippet):
  - `✓ test/desktopSettingsControls.js (4 tests)`
  - `Test Files  4 passed (4)`
  - `Tests  59 passed (59)`
  - `✓ built in 1.79s`
