# Task-ID: TID-20260409-break-message-copy-centralization

## Summary
- Title: 统一管理消息页提示语
- Date: 2026-04-09
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 将 break 消息页提示语改成单独文件集中管理，同时切断桌面端 locale 构建对 legacy `app/locales` / `app/preferences.html` 的依赖，避免新旧设计继续混用。
- In-scope:
  - `apps/desktop/src/locales/break-message-copy.ts`
  - `apps/desktop/src/{App.tsx,lib/break-prompt.ts}`
  - `scripts/sync_desktop_locales.py`
  - `docs/{RepositoryGuidelines,CodeMap,Architecture,UI,CHANGELOG}.md`
- Out-of-scope:
  - 删除 legacy Electron 的 locale 资源
  - 为所有语言补齐新的 break 页面专属文案
  - 调整 break 页面视觉布局或 CTA 行为
- Assumptions:
  - 通用 locale registry 结构保持兼容，前后端消费者无需变更
  - break 页面专属文案目前仅需覆盖桌面端已准备好的 `zh-CN` / `en`
- Risks:
  - 如果存在残留 `ui.break.*` 读取，页面可能出现文案缺失
  - 如果 locale config / message / override 目录不一致，registry 构建会被新校验阻断
- Interaction impact: none
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让桌面端 break 消息页文案和桌面端 locale 真源都收口到新设计自己的目录下，不再继续回头依赖 legacy `app/`。

## Scope
- In-scope:
  - `apps/desktop/src/locales/break-message-copy.ts`
  - `apps/desktop/src/{App.tsx,lib/break-prompt.ts}`
  - `apps/desktop/src/locales/{messages,config,overrides,registry.generated.json}`
  - `scripts/sync_desktop_locales.py`
  - 本任务 specs 与相关 docs 更新
- Out-of-scope:
  - Rust host 逻辑
  - legacy Electron 运行时 locale 迁移或清理

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/lib/break-prompt.ts`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src/locales/{overrides/en.json,overrides/zh-CN.json,config/en.json,config/zh-CN.json,registry.generated.json}`
  - `apps/desktop/src-tauri/src/i18n.rs`
  - `scripts/sync_desktop_locales.py`
- Related docs/specs/logs reviewed:
  - `docs/{RepositoryGuidelines,CodeMap,Architecture,UI,CHANGELOG}.md`
  - `docs/{plans,logs}/2026-04-09.md`
  - `docs/specs/TID-20260409-break-surface-i18n-reduction/*`
- Why these are sufficient:
  - 已覆盖 break 消息页当前文案读取点、桌面端 locale registry 生成链路、Rust/前端消费者契约，以及最近一轮 i18n 重构上下文，足以支撑本次去 legacy 混用与提示语收口改动。

## Acceptance Criteria (AC)
- AC1: break 页面主提示语、默认提示语、等待完成文案与 CTA 标签统一收口到单一文件维护。
- AC2: `scripts/sync_desktop_locales.py` 不再读取 `app/locales` 或 `app/preferences.html`，桌面端 locale registry 只依赖 `apps/desktop/src/locales/**`。
- AC3: `python3 scripts/sync_desktop_locales.py`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build` 通过。
- AC4: 与本次改动直接相关的文档明确指出桌面端 locale 真源与 break 消息页文案入口。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: N/A
- Primary flow: N/A
- Fallback / secondary flow: N/A
- Interaction authority / ownership boundary: N/A
- Visible entrypoints / handoff cues: N/A
- In-scope interactions: N/A
- Out-of-scope interactions: N/A
- Interaction acceptance criteria: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节与 Requirement Brief 中的交互字段不得继续保留未填写占位。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅影响桌面端 locale 构建链路、前端 break 页面文案读取与相关文档。
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 Git diff、`python3 scripts/sync_desktop_locales.py`、`npm --prefix apps/desktop run typecheck` 与 `npm --prefix apps/desktop run build` 作为回退边界。
- rollback_plan: 回退 `apps/desktop/src/locales/**`、`App.tsx`、`break-prompt.ts`、`scripts/sync_desktop_locales.py` 与相关 docs。
- destructive_operations: none
- operator_approval_required: no
- rationale: 无线上服务、外部副作用、提权或新增依赖。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 切断桌面端 locale 对 legacy `app/` 的构建依赖
  - DoD: `scripts/sync_desktop_locales.py` 只依赖 `apps/desktop/src/locales/{messages,config,overrides}` 即可生成 registry。
- [x] Task-2: 收口 break 消息页提示语
  - DoD: break 页面文案只需编辑 `apps/desktop/src/locales/break-message-copy.ts` 即可生效，组件内不再散落 `ui.break.*` 文案 key。

## Evidence Plan (UI / E2E)
- Evidence required: no
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-break-message-copy-centralization/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs:
  - `scripts/sync_desktop_locales.py` 现在会在 config / message / override 不匹配时直接报出 language code。
- Error codes: 不新增。
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - break 页面若仍有残余旧 key 读取，会在运行时露出缺失文案
  - locale 目录存在孤儿文件时，新的校验会让 registry 构建失败
- Rollback plan:
  - 回退新文件 `break-message-copy.ts` 与相关消费改动
  - 回退 `sync_desktop_locales.py` 到旧实现
  - 重新生成 registry 并跑前端 build 验证恢复

## Sequential Phases
- phase_execution: N/A
- phase_confirmation_policy: N/A
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 新增 break 消息页专属文案文件，并改造 `App.tsx` / `break-prompt.ts` 的读取方式。
  2. 改造 locale registry 生成脚本，移除 legacy `app/` 输入。
  3. 重新生成 registry，跑前端类型检查与构建。
  4. 更新 docs 与任务闭环记录。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留未填写占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
