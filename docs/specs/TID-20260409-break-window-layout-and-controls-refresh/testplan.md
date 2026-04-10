# Task-ID: TID-20260409-break-window-layout-and-controls-refresh

## Test Strategy
- Unit:
  - 无独立纯函数单测；以 break 页面渲染路径审查为主
- Integration:
  - `npm test`
  - `npm --prefix apps/desktop run build`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
- E2E (if applicable):
  - 当前未保留专门录屏；真实 break 页面截图可在后续设计回顾或官网素材阶段补采

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `App.tsx` break 页面结构审查 + desktop build
- AC2 -> `App.tsx` 的 countdown / meter 渲染路径 + desktop build
- AC3 -> `App.tsx` 中 `manualAwaiting` / `canPostpone` 条件渲染 + `npm test`
- AC4 -> `npm test`; `npm --prefix apps/desktop run build`; `python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: 单列布局、数字倒计时与细进度条由 `App.tsx` 结构和 `evidence/README.md` 覆盖。
- Fallback / secondary flow -> tests/evidence: `manualAwaiting` / `canPostpone` 驱动的 CTA 渲染由 `App.tsx` 条件路径与 `npm test` / build 兜底。
- Visible states / transitions -> tests/evidence: `break start -> active countdown -> manualAwaiting / canPostpone -> exit` 的前台可见状态已在 UI spec、plan 与 evidence README 中对照记录。
- Validator expectation: 交互字段和 Evidence Capture 已补齐。

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
- Required: partial   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260409-break-window-layout-and-controls-refresh/evidence/
- What to capture:
  - Screenshots: 当前未保留
  - Video/trace (optional): 无
  - HAR/console logs (optional): 无

## Quality Gates (Non-functional)
- a11y: 焦点顺序与按钮语义保持现状，只减少不允许的 CTA
- perf budget: 纯前台布局收敛，不新增运行时依赖
- error handling / observability: 沿用现有 command error / busyAction
- security / privacy: 不新增权限、网络或数据处理

## Boundary / Invalid Input Cases
- `manualAwaiting = false` 时不应出现 `Resume work`
- `canPostpone = false` 时不应出现 `Later`
- break 页面不应因为移除 `Skip` 而丢失现有允许动作

## Concurrency / Race Cases (if applicable)
- break snapshot 更新与用户点击 CTA 同时发生时，前台仍应以最新 snapshot 决定按钮可见性

## Mocks & Test Data
- 使用现有 `DesktopSnapshot.currentBreak` 前台数据结构与 release 时的 build/test 验证链路

## Commands to Run
- `npm test`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
- Outputs to keep (10~20 lines snippet):
- PASS criteria:
  - break 页面结构保持单列主视觉
  - `Skip` 不再出现在主界面
  - build/test/docs gate 通过
- Outputs to keep (10~20 lines snippet):
  - `Test Files  4 passed (4)`
  - `Tests  59 passed (59)`
  - `✓ built in 1.92s`
