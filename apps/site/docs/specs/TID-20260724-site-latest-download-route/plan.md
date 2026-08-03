# Task-ID: TID-20260724-site-latest-download-route

## Summary
- Title: 官网 latest 下载路径修复与部署
- Date: 2026-07-24
- Level: moderate
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复并部署官网，让下载按钮通过 GitHub latest API 获取资产名并拼接 latest download 路径。
- In-scope: 下载配置、解析逻辑、0.1.4 fallback、README、任务审计、独立官网分支推送与线上核验。
- Out-of-scope: 页面视觉、桌面端代码、现有 Release/tag/DMG、父仓库分支历史。
- Assumptions: latest release 持续提供一个 `_aarch64.dmg`；Vercel Production 监听 `IHKYoung/Pauza:baseline`。
- Risks: API 限流或资产命名变化会触发 fallback；错误分支推送会污染官网仓库，因此只在 `apps/site` 内提交。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 页面加载后 href 从 0.1.4 pinned 更新为 latest download 路径；点击由 GitHub 302 到 latest DMG。
- Fallback / secondary flow: API 失败或没有匹配资产时，按钮继续下载 pinned 0.1.4。
- User-visible boundary: 仅首页右上角“下载”按钮的最终导航地址。
- Key visible states / transitions: 可用 pinned -> resolving（仍可用）-> latest；失败则 pinned -> pinned。

## Goal
- 官网无需每次因版本名不一致拒绝 latest，并在本次部署后下载 0.1.4。

## Scope
- In-scope: `index.html`、`download/targets.js`、`script.js`、`README.md`、本 Task-ID 文档、daily plan/log、CHANGELOG 和 evidence。
- Out-of-scope: `copy.js`、现有模板改动、父仓库 `.gitignore`/`scratch`、桌面端与 Release 资产。

## Source Basis (Read Before Code)
- Related code/files reviewed: `index.html`、`download/targets.js`、`script.js`、`scripts/publish_site_release.py`、独立 `.git/config` 与分支状态。
- Related docs/specs/logs reviewed: `README.md`、`docs/RepositoryGuidelines.md`、0.1.2 下载 spec/evidence、Release Playbook、Vercel deployment 和 GitHub latest metadata。
- Why these are sufficient: 覆盖静态入口、运行时状态所有者、失败回退、发布脚本、部署分支及线上实际响应。

## Acceptance Criteria (AC)
- AC1: latest API 返回匹配资产名时，最终 URL 为 `https://github.com/IHKYoung/Pauza/releases/latest/download/<asset-name>`，不再比较 pinned 资产名。
- AC2: API 非 2xx/reject 或无 `_aarch64.dmg` 时，最终 URL 保持 pinned 0.1.4。
- AC3: `baseline` 推送后 Vercel Production 成功，线上最终 DOM 指向 `releases/latest/download/Pauza_0.1.4_aarch64.dmg`，该 URL 302 到 v0.1.4。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: ACTIVE；视觉、文案、动效、键盘与 focus 行为冻结。
- Primary flow: 初始 pinned 可点击，latest 解析成功后透明替换 href，点击进入 GitHub latest redirect。
- Fallback / secondary flow: 请求失败或空资产保持 pinned，不显示虚假错误页或禁用按钮。
- Interaction authority / ownership boundary: GitHub API 决定最新资产；前端只筛选资产名、编码并展示路径。
- Visible entrypoints / handoff cues: 首页右上角原生 `<a data-download-button>`。
- In-scope interactions: 页面加载后的 href 更新和点击导航。
- Out-of-scope interactions: 按钮样式、页面文案/粒子/打字机、下载后的安装流程。
- Interaction acceptance criteria: 正常路径命中 latest；失败路径仍能下载 pinned；无新增视觉状态。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: architect,scribe,coder,tester,evidence_collector
- Delegation Policy: after the user grants orchestration authority, `orchestrator` may autonomously `spawn_agent`
- Automatic Delegation Triggers: `moderate/complex` | long-running task | independent sidecar tasks
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: yes，改变生产官网下载目标解析。
- touches_running_service: yes，推送触发 Vercel Production。
- backup_required: yes，Git/Vercel 历史即备份。
- backup_plan: 保留远端 `bad94ee` 和其成功 Vercel deployment。
- rollback_plan: revert 新提交并推送，或 Vercel 回滚上一生产部署。
- destructive_operations: none。
- operator_approval_required: yes。
- rationale: 用户于 2026-07-24 明确要求官网更新并使用 latest 拼接下载路径。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 修复 latest 解析与 fallback 契约。
  - DoD: 三条行为 smoke、语法、diff 与 113 项测试通过。
- [x] Task-2: 推送独立官网分支并验证生产。
  - DoD: Vercel deployment 成功，线上 DOM 与 302 目标留证。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260724-site-latest-download-route/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: 请求期间 href 保持 pinned 可点击。
  - empty: API 无匹配资产时保持 pinned。
  - error: fetch reject 时保持 pinned 并打印 warning。
  - disabled: 不引入 disabled 状态。
  - success: 最终 href 为 latest download 0.1.4，且 HEAD 返回 302 到 tag URL。

## Observability / Debug Plan
- Logs: 保留 console warning；记录 curl、smoke、测试和 deployment 元数据。
- Error codes: GitHub API 非 2xx 统一进入 fallback，不合成业务错误码。
- Trace/metrics (optional): GitHub deployment status 与 Vercel target URL。
- Debug flags (optional): 无。

## Risks & Rollback
- Risks: API rate limit、资产后缀变化、Vercel 未自动部署、误纳入独立工作树的用户改动。
- Rollback plan: 只显式 stage 本任务路径；需要回退时 revert 本任务提交或恢复上一 Vercel deployment。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 核对远端分支、Release API、latest redirect 和 Vercel 旧部署。
  2. 修改配置/解析/说明并运行本地回归。
  3. 仅 stage 本任务文件，通过 workflow hook 后提交并推送 `baseline`。
  4. 轮询 Vercel deployment，核对线上运行时 href 与下载 302，补齐 evidence/audit。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no；仅高风险动作写 yes -->
- Approved: yes；用户于 2026-07-24 明确要求“官网进行更新，然后应该使用latest拼接下载路径”。
