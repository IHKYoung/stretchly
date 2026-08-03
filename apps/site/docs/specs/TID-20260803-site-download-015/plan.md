# Task-ID: TID-20260803-site-download-015

## Summary
- Title: 更新官网下载目标到 Pauza 0.1.5
- Date: 2026-08-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: IN_PROGRESS

## Requirement Brief
- Goal restatement: 将官网下载按钮的固定回退从 `v0.1.4` 更新到 `v0.1.5`，并验证 latest 和 pinned 两条生产路径都可下载同一 DMG。
- In-scope: `index.html`、`download/targets.js`、README、CHANGELOG、任务/审计文档、站点测试、push 和生产回验。
- Out-of-scope: 视觉改版、文案池、Release Playbook、模板格式、桌面端构建、删除/覆盖部署。
- Assumptions: `IHKYoung/Pauza` 的 `v0.1.5` Release 在站点 push 前已创建并完成 hash 回验；Vercel 继续跟踪 `baseline`。
- Risks: latest 缓存、fixed URL 拼错、部署失败、误暂存既有 dirty 文件。
- Interaction impact: indirect
- Primary visible flow: 点击下载 -> latest API -> `Pauza_0.1.5_aarch64.dmg`。
- Fallback / secondary flow: latest 失败 -> pinned v0.1.5；两者均失败 -> 既有错误提示。
- User-visible boundary: 官网下载按钮到浏览器开始 GitHub asset 下载。
- Key visible states / transitions: idle -> loading -> redirect；latest error -> fallback；fallback error -> visible error/retry。

## Goal
- 让官网在本次 Release 后不再把任何下载回退指向 `v0.1.4`。

## Scope
- In-scope: 两个下载目标文件、说明/审计、静态与生产验证。
- Out-of-scope: 其余站点行为、样式、内容和已有用户改动。

## Source Basis (Read Before Code)
- Related code/files reviewed: `index.html`、`download/targets.js`、`script.js`、`scripts/publish_site_release.py`、当前 git status/remote/branch ancestry。
- Related docs/specs/logs reviewed: `README.md`、`docs/CHANGELOG.md`、`docs/ReleasePlaybook.md`、`TID-20260724-site-latest-download-route` 及其生产 evidence、根 `v0.1.5` 发布 task。
- Why these are sufficient: 覆盖下载入口、运行时解析契约、发布工具、既有生产行为和本轮资产权威，且明确了不可纳入的 dirty 文件。

## Acceptance Criteria (AC)
- AC1: `index.html` 与 `download/targets.js` 的 pinned URL 完全一致并指向 `v0.1.5` arm64 DMG。
- AC2: latest API 仍是主路径，loading/error/fallback 行为不变，静态检查和 smoke test 通过。
- AC3: 提交只包含列明的站点文件及本任务治理文件，既有 4 个 dirty 文件不进入提交。
- AC4: 站点分支仅快进 push，生产 HTML 与两条 GitHub 下载 URL 均验证可达 v0.1.5。

## Interaction Freeze
- Freeze status: frozen
- Primary flow: 下载按钮通过 latest release API 选择 `_aarch64.dmg` 并构造 latest download URL。
- Fallback / secondary flow: API/响应/资产异常时使用固定 v0.1.5 URL，固定跳转失败显示既有错误。
- Interaction authority / ownership boundary: GitHub Release 是版本/资产事实源；站点只解析、回退和呈现。
- Visible entrypoints / handoff cues: 右上角下载按钮、解析状态、错误提示、浏览器下载。
- In-scope interactions: URL 目标与真实网络回验。
- Out-of-scope interactions: 按钮样式、文字、动效和站点其他互动。
- Interaction acceptance criteria: primary/fallback 同版、同架构、同资产，线上不再回到 v0.1.4。
- Validator expectation: 本节全部字段已冻结。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe,reality_checker
- Delegation Policy: 当前会话规则禁止未获请求时启动 sub-agent。
- Automatic Delegation Triggers: 被更高优先级会话规则覆盖。
- Execution Mode Policy: 有界 single-agent fallback。
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`；本任务不启动 sub-agent。
- Escalation Route: orchestrator -> direct local execution；远端冲突或破坏性需求 -> 用户。
- Safe-local Command Route: orchestrator 以显式 path 执行 staging/validation。
- Approval Packet Fields: command / purpose / risk / rollback。

## Execution Safety Block
- service_impact: 更新官网生产下载兜底。
- touches_running_service: yes
- backup_required: no；git 历史和当前 v0.1.4 URL 提供可恢复基线。
- backup_plan: 记录更新前 commit、dirty paths、production URL 与 v0.1.4 fallback。
- rollback_plan: 使用后续 revert 将 pinned URL 恢复 v0.1.4；不删除部署或改写历史。
- destructive_operations: none。
- operator_approval_required: yes。
- rationale: 用户明确要求官网发布下载。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 更新 pinned 下载目标和说明
  - DoD: 两份 URL 一致为 v0.1.5，README/CHANGELOG/任务文档同步。
- [ ] Task-2: 验证、提交、推送和生产回验
  - DoD: 静态/interaction/workflow gates 通过，提交无混入，生产 latest/fallback 可达。

## Evidence Plan (UI / E2E)
- Evidence required: yes
- Owner: orchestrator
- Artifact path: docs/specs/TID-20260803-site-download-015/evidence/
- Interaction validation note: 记录源码断言、GitHub latest/fixed HTTP、Vercel deployment 与生产 HTML。
- Required states to capture:
  - loading: 源码/测试确认现有 loading 状态不变。
  - empty: latest 无匹配资产时走 pinned。
  - error: 双路径失败保留可见错误。
  - disabled: 请求期间保持既有防重复触发。
  - success: production 下载到 v0.1.5。

## Observability / Debug Plan
- Logs: git diff/status、validator、GitHub API、HTTP headers、Vercel metadata。
- Error codes: command exit status 与 HTTP status。
- Trace/metrics: site commit -> deployment -> production HTML -> GitHub asset。
- Debug flags: 不开启敏感 verbose trace。

## Risks & Rollback
- Risks: asset 尚未发布、缓存、部署失败、dirty 文件混入。
- Rollback plan: 发布前失败停止；站点问题用普通 revert 恢复 v0.1.4 fixed URL。

## Sequential Phases
- phase_execution: N/A
- phase_confirmation_policy: N/A
- phase_stop_conditions:
  - N/A（single-task 由 Acceptance Criteria 和 Safety Block 控制）

## Execution Plan
- Steps:
  1. 更新下载目标、README、CHANGELOG 与治理文档。
  2. 本地静态/interaction/workflow 验证并显式 staging。
  3. 等 v0.1.5 Release 回验通过后提交、快进 push。
  4. 验证 Vercel 生产 HTML、latest/fixed URL 与实际 asset。

## Definition of Done (DoD)
- 任务文档无占位字段，授权、风险和回滚可追溯。
- 下载目标、提交范围和生产证据可复现。

## Approval
- Approval needed: yes
- Approved: yes；用户明确要求 GitHub Release 管理并在官网提供下载。
