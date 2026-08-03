# Task-ID: TID-20260724-site-latest-download-route

## Test Strategy
- Unit: 使用临时 jsdom smoke 执行真实 `script.js`，覆盖 newer latest、fetch reject、无匹配资产。
- Integration: 运行发布脚本 `--skip-release`，确认 pinned `index.html`/`targets.js` 幂等；运行 113 项根仓库 Vitest。
- E2E (if applicable): 推送后读取线上页面资源并执行脚本，核对最终 DOM href；对 latest URL 执行 HEAD，确认 302 到 v0.1.4。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> smoke 用 pinned 0.1.4 + API 资产 0.1.5，期望 latest/download/0.1.5，且忽略 API 的 tagged `browser_download_url`。
- AC2 -> smoke 的网络 reject 与仅 checksums.txt 两条用例，均期望 pinned 0.1.4。
- AC3 -> GitHub deployments/status、线上 jsdom DOM probe、latest URL HEAD 302。

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: smoke 和生产 DOM probe 验证 href 从 pinned 收敛到 latest。
- Fallback / secondary flow -> tests/evidence: API reject/无资产保持 pinned。
- Visible states / transitions -> tests/evidence: 按钮始终可点击，无 loading/disabled 视觉态；只透明替换 href。

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
- 高风险任务在 `Execution Safety Block` 已标记风险的情况下，仍把 `Approval needed` 写成 `no`。
- 已使用 `single-agent-fallback`，但 logs/plans 没有单独记录 `Execution Mode` / `Fallback Scope` / `Fallback Reason Code`。
- 需要受角色边界约束的文件系统写命令没有经过 `run_role_guard.py`，只在结案时补跑范围校验。
- `git add -- <explicit paths...>` 仍被包进 wrapper / helper script，导致运行时看不到裸命令前缀。
- `interaction_impact != none`，但 plan/testplan/ui spec 没有定义 primary flow / fallback flow / visible states / evidence coverage。

## Evidence Capture (UI / E2E)
- Required: partial   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260724-site-latest-download-route/evidence/
- What to capture:
  - Screenshots: 无视觉变化，不要求截图。
  - Video/trace (optional): 不要求。
  - HAR/console logs (optional): 保留 smoke 输出、线上 DOM href、GitHub/Vercel deployment SHA 和 302 Location。

## Quality Gates (Non-functional)
- a11y: 原生 anchor、`aria-label` 和 focus 行为不变。
- perf budget: 仍只有一次 GitHub latest API 请求；多次点击复用 promise。
- error handling / observability: 失败显性 console warning，并回退可用链接。
- security / privacy: 不新增 token/权限/数据采集；资产名 URL 编码。

## Boundary / Invalid Input Cases
- 非 2xx、reject、非法/空 assets、仅非 DMG 资产、匹配资产名需要编码。

## Concurrency / Race Cases (if applicable)
- 初始化与点击共享 `latestDownloadUrlPromise`；成功/失败结果都只解析一次，不发生多请求竞争。

## Mocks & Test Data
- pinned `Pauza_0.1.4_aarch64.dmg`；latest mock `Pauza_0.1.5_aarch64.dmg`；空匹配样本 `checksums.txt`；真实 latest 为 v0.1.4。

## Commands to Run
- `node /tmp/pauza-site-download-smoke.mjs`
- `node --check script.js`
- `npm test`（父仓库）
- `python3 scripts/publish_site_release.py --asset .../Pauza_0.1.4_aarch64.dmg --skip-release`
- `python3 scripts/validate_agent_configs.py`
- `python3 scripts/validate_workflow_kit_sync.py`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `curl -I https://github.com/IHKYoung/Pauza/releases/latest/download/Pauza_0.1.4_aarch64.dmg`

## Expected Results
- PASS criteria: 本地三路径、语法、113 项测试、workflow 门禁全部通过；生产 SHA 更新；线上 href 为 latest download；302 Location 为 v0.1.4 tagged asset。
- Outputs to keep (10~20 lines snippet): smoke 三条 PASS、Vitest 113 PASS、validator PASS、deployment SHA/status、最终 href 与 302 Location。
