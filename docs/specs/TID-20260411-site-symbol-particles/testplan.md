# Task-ID: TID-20260411-site-symbol-particles

## Test Strategy
- Unit: N/A（纯静态前端样式与轻脚本微调）
- Integration: 本地打开首页，确认 prompt、输出区宽度和符号粒子同时成立
- E2E (if applicable): Playwright 现实检查 DOM 样式、宽度比例、粒子字元采样和 console

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 读取 `.prompt-line` / `.prompt-mark` 样式，确认字号与字重提升
- AC2 -> 读取 `.typewriter-copy` 宽度与 `window.innerWidth`，确认比例为 `0.8`
- AC3 -> 点击舞台后采样 `.particle` 文本，确认包含 `0 / 1 / #` 等符号
- AC4 -> 浏览器现实检查打字机、点击提醒和下载按钮继续可用
- AC5 -> `node --check apps/site/script.js` + `python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct
- Primary flow -> tests/evidence: 首页桌面端 DOM 度量 + 浏览器快照，验证更醒目的 `Pauza>`、80vw 输出区和符号粒子
- Fallback / secondary flow -> tests/evidence: reduced-motion 路径通过结构/样式检查兜底，不依赖高频粒子
- Visible states / transitions -> tests/evidence: prompt-emphasized、output-width-80vw、symbol-particles-active

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- `DONE` 任务对应的 spec 仍保留 `TBD/INIT` 占位。
- DOM 看起来变了，但输出区宽度比例没有真正到 `0.8`。
- 粒子样式改了颜色，却仍然是圆点/背景粒子，不是字符粒子。
- `interaction_impact != none`，但 evidence 不覆盖 prompt / width / particle 三项。

## Evidence Capture (UI / E2E)
- Required: partial
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260411-site-symbol-particles/evidence/
- What to capture:
  - Screenshots: N/A（本轮改用 DOM snapshot + 度量结果）
  - Video/trace (optional): N/A
  - HAR/console logs (optional): `console.log`

## Quality Gates (Non-functional)
- a11y: 下载按钮与舞台焦点顺序不变
- perf budget: 只改 CSS 与符号池，不增加持续节点堆积
- error handling / observability: 首页 console 保持 0 error / 0 warning
- security / privacy: 不新增网络请求或用户数据采集

## Boundary / Invalid Input Cases
- 粒子字元需要以 `0 / 1 / #` 为主，但允许少量 `> / / / { } / _` 补充终端感
- 输出区变宽后，长句仍需保持整体居中，不变成左对齐段落

## Concurrency / Race Cases (if applicable)
- 点击与移动同时发生时，符号粒子应正常出现，不影响打字机循环

## Mocks & Test Data
- 使用首页真实 DOM、现有互动逻辑和当前粒子字元池，无需 mocks

## Commands to Run
- `node --check apps/site/script.js`
- Playwright 现实检查首页 DOM 与交互
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - `Pauza>` 更醒目
  - 输出区宽度比为 `0.8`
  - 粒子采样中出现 `0 / 1 / #` 等符号
  - workflow docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - DOM 度量摘要
  - 粒子采样摘要
  - docs validator 输出
