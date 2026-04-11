# Task-ID: TID-20260411-site-terminal-shell-refine

## Test Strategy
- Unit: N/A（纯静态结构微调）
- Integration: 本地打开首页，检查结构与样式未破坏既有脚本
- E2E (if applicable): Playwright 现实检查首页桌面端布局与控制台

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 桌面端截图 + DOM 样式审查，确认舞台无卡片边框/底色/阴影
- AC2 -> 桌面端截图 + DOM 位置检查，确认 `Pauza>` 位于左上角且主文案仍居中
- AC3 -> 浏览器现实检查下载按钮、打字机和互动脚本仍可运行
- AC4 -> `python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct
- Primary flow -> tests/evidence: 首页桌面端截图验证左上角 `Pauza>` 与中央输出区层级
- Fallback / secondary flow -> tests/evidence: 移动端与 reduced-motion 路径通过结构审查和样式检查兜底
- Visible states / transitions -> tests/evidence: cardless-stage、shell-header-top-left、centered-output-retained

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
- Required: partial
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260411-site-terminal-shell-refine/evidence/
- What to capture:
  - Screenshots: 首页桌面端 cardless terminal 布局
  - Video/trace (optional): N/A
  - HAR/console logs (optional): console 摘要

## Quality Gates (Non-functional)
- a11y: 舞台与下载按钮的可聚焦顺序不变
- perf budget: 不引入新脚本、不增加持续节点堆积
- error handling / observability: 首页 console 保持 0 error
- security / privacy: 不新增网络请求或数据采集

## Boundary / Invalid Input Cases
- 去卡片后主文案不能飘散到难以聚焦的程度
- 左上角 `Pauza>` 不能把移动端主文案挤出首屏

## Concurrency / Race Cases (if applicable)
- 不适用；本轮不改脚本事件逻辑

## Mocks & Test Data
- 使用现有首页真实文案与交互脚本，无需 mocks

## Commands to Run
- `node --check apps/site/script.js`
- Playwright 现实检查首页布局
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
- 首页成为无卡片的终端式排版，且现有行为不回退
- workflow docs validator 通过
- Outputs to keep (10~20 lines snippet):
- DOM 布局关键数值
- docs validator 输出
