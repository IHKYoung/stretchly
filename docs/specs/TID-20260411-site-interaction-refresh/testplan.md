# Task-ID: TID-20260411-site-interaction-refresh

## Test Strategy
- Unit: N/A（纯静态站点，本轮不新增独立单测）
- Integration: 本地启动 `npm run site:dev`，验证首页脚本、布局和下载按钮在真实浏览器中正常工作
- E2E (if applicable): 使用 Playwright 现实检查桌面端 / 移动端首页，覆盖移动、点击和下载入口可达性

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 审查 `apps/site/script.js` 的 hold 时长配置 + 浏览器现实检查单条文案打完后停留约 10 秒
- AC2 -> 桌面端截图 / DOM 尺寸审查，确认主舞台水平居中且宽度约等于视口 80%
- AC3 -> 浏览器现实检查鼠标移动、点击、滚动时的粒子/跟随反馈
- AC4 -> 浏览器现实检查点击后出现调侃短句气泡，且文案来自扩充后的池
- AC5 -> 移动端截图 + `scrollHeight <= innerHeight` 检查
- AC6 -> `npm test` + `python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct

### Primary flow -> tests/evidence
- 桌面端首页截图 + 现实检查，验证居中舞台、10 秒 hold、点击气泡和粒子反馈

### Fallback / secondary flow -> tests/evidence
- reduced-motion / 小屏路径以移动端现实检查和脚本审查兜底

### Visible states / transitions -> tests/evidence
- typewriter-hold、pointer-aura、click-burst、nudge-bubble、download-button

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
- Required: yes
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260411-site-interaction-refresh/evidence/
- What to capture:
  - Screenshots: 首页桌面端、首页移动端、点击气泡或粒子反馈至少一张
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 浏览器 console 摘要

## Quality Gates (Non-functional)
- a11y: 下载按钮和舞台交互提示在键盘聚焦下仍可见；交互增强不应破坏文本可读性
- perf budget: 单页静态站点不引入新依赖；粒子节点应自动回收，不造成持续堆积
- error handling / observability: 首页 console 保持 0 error；`targets.js` 未配置时不影响首页
- security / privacy: 不新增网络请求和用户数据采集

## Boundary / Invalid Input Cases
- 超长提醒文案需要在居中舞台内自然换行，不能把页面重新挤成偏左布局
- 连续快速点击时，提醒气泡和粒子需要可回收，不能无限堆积
- 点击下载按钮时不应被互动脚本阻断

## Concurrency / Race Cases (if applicable)
- 连续打字轮换与点击气泡同时发生时，typed output 不应闪烁或被重置
- 高频移动鼠标时，粒子数量应受节流/限额控制

## Mocks & Test Data
- 使用 `apps/site/copy.js` 中的真实文案池与新增短句池作为测试数据

## Commands to Run
- `npm run site:dev`
- `npm test`
- Playwright 现实检查首页桌面端 / 移动端
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 首页在桌面端与移动端都保持视觉稳定
  - 打字机 hold 时间、居中布局与交互反馈全部符合 AC
  - 自动化命令和 docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - `npm test` 摘要
  - docs validator 输出
  - 浏览器现实检查关键结论
