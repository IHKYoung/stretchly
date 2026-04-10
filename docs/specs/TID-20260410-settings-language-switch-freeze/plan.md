# Task-ID: TID-20260410-settings-language-switch-freeze

## Summary
- Title: 修复设置页切换语言卡死
- Date: 2026-04-10
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复当前设置页在切换语言时会卡住/卡死的问题，同时保留自动保存与完整 locale 列表能力。
- In-scope:
  - `App.tsx` 的显示语言解析与状态轮询依赖
  - `i18n.ts` 的前台语言生效 helper
  - 对应前端回归测试
- Out-of-scope:
  - locale 文案内容
  - Rust host 的语言归一化与 tray 文案
  - 设置 schema / 持久化结构
- Assumptions:
  - 卡死主因不是 locale 文件损坏，而是设置页在语言下拉关闭和 autosave 往返期间，用草稿语言立刻触发整页重翻译与 `dir/lang` 切换，造成交互重入。
- Risks:
  - 语言切换现在会等保存回包后才真正切换可见文案，体感上会多一个很短的延迟。
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 去掉“语言草稿值一变化就整页换语言”的重入链路，让设置页只在持久化成功后切换可见语言。

## Scope
- In-scope:
  - `/Users/changkunyang/CKProjects/Pauza/apps/desktop/src/App.tsx`
  - `/Users/changkunyang/CKProjects/Pauza/apps/desktop/src/i18n.ts`
  - `/Users/changkunyang/CKProjects/Pauza/test/desktopSettingsControls.js`
  - `/Users/changkunyang/CKProjects/Pauza/docs/specs/TID-20260410-settings-language-switch-freeze/*`
  - `/Users/changkunyang/CKProjects/Pauza/docs/{plans,logs,CHANGELOG,UI,Architecture}.md`
- Out-of-scope:
  - `/Users/changkunyang/CKProjects/Pauza/apps/desktop/src/locales/**`
  - `/Users/changkunyang/CKProjects/Pauza/apps/desktop/src-tauri/src/**`

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src-tauri/src/{commands,state,shell,i18n}.rs`
  - `test/desktopSettingsControls.js`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/specs/TID-20260409-language-selector-multilang-fix/plan.md`
- Why these are sufficient:
  - 已覆盖设置页 autosave 入口、宿主设置更新链路、现有语言选择回归测试，以及最近一轮多语言选择重构背景。

## Acceptance Criteria (AC)
- AC1: 设置页选择语言时，不再在保存完成前立刻触发整页可见语言和 `dir/lang` 切换。
- AC2: 语言切换不再重启设置页的 snapshot 轮询 effect，减少切换瞬间的宿主重入。
- AC3: `npm test`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build` 通过。

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
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,ui_designer,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 desktop 设置页前台语言生效时机与轮询行为
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 `npm test`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`
- rollback_plan: 回退 `App.tsx`、`i18n.ts`、测试与文档改动
- destructive_operations: none
- operator_approval_required: no
- rationale: 纯前台行为修复，无外部副作用、无数据迁移、无提权

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 审查语言选择、autosave 和 snapshot polling 的耦合点
  - DoD: 明确卡死入口来自“草稿语言立即驱动整页切换”而非 locale 真源缺失。
- [x] Task-2: 将设置页显示语言改为以持久化 snapshot 为准
  - DoD: 语言选择仍可自动保存，但 labels / `dir/lang` 只在保存完成后切换。
- [x] Task-3: 补充针对性回归测试与结案文档
  - DoD: 有 helper 级自动化覆盖，并在 changelog / 架构 / UI 文档中落盘行为边界。

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260410-settings-language-switch-freeze/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: 继续沿用前台已有的 load/save error 呈现，不新增额外埋点
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 若宿主保存失败，语言切换会继续停留在已持久化语言，直到下一次成功保存
- Rollback plan:
  - 回退 `resolveUiLanguage()` 和 `App.tsx` 中的显示语言解析逻辑，恢复为草稿语言立即驱动 UI

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 复核语言选择的前台状态链路与宿主保存链路。
  2. 把设置页可见语言、文档方向和加载轮询从草稿语言变化中解耦。
  3. 补测试并完成文档/日志/变更记录同步。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（orchestrator 已按当前 session 授权路由批准执行）
