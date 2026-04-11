# Task-ID: TID-20260411-site-typewriter-redesign

## Test Strategy
- Unit:
  - N/A（纯静态页面）
- Integration:
  - `npm run site:dev`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
- E2E (if applicable):
  - 浏览器现实检查首页桌面端、首页移动端和下载 fallback 路由

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 浏览器现实检查首页桌面端截图，确认视觉只剩中央舞台和下载按钮
- AC2 -> 浏览器现实检查打字机循环，确认输入 / 停留 / 退格存在
- AC3 -> 浏览器现实检查 + 本地字体路径验证
- AC4 -> 点击首页下载按钮进入 `/download/releases/`
- AC5 -> 截图落盘 + `python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->

### Primary flow -> tests/evidence
- 首页桌面端 / 移动端截图 + 浏览器现实检查

### Fallback / secondary flow -> tests/evidence
- `/download/releases/` fallback 截图

### Visible states / transitions -> tests/evidence
- typing / hold / deleting / CTA click / download fallback

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
- Artifacts path: docs/specs/TID-20260411-site-typewriter-redesign/evidence/
- What to capture:
  - Screenshots: 首页桌面端、首页移动端、下载页 fallback
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 下载路由缺失目标时的预期 `console.warn`

## Quality Gates (Non-functional)
- a11y: 下载按钮键盘可达；`prefers-reduced-motion` 下停用打字机循环
- perf budget: 纯静态首页，不引入第三方脚本
- error handling / observability: 首页控制台无额外错误；下载目标缺失时只有预期警告
- security / privacy: 无表单、无埋点、无本地存储写入

## Boundary / Invalid Input Cases
- `copy.js` 为空时首页仍需回退到默认句子
- 下载目标为空时首页仍可点击，下载页负责 fallback

## Concurrency / Race Cases (if applicable)
- 重复循环切换文案时，前一轮动画不应残留到下一句

## Mocks & Test Data
- `copy.js` 中的精选提醒文案
- `targets.js` 空 URL 作为下载 fallback 样本

## Commands to Run
- `npm run site:dev`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 首页桌面端与移动端都呈现极简纸面打字机设计
  - 打字机循环工作正常
  - 下载按钮进入既有下载路由
  - workflow docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - 本地静态服务器启动输出
  - workflow docs validator 的 `[OK]`

## Observed Results
- 首页桌面端现实检查通过：控制台 `0 error / 0 warning`
- 首页移动端现实检查通过：`scrollHeight == innerHeight`
- 打字机循环通过：读取到的文本在 7 秒后从 `水杯空了？你的身体可不是仙人掌。` 变为 `检查一下你的坐姿：背挺直了吗？别像一只虾一样蜷着。`
- `/download/releases/` fallback 通过：仅输出预期 `console.warn`
- Evidence 已落盘到 `docs/specs/TID-20260411-site-typewriter-redesign/evidence/README.md`
