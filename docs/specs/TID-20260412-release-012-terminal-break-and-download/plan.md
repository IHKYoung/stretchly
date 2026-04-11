# Task-ID: TID-20260412-release-012-terminal-break-and-download

## Summary
- Title: 发布 0.1.2 终端式休息页与下载直链整理
- Date: 2026-04-12
- Level: moderate
- Lane: deep
- Execution Profile: merge-gate
- Status: DONE

## Requirement Brief
- Goal restatement: 将已经确认的 break 页“居中终端打字效果”、官网首页下载直链整理和本地版本号升级一起收口为 `0.1.2` 的一次本地提交，并补齐 workflow docs / changelog 使 commit hooks 可通过。
- In-scope:
  - `apps/desktop` break prompt 的终端式打字展示、微休息单条文案、长休息 `30s` 轮播
  - `apps/site` 首页下载按钮直链化、latest release 解析与 pinned fallback
  - 版本号统一更新到 `0.1.2`
  - 当天 specs、plans、logs、changelog 与本地 git commit
- Out-of-scope:
  - 推送远端、打 tag、创建 GitHub release
  - 修复用户尚未要求纳入本次提交的 `src-tauri/src/{commands,state}.rs` 其他改动
  - 把官网 pinned fallback 强行改成一个尚未在 GitHub 发布的下载地址
- Assumptions:
  - 用户要的是对当前这轮已完成修改做一次干净收口，而不是继续延伸新功能
  - GitHub 最新 release 资产若仍停留在 `v0.1.1`，官网 pinned fallback 应继续指向真实存在的包
  - 已有 break prompt 相关验证足以支撑本次 release 收口
- Risks:
  - 若 workflow docs 继续保留 `TBD/INIT`，commit 会被 pre-commit 拦截
  - 若误把未完成的 `src-tauri` 改动一起暂存，会扩大本次提交边界
  - GitHub API 失败时官网会回退到 pinned `v0.1.1` 链接，这与本地 `0.1.2` 版本线存在短期错位
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 形成一条可追溯的本地 `0.1.2` 收口提交，覆盖 break 终端化体验、官网下载入口收敛和版本元数据统一。

## Scope
- In-scope:
  - `apps/desktop/src/{App.tsx,styles.css}`
  - `apps/desktop/src/lib/break-ideas.ts`
  - `test/desktopBreakIdeas.js`
  - `apps/site/{index.html,script.js,README.md,download/targets.js}`
  - 删除 `apps/site/download/{macos-apple-silicon,macos-intel,releases}/index.html`、`apps/site/download/{redirect.js,styles.css}`
  - `package.json`
  - `apps/desktop/package.json`
  - `apps/desktop/package-lock.json`
  - `apps/desktop/src-tauri/{Cargo.toml,Cargo.lock,tauri.conf.json}`
  - `docs/UI.md`、`docs/CHANGELOG.md`
  - 本 task spec 与 `docs/{plans,logs}/2026-04-12.md`
- Out-of-scope:
  - `apps/desktop/src-tauri/src/{commands.rs,state.rs}` 的未完成用户改动
  - `docs/Architecture.md`、`docs/commits/2026-04-11.md`、`docs/logs/2026-04-11.md`、`docs/plans/2026-04-11.md` 的其他未暂存变更
  - 任意远端发布动作

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/lib/break-ideas.ts`
  - `apps/desktop/src/styles.css`
  - `apps/site/index.html`
  - `apps/site/script.js`
  - `apps/site/download/targets.js`
  - `apps/site/README.md`
  - `package.json`
  - `apps/desktop/package.json`
  - `apps/desktop/src-tauri/tauri.conf.json`
- Related docs/specs/logs reviewed:
  - `docs/UI.md`
  - `docs/CHANGELOG.md`
  - `docs/specs/TID-20260411-release-011-archive-commit/{README,plan,testplan}.md`
  - `docs/{plans,logs}/2026-04-12.md`
  - GitHub `IHKYoung/Pauza` latest release 页面（确认截至 `2026-04-12` 最新已发布版本仍是 `v0.1.1`）
- Why these are sufficient:
  - 已覆盖本次实际修改的桌面端、官网端、版本真源和提交门禁文档，同时补充了 GitHub release 实际发布状态，足以定义本次 commit 边界与下载回退策略。

## Acceptance Criteria (AC)
- AC1: 桌面端 break 页采用居中的终端式打字效果，微休息只展示一条 prompt，长休息整句打完后停留 `30s` 再轮播。
- AC2: 官网首页下载按钮直接触发下载，不再依赖 `/download/*` 中转页面，并且优先解析 GitHub 最新 release 的 `_aarch64.dmg`。
- AC3: 根、desktop、Cargo 与 Tauri 配置中的版本号统一为 `0.1.2`。
- AC4: workflow docs validator 通过，形成一条只包含本次范围的详细中文本地 commit。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: N/A
- Primary flow: N/A
- Fallback / secondary flow: N/A
- Interaction authority / ownership boundary: N/A
- Visible entrypoints / handoff cues: N/A
- In-scope interactions: N/A
- Out-of-scope interactions: N/A
- Interaction acceptance criteria: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节与 Requirement Brief 中的交互字段不得继续保留 `N/A/TBD`

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: merge-gate
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限本地代码/文档收口、版本号提升与 git commit
- touches_running_service: no
- backup_required: no
- backup_plan: `git diff --check` + `npm --prefix apps/desktop run typecheck` + `npm test -- test/desktopBreakCopyLayout.js test/desktopBreakIdeas.js` + `python3 scripts/validate_workflow_docs.py --mode manual`
- rollback_plan: 提交前通过 staged diff 复核范围；如需撤回，后续以本次 commit 为边界执行 `git revert`
- destructive_operations: git commit（用户已明确授权）
- operator_approval_required: no
- rationale: 本次只做本地 merge-gate 收口，不涉及运行中服务、数据迁移、外部成本或提权

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收口 break 页终端式打字体验
  - DoD: break 页正文居中；微休息单条展示；长休息完整显示后停留 `30s` 再轮播。
- [x] Task-2: 收口官网单页下载入口
  - DoD: 首页按钮点击后直接进入下载；旧 `/download/*` 中转页面已删除；API 失败时能回退到 pinned 稳定链接。
- [x] Task-3: 统一版本号与工作流文档，并完成本地提交
  - DoD: 版本元数据统一为 `0.1.2`；`CHANGELOG` / specs / plans / logs 无占位；commit hooks 通过并生成本地 commit。

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260412-release-012-terminal-break-and-download/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: 通过 `docs/plans/2026-04-12.md`、`docs/logs/2026-04-12.md` 和记 commit message 追踪本次 `0.1.2` 收口边界
- Error codes: N/A（本任务不新增运行时错误码）
- Trace/metrics (optional): N/A
- Debug flags (optional): `?window=break` preview 可继续用于人工验证 break prompt 动画与布局

## Risks & Rollback
- Risks:
  - GitHub API 匿名限流或资产命名变化会让官网落回 pinned `v0.1.1` 下载链接
  - 若误暂存其他 `src-tauri` 或旧文档改动，会污染本次提交边界
  - 如果后续用户觉得打字速度仍偏快，需要在下一轮继续调节时序参数
- Rollback plan:
  - 提交前通过 `git diff --cached --name-only` 与 `git diff --cached --stat` 复核范围
  - 提交后若需要回撤，基于本次 commit hash 执行 `git revert`

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 确认 break prompt、官网直链和版本 bump 的实际改动范围。
  2. 补齐 `docs/specs/TID-20260412-release-012-terminal-break-and-download/**`、当天 `plans/logs` 与 `docs/CHANGELOG.md`。
  3. 运行类型检查、针对性测试、workflow docs validator 与 diff 体检。
  4. 只暂存本次相关路径，执行详细中文本地 commit。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户已在 2026-04-12 明确表示“可以，然后总结提交一下”
