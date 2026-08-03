# Task-ID: TID-20260803-site-download-015

## Test Strategy
- Unit: `node --check` 站点 JS。
- Integration: 静态断言 primary/fallback 配置、既有 interaction smoke、workflow validators、staged diff 审查。
- E2E: GitHub latest/fixed URL、Vercel deployment、生产 HTML 与实际 asset HTTP 回验。

## Acceptance Criteria Coverage Map (AC -> Tests)
- AC1 -> 两文件 URL grep/Node 断言。
- AC2 -> JS syntax 与 interaction smoke。
- AC3 -> `git status` / `git diff --cached --name-only`。
- AC4 -> remote ancestry、push、Vercel/production/GitHub HTTP。

## Interaction Contract Coverage
- Interaction impact: indirect
- Primary flow -> tests/evidence: latest API tag/assets 与 production latest download。
- Fallback / secondary flow -> tests/evidence: fixed URL 静态断言和 direct HTTP。
- Visible states / transitions -> tests/evidence: 既有 script smoke 覆盖 loading/fallback/error，源码未改交互逻辑。
- Validator expectation: evidence 为 required。

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- 只更新一个 URL；只验证 API 不验证 fixed；只验证源码不验证生产；提交混入 4 个既有 dirty 文件。

## Evidence Capture (UI / E2E)
- Required: yes
- Owner: orchestrator
- Artifacts path: docs/specs/TID-20260803-site-download-015/evidence/
- What to capture:
  - Screenshots: 浏览器 backend 可用时采集，否则以生产 HTML/HTTP 代替并记录限制。
  - Video/trace: 不需要。
  - HAR/console logs: 可选；保留 curl headers 与 metadata。

## Quality Gates (Non-functional)
- a11y: DOM/label/focus 不变。
- perf budget: 不新增资源或依赖。
- error handling / observability: latest 失败仍回退，双失败仍可见。
- security / privacy: 不记录 token。

## Boundary / Invalid Input Cases
- latest 无匹配资产、HTTP 失败、tag 错误或 v0.1.5 asset 尚不存在时不得部署。

## Concurrency / Race Cases
- push 前 fetch 并确认 fast-forward；Release/site deployment 事实分别回验。

## Mocks & Test Data
- 本地 smoke 可模拟 latest success/empty/error；最终以真实 v0.1.5 Release 和生产站点为准。

## Commands to Run
- `node --check script.js && node --check download/targets.js`
- 既有 Node 下载 resolver smoke test。
- `python3 scripts/validate_agent_configs.py`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `git diff --check` 与显式 staged diff。
- `gh release view v0.1.5 --repo IHKYoung/Pauza`、`curl` latest/fixed/production。

## Expected Results
- PASS criteria: AC1-AC4 全部满足，无 v0.1.4 下载残留，无无关 staged 文件，生产路径可达 v0.1.5。
- Outputs to keep: commit SHA、deployment URL/status、production HTML target、GitHub tag/asset/HTTP status。
