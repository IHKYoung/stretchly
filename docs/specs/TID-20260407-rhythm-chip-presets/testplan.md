# Task-ID: TID-20260407-rhythm-chip-presets

## Test Strategy
- Unit: 本轮不新增独立单测文件；以现有 React 编译检查保证组件类型与 JSX 合法。
- Integration: 通过 `npm --prefix apps/desktop run typecheck` 与 `npm --prefix apps/desktop run build` 验证设置页前端能完整编译。
- E2E (if applicable): 通过浏览器预览采集节奏页截图和控制台日志，验证 primary flow / fallback flow 在视觉上同时存在。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 浏览器预览截图中应看到微休息/长休息区块的 preset 芯片集合；代码检查确认这四个字段不再使用顶部 stepper。
- AC2 -> 浏览器截图和交互检查确认选中态为白底+阴影，未选中态为浅灰底；Tab focus 不丢失 button 语义。
- AC3 -> 节奏页下半部分“提前提醒 / 延后”仍使用 `CompactNumber` stepper；通过代码检查和截图共同证明。
- AC4 -> `npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、浏览器控制台日志与 evidence README。

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: 浏览器预览截图展示微休息/长休息 preset 区块；必要时通过点击切换证明单击即可选中。
- Fallback / secondary flow -> tests/evidence: 同一截图保留“提前提醒 / 延后” stepper，证明低频设置未被芯片化。
- Visible states / transitions -> tests/evidence: 截图或交互记录覆盖至少一个 selected 芯片态、一个未选中态以及区块开关可见状态。

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
- Required: yes   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260407-rhythm-chip-presets/evidence/
- What to capture:
  - Screenshots: 节奏页默认态，能同时看到微休息/长休息 preset 与下方至少一组 stepper。
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 浏览器控制台日志一份，确认无显著报错。

## Quality Gates (Non-functional)
- a11y: 芯片保持 button 语义并有 focus-visible 样式。
- perf budget: 不新增依赖，不引入明显额外渲染成本。
- error handling / observability: 保存失败仍走原有错误条；控制台无新增异常。
- security / privacy: 无新权限、无新外部请求。

## Boundary / Invalid Input Cases
- preset 集固定后，不再允许核心时间输入超出 preset 的任意数字。
- 长文本 locale 不应破坏 preset 行的点击区和换行节奏。

## Concurrency / Race Cases (if applicable)
- 不适用；仅沿用现有自动保存节流流程。

## Mocks & Test Data
- 使用浏览器 preview 默认 settings 快照，无需额外 mock。

## Commands to Run
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `command -v npx >/dev/null 2>&1`
- Playwright / 浏览器预览截图与控制台采集

## Expected Results
- PASS criteria:
  - 构建命令通过。
  - 视觉证据显示 preset 芯片和 stepper 分工符合需求。
  - 控制台无新的显著错误。
- Outputs to keep (10~20 lines snippet):
  - `typecheck/build` 关键 PASS 输出。
  - 控制台日志摘要。
