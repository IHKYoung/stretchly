# Task-ID: TID-20260412-site-release-0-1-2

## Summary
- Title: 更新 site 下载链接到 0.1.2 release
- Date: 2026-04-12
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 更新官网下载按钮到 `Pauza_0.1.2_aarch64.dmg`，并防止 GitHub latest release API 仍停留旧版本时把按钮回退到 `0.1.1`。
- In-scope: 更新 pinned 下载 URL、修正运行时 release 解析逻辑、补齐 workflow 文档并完成 release commit。
- Out-of-scope: 发布 GitHub `v0.1.2` release、本地托管 DMG、修改页面视觉或其它交互。
- Assumptions: `0.1.2` 资产命名保持 `Pauza_0.1.2_aarch64.dmg`；站点仍以 GitHub Releases 为最终下载源。
- Risks: 若 pinned URL 提前于 GitHub release 发布上线，点击可能暂时 404；若 API 结构变化，页面会退回 pinned URL。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户打开首页后看到下载按钮，按钮最终指向 `Pauza_0.1.2_aarch64.dmg`；点击后跳转到同一资产。
- Fallback / secondary flow: 若 API 不可用或返回 `Pauza_0.1.1_aarch64.dmg`，按钮仍保留 pinned `0.1.2` 链接。
- User-visible boundary: 仅首页下载按钮的 href 与点击跳转目标。
- Key visible states / transitions: 初始 pinned href -> 异步校验 latest API -> 若同名则保持/切到该 URL，若旧版本或失败则保持 pinned URL。

## Goal
- 在不改变官网视觉与文案的前提下，完成 `0.1.2` release 下载切换，并让页面对“latest 尚未切换”的窗口期具备稳态行为。

## Scope
- In-scope: `index.html`、`download/targets.js`、`script.js`、`README.md`、workflow docs 与 changelog。
- Out-of-scope: 其它平台下载、版本列表页、服务端代理、发布流水线自动化。

## Source Basis (Read Before Code)
- Related code/files reviewed: `index.html`、`download/targets.js`、`script.js`、`README.md`
- Related docs/specs/logs reviewed: 当前任务 spec 模板、`scripts/validate_workflow_docs.py`、`scripts/scaffold_task.py`
- Why these are sufficient: 本次只影响单页下载按钮与 workflow 门禁，已覆盖下载配置源、运行时解析逻辑、仓库说明与校验规则。

## Acceptance Criteria (AC)
- AC1: 首页下载按钮的固定 href 与 `download/targets.js` 中的 pinned fallback 都切换为 `https://github.com/IHKYoung/Pauza/releases/download/v0.1.2/Pauza_0.1.2_aarch64.dmg`。
- AC2: 当 GitHub latest API 仍返回 `Pauza_0.1.1_aarch64.dmg` 时，页面最终 href 仍保持 `Pauza_0.1.2_aarch64.dmg`。
- AC3: workflow 文档、CHANGELOG、验证证据与 git commit 一并完成，pre-commit 门禁通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen-after-download-target-update
- Primary flow: 首页首屏下载按钮最终落到 `0.1.2` 资产，点击后导航到同一目标。
- Fallback / secondary flow: API 返回旧版本或失败时，按钮保持 pinned `0.1.2` URL，不出现空链接。
- Interaction authority / ownership boundary: 仅 `download-button` 目标解析逻辑在本次任务内可修改。
- Visible entrypoints / handoff cues: 用户可见入口仅为右上角下载按钮，无额外系统提示。
- In-scope interactions: 页面加载后的 href 解析、下载按钮点击跳转。
- Out-of-scope interactions: 打字机文案、交互粒子、键盘提示、布局与样式。
- Interaction acceptance criteria: 页面最终 href 与点击跳转都命中 `0.1.2`，且旧 latest 响应不能把按钮回退到 `0.1.1`。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: scribe,coder,tester
- Delegation Policy: after the user grants orchestration authority, `orchestrator` may autonomously `spawn_agent`
- Automatic Delegation Triggers: `moderate/complex` | long-running task | independent sidecar tasks
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: no；纯静态站点文件更新。
- touches_running_service: no
- backup_required: no
- backup_plan: 需要回退时直接 `git revert <release-commit>`，或手动恢复 `index.html`、`download/targets.js`、`script.js` 到 `v0.1.1` 行为。
- rollback_plan: 移除 pinned-version 保护逻辑并把固定链接恢复到 `0.1.1`。
- destructive_operations: none
- operator_approval_required: no
- rationale: 无数据删除、无依赖安装、无线上服务写操作，属于低风险 trivial release 更新。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 补齐 workflow 文档骨架与本次任务 spec
  - DoD: `README/arch/ui/plan/testplan`、daily plan/log、CHANGELOG 不再保留占位字段。
- [x] Task-2: 更新下载目标与运行时解析逻辑
  - DoD: 代码中所有 `0.1.1` 下载目标切换到 `0.1.2`，旧 latest API 不会把按钮回退到旧版本。
- [x] Task-3: 浏览器验证与 git 提交
  - DoD: 本地浏览器确认最终 href 命中 `0.1.2`，workflow validator 通过并完成 commit。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260412-site-release-0-1-2/evidence/
- Interaction validation note: 本次仅做下载按钮交互校验，覆盖 primary flow、fallback flow 与最终成功态。
- Required states to capture:
  - loading: 页面初始使用 pinned href，无独立 loading UI
  - empty: 按钮 href 不得为空
  - error: API 失败或返回旧版本时，按钮仍保留 pinned href
  - disabled: 不涉及，按钮始终可点击
  - success: 页面最终 href 与点击目标都指向 `0.1.2`

## Observability / Debug Plan
- Logs: 继续使用现有 `console.warn` 记录 GitHub release 解析失败。
- Error codes: 不新增。
- Trace/metrics (optional): 不新增。
- Debug flags (optional): 不新增。

## Risks & Rollback
- Risks: pinned URL 与真正已发布的 GitHub asset 不一致时会出现下载失败；若未来命名规则变化，basename 比对需同步调整。
- Rollback plan: 回退 3 个站点文件到上一个 commit，恢复原有 latest API 解析路径。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 建立任务 spec / plan / log / changelog。
  2. 更新 pinned 下载链接与脚本决策逻辑。
  3. 本地浏览器验证 `href` 与点击目标。
  4. 通过 workflow validator 后完成 git commit。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no；仅高风险动作写 yes -->
- Approved: no
