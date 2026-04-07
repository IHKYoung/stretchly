# Task-ID: TID-20260402-modern-break-experience

## Test Strategy
- Unit: 为新的 interruption style / break window profile 提供纯逻辑测试
- Integration: 运行现有 Vitest 套件，覆盖 break / display / settings 相关路径
- E2E (if applicable): 不做真实桌面 UI 自动化

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 新增 interruption style helper 测试 + settings 绑定检查
- AC2 -> break renderer / main 改造后通过回归测试与代码检查
- AC3 -> tray menu 新分支通过 lint 和人工代码审查
- AC4 -> `npm run lint` + 关键 renderer DOM 选择器手检
- AC5 -> `npm test` + `npm run lint`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: gentle / balanced / immersive break card 行为通过逻辑测试与代码检查覆盖
- Fallback / secondary flow -> tests/evidence: tray focus session 复用 pause 逻辑；strict mode 继续由现有逻辑兜底
- Visible states / transitions -> tests/evidence: postpone / skip / finish 的显隐逻辑通过 renderer 行为与回归测试验证

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
- Artifacts path: docs/specs/TID-20260402-modern-break-experience/evidence/
- What to capture:
  - Screenshots: 本轮不强制落盘，必要时后补
  - Video/trace (optional): N/A
  - HAR/console logs (optional): CLI lint/test 输出摘要

## Quality Gates (Non-functional)
- a11y: 保持键盘可达与可见 focus
- perf budget: 不新增后台轮询
- error handling / observability: 延续 electron-log settings / break logs
- security / privacy: 不新增网络采集和远程依赖

## Boundary / Invalid Input Cases
- interruption style 取非法值时应回退到可接受默认行为
- break idea 为空时卡片仍可渲染

## Concurrency / Race Cases (if applicable)
- break 进行中切换设置时，不应导致窗口创建链路崩溃

## Mocks & Test Data
- 复用现有 display / settings mock；新增纯函数输入样本

## Commands to Run
- `npm run lint`
- `npm test`

## Expected Results
- PASS criteria:
  - lint 无错误
  - 所有现有测试通过，新增测试通过
- Outputs to keep (10~20 lines snippet):
  - lint/test 结果摘要
