# Task-ID: TID-20260409-break-ideas-source-random-fix

## Summary
- Title: 接回 break ideas 真源并删除过渡 prompts
- Date: 2026-04-09
- Level: moderate
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 让 break 页中央交互语重新从 `messages/*.json` 顶层的 `miniBreakIdeas` / `longBreakIdeas` 读取，并删除 `ui.breakCopy.prompts` 这组过渡字段；同时修复固定 10m/30m 节奏总是落在同一条 prompt 的索引缺陷。
- In-scope:
  - `App.tsx` 的 prompt 读取链路
  - `i18n.ts` 的 ideas 列表读取 helper
  - 新的 `break-ideas.ts` 选择逻辑
  - `messages/{en,zh-CN}.json` 里的过渡字段删除
  - 回归测试与当前文档口径
- Out-of-scope:
  - break 页视觉改版
  - CTA、声音、背景、调度状态机变更
  - 全量多语言文案重写
- Assumptions:
  - 当前各语言都保留 `miniBreakIdeas` / `longBreakIdeas` 结构
  - `ui.breakCopy.defaultPrompt.*` 仍保留为 `breakIdeasEnabled = false` 的默认文案
- Risks:
  - 删除过渡字段后若 ideas 取值路径接错，会导致 break 页退回固定默认文案
  - prompt 选择若直接 `Math.random()` 会在每秒重渲染时抖动
- Interaction impact: none
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- break prompt 真源回归 locale ideas，删除过渡 prompt 数组，并让固定节奏下的 prompt 选择不再锁死。

## Scope
- In-scope:
  - `apps/desktop/src/{App.tsx,i18n.ts,lib/break-ideas.ts}`
  - `apps/desktop/src/locales/messages/{en,zh-CN}.json`
  - `test/{desktopBreakIdeas.js,desktopBreakCopySource.js}`
  - `docs/{UI,CodeMap,CHANGELOG}.md` 与本任务 specs/logs/plans
- Out-of-scope:
  - break 页视觉结构与按钮语义
  - Rust host / Tauri 侧逻辑

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/{App.tsx,i18n.ts,lib/break-prompt.ts}`
  - `apps/desktop/src/locales/messages/{en,zh-CN}.json`
  - `app/main.js`
  - `app/utils/{defaultMicrobreakIdeas.js,defaultBreakIdeas.js,ideasLoader.js,shuffled.js}`
- Related docs/specs/logs reviewed:
  - `docs/UI.md`
  - `docs/specs/TID-20260409-apps-locale-single-source-refactor/*`
  - 当日 logs/plans 中 tray/settings/break 相邻任务
- Why these are sufficient:
  - 已覆盖当前桌面端 prompt 渲染入口、locale 真源、原版 app ideas 加载方式，以及当前 docs 对 prompt 来源的口径。

## Acceptance Criteria (AC)
- AC1: break 页开启 `breakIdeasEnabled` 时，交互语直接来自 `miniBreakIdeas` / `longBreakIdeas` 的 `.text` 列表。
- AC2: `ui.breakCopy.prompts` 从桌面端 locale 真源中移除，代码中也不再保留对它的 fallback。
- AC3: 对固定 10m/30m break 周期，prompt 选择不再因简单取模而一直重复同一条。
- AC4: `breakIdeasEnabled = false` 时仍回退到 `ui.breakCopy.defaultPrompt.*`。

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
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 break prompt 文案来源、前端 helper、locale JSON、测试与相关 docs
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 locale sync、typecheck、test、build 与 docs validator
- rollback_plan: 回退 `i18n.ts`、`break-ideas.ts`、`App.tsx`、locale JSON、测试与 docs
- destructive_operations: 删除 `ui.breakCopy.prompts` 过渡字段
- operator_approval_required: no
- rationale: 前端文案来源收口，无外部系统、权限或数据迁移风险

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 新增 ideas 读取与稳定选择 helper
  - DoD: `App.tsx` 不再走 `ui.breakCopy.prompts.*`，同一 break 内 prompt 稳定
- [x] Task-2: 删除过渡 locale 字段并补回归测试
  - DoD: `messages/{en,zh-CN}.json` 不再存在 `ui.breakCopy.prompts`，测试覆盖真源与固定周期 bug

## Evidence Plan (UI / E2E)
- Evidence required: no
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-break-ideas-source-random-fix/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs: 无新增 runtime log；排查优先看 `tBreakIdeaList()` 结果与 `pickBreakPromptIndex()` 对固定周期的输出。
- Error codes: 无
- Trace/metrics (optional): 无
- Debug flags (optional): 无

## Risks & Rollback
- Risks:
  - 误删过渡字段后残留旧代码读取
  - prompt 选择函数不稳定，导致每秒重渲染切词
- Rollback plan:
  - 恢复 `messages/{en,zh-CN}.json` 的 `ui.breakCopy.prompts`
  - 回退 `break-ideas.ts` / `i18n.ts` / `App.tsx`
  - 重跑 locale sync、test、typecheck、build

## Sequential Phases
- phase_execution: N/A
- phase_confirmation_policy: N/A
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审计当前 prompt 来源与原版 app ideas 加载方式。
  2. 把读取链路改为 `miniBreakIdeas` / `longBreakIdeas`。
  3. 删除 `ui.breakCopy.prompts` 与所有 fallback。
  4. 补测试并更新文档口径。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes
- Approved: 用户在当前线程中明确要求“`ui.breakCopy.prompts` 这个过渡的应该直接删掉”
