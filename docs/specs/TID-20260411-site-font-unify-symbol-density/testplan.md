# Task-ID: TID-20260411-site-font-unify-symbol-density

## Test Strategy
- Unit: N/A（纯静态前端样式与轻脚本微调）
- Integration: 本地打开首页，确认字体统一、输出区宽度保持、粒子密度提升
- E2E (if applicable): Playwright 现实检查 `font-family`、输出区宽度比例、粒子字符采样与 console

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 读取 `body / prompt / button / hint / particle` 的 `font-family`
- AC2 -> 读取 `.prompt-line` 与 `.prompt-mark` 的样式，确认 `Pauza>` 仍是 prompt 但字体同源
- AC3 -> 点击舞台后读取粒子字符列表与计数，确认 `0 / 1 / # / @ / ！ / ¥ / $` 全部出现且数量更高
- AC4 -> 读取 `.typewriter-copy` 宽度与 `window.innerWidth`，确认比例仍为 `0.8`
- AC5 -> `node --check apps/site/script.js` + `npm test` + `python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct
- Primary flow -> tests/evidence: 首页 Playwright DOM 度量验证统一字体、80vw 输出区和更高密度字符粒子
- Fallback / secondary flow -> tests/evidence: reduced-motion 路径通过结构/样式检查兜底，不依赖高频粒子数量
- Visible states / transitions -> tests/evidence: single-font-lxgw、prompt-lxgw、symbol-particles-dense、output-width-80vw-retained

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- 只有主文案是 LXGW，但 `Pauza>`、下载按钮或粒子还在用别的字体。
- 粒子数变多了，但目标字符集合不完整。
- 输出区宽度被意外改离 `0.8`。

## Evidence Capture (UI / E2E)
- Required: partial
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260411-site-font-unify-symbol-density/evidence/
- What to capture:
  - Screenshots: N/A（本轮用 DOM 快照和度量结果取证）
  - Video/trace (optional): N/A
  - HAR/console logs (optional): `console.log`

## Quality Gates (Non-functional)
- a11y: 下载按钮和舞台焦点顺序不变
- perf budget: 粒子数量提高但节点仍能自动回收
- error handling / observability: 首页 console 保持 0 error / 0 warning
- security / privacy: 不新增网络请求和用户数据采集

## Boundary / Invalid Input Cases
- 单字体后 `Pauza>` 不能失去 prompt 感
- 粒子数量增加后不能出现无限堆积

## Concurrency / Race Cases (if applicable)
- 高频移动和点击同时触发时，粒子密度提高后仍需保持自动回收

## Mocks & Test Data
- 使用首页真实 DOM、现有互动逻辑和当前粒子字符池，无需 mocks

## Commands to Run
- `node --check apps/site/script.js`
- `npm test`
- Playwright 现实检查首页 DOM 与交互
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 全站字体统一到 `LXGW WenKai Screen`
  - 输出区宽度比保持 `0.8`
  - 点击 burst 中稳定出现 `0 / 1 / # / @ / ！ / ¥ / $`
  - workflow docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - `font-family` 摘要
  - 粒子字符采样结果
  - docs validator 输出
