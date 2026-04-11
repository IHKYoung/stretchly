# Task-ID: TID-20260411-site-landing-page

## Test Strategy
- Unit:
  - N/A（纯静态页面，无独立业务函数测试框架）
- Integration:
  - 本地静态服务器预览 `apps/site`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
- E2E (if applicable):
  - 浏览器现实检查首页与下载跳转页
  - 截图首页首屏和下载页 fallback / loading 状态

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 本地静态服务器能直接打开 `apps/site/index.html`
- AC2 -> 浏览器现实检查首页文案、信息层级与下载 CTA
- AC3 -> 浏览器现实检查 `/download/macos-apple-silicon/` 与 `/download/macos-intel/`
- AC4 -> 浏览器现实检查动画、hover 与 pointer spotlight；截图留证
- AC5 -> `python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: 首页首屏截图 + 浏览器交互检查
- Fallback / secondary flow -> tests/evidence: 下载跳转页 fallback / loading 截图 + 浏览器交互检查
- Visible states / transitions -> tests/evidence: reveal、CTA hover、redirect loading、redirect fallback
- Validator expectation: direct interaction 覆盖已补齐。

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
- Artifacts path: docs/specs/TID-20260411-site-landing-page/evidence/
- What to capture:
  - Screenshots: 首页首屏、下载页 fallback / loading
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 下载页未配置目标时的控制台警告

## Quality Gates (Non-functional)
- a11y: 键盘可达、focus ring、对比可读
- perf budget: 纯静态首屏，无第三方脚本和大资源阻塞
- error handling / observability: 下载地址未配置时必须有明确 fallback
- security / privacy: 无登录、无采集、无跨站表单

## Boundary / Invalid Input Cases
- 目标 URL 为空时，下载页必须停在 fallback 状态
- 用户阻止自动跳转时，页面仍需保留手动打开按钮

## Concurrency / Race Cases (if applicable)
- 多次点击下载按钮时只影响浏览器导航，不引入额外站内状态竞争

## Mocks & Test Data
- 使用 `targets.js` 空 URL 作为 fallback 样本

## Commands to Run
- `python3 -m http.server 43210 --directory apps/site`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - 首页单屏可读、动画不喧宾夺主
  - 下载 CTA 能进入站内跳转页
  - 跳转页在无目标地址时显示清晰 fallback
  - workflow docs validator 通过
- Outputs to keep (10~20 lines snippet):
  - 本地静态服务器访问正常
  - workflow docs validator 的 `[OK]`

## Observed Results
- 首页桌面端现实检查通过：控制台 `0 error / 0 warning`
- 下载路由现实检查通过：三条稳定路由均可打开；`targets.js` 为空时仅输出预期 `console.warn`
- Evidence 已落盘到 `docs/specs/TID-20260411-site-landing-page/evidence/README.md`
