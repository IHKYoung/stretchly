# Task-ID: TID-20260412-site-release-0-1-2

## Test Strategy
- Unit: 不新增单元测试；本次逻辑集中在静态页面脚本与配置，优先做浏览器级回归验证。
- Integration: 通过本地静态服务器加载页面，验证 `download/targets.js`、`index.html` 与 `script.js` 的联动结果。
- E2E (if applicable): 使用浏览器打开首页，确认下载按钮最终 href 与点击跳转目标指向 `0.1.2`。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 检查 `index.html` 与 `download/targets.js` 中的固定 URL 均为 `Pauza_0.1.2_aarch64.dmg`；页面加载后读取 `[data-download-button].href`
- AC2 -> 在真实 GitHub latest 仍为 `0.1.1` 的情况下浏览器加载页面，确认最终 href 仍保持 `0.1.2`
- AC3 -> 运行 `python3 scripts/validate_agent_configs.py` 与 `python3 scripts/validate_workflow_docs.py --mode manual`，并完成 git commit

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: 本地浏览器加载首页后读取下载按钮最终 href，并记录截图与 DOM 结果到 `docs/specs/TID-20260412-site-release-0-1-2/evidence/README.md`
- Fallback / secondary flow -> tests/evidence: 利用当前 GitHub latest 仍为 `v0.1.1` 的实际状态，验证页面不会被旧 latest 响应回退
- Visible states / transitions -> tests/evidence: 记录“初始 pinned href -> 网络请求完成后最终 href 仍为 0.1.2”的检查结果

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
- Artifacts path: docs/specs/TID-20260412-site-release-0-1-2/evidence/
- What to capture:
  - Screenshots: 首页首屏，包含右上角下载按钮
  - Video/trace (optional): 不要求
  - HAR/console logs (optional): 记录浏览器检查到的最终 href 与当前 GitHub latest 仍为 `v0.1.1` 的事实

## Quality Gates (Non-functional)
- a11y: 下载按钮原生 `<a>` 语义、`aria-label` 与 focus 样式不回退。
- perf budget: 不新增额外请求数量，只保留现有一次 GitHub latest 请求。
- error handling / observability: API 异常时继续回退到 pinned href，并保留现有 `console.warn`。
- security / privacy: 不新增任何权限、token 或用户数据处理。

## Boundary / Invalid Input Cases
- GitHub latest API 返回旧资产名：最终 href 仍应保持 `0.1.2`
- GitHub latest API 不可用：最终 href 仍应保持 `0.1.2`
- `fallbackUrl` 为空：不属于本次范围，但当前实现仍会回退到初始 `<a href>`

## Concurrency / Race Cases (if applicable)
- `latestDownloadUrlPromise` 只缓存一次请求结果；多次点击不应触发多个结果互相覆盖。
- 旧版本 API 响应与初始 pinned href 的竞争，最终应由 basename 比对逻辑保证 `0.1.2` 胜出。

## Mocks & Test Data
- 本地静态页面使用真实 GitHub latest API 响应作为回归样本；当前远端 `git ls-remote --tags origin` 仅返回 `v0.1.1`。

## Commands to Run
- `python3 scripts/validate_agent_configs.py`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `python3 -m http.server 43211 --bind 127.0.0.1`
- 浏览器打开 `http://127.0.0.1:43211/`，检查 `[data-download-button].href`

## Expected Results
- PASS criteria: workflow validator 通过；页面最终 href 为 `Pauza_0.1.2_aarch64.dmg`；旧 latest 响应不会把按钮回退到 `0.1.1`
- Outputs to keep (10~20 lines snippet): validator PASS 摘要、浏览器检查到的最终 href、远端 tag 仅有 `v0.1.1` 的证据
