# Task-ID: TID-20260424-break-ideas-asset-migration

## Summary
- Title: 拆分 break ideas 资产真源
- Date: 2026-04-24
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: INIT

## Requirement Brief
- Goal restatement: 把 break ideas 从 `messages/*.json` 中独立出来，迁移为单独的 JSON 资产目录与 registry，同时保留原始 `miniBreakIdeas / longBreakIdeas` 结构。
- In-scope:
  - 迁移全部现有语言 break ideas 到独立目录
  - 新增 break ideas sync/registry
  - 调整前台读取链路、测试与当前 docs
- Out-of-scope:
  - 修改 break idea 文案内容
  - 删除 legacy 语言 ideas bundle
  - 改动提醒策略、Rust host i18n 或设置 UI
- Assumptions:
  - 现有 50 语言 ideas 内容都应先完整保留
  - future cleanup 应通过删 bundle + fallback，而不是再塞回 messages
- Risks:
  - 双真源复燃
  - 运行时仍读旧路径
  - 后续删 bundle 时 fallback 漂移
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 形成“UI 文案 / break ideas 内容资产 / registry 元数据 / helper 读取逻辑”四层清晰边界

## Scope
- In-scope:
  - `apps/desktop/src/locales/break-ideas/**`
  - `apps/desktop/src/locales/messages/*.json`
  - `apps/desktop/src/lib/break-ideas.ts`
  - `apps/desktop/src/i18n.ts`
  - `scripts/{sync_desktop_locales.py,sync_desktop_break_ideas.py}`
  - `test/{desktopBreakIdeas,desktopBreakCopySource,translations}.js`
  - 当前 docs / changelog / task docs
- Out-of-scope:
  - 具体 break idea 文案重写
  - official/legacy 语言策略的产品决策收缩
  - Rust host locale 行为调整

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/{i18n.ts,App.tsx,lib/break-ideas.ts}`
  - `apps/desktop/src/locales/{messages,config}/`
  - `scripts/sync_desktop_locales.py`
  - `test/{desktopBreakIdeas,desktopBreakCopySource,translations}.js`
- Related docs/specs/logs reviewed:
  - `docs/{RepositoryGuidelines,CodeMap,UI,Architecture,CHANGELOG}.md`
  - `docs/specs/TID-20260409-break-ideas-source-random-fix/{README,plan,arch,testplan}.md`
  - `docs/logs/2026-04-24.md`
- Why these are sufficient:
  - 覆盖了当前 break prompt 读取路径、locale 生成链路、测试边界和历史“唯一真源”约束来源

## Acceptance Criteria (AC)
- AC1: `messages/*.json` 不再包含 `miniBreakIdeas / longBreakIdeas` 顶层 key，break ideas 内容唯一真源迁移到 `locales/break-ideas/messages/*.json`
- AC2: `lib/break-ideas.ts` 只从 `break-ideas/registry.generated.json` 读取 ideas，并保留当前语言优先、locale fallback 补位的运行时行为
- AC3: `break-ideas/registry.json` 以数据标签标记 `official / legacy` 语言；当前 `zh-CN / zh-TW / en` 为 official，其它保留为 legacy
- AC4: `sync_desktop_break_ideas.py`、`sync_desktop_locales.py`、`typecheck`、`npm test`、`cargo test` 与 workflow docs validator 全部通过

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
- service_impact: 仅限 desktop locale 资产、前台 helper、sync scripts、测试与 docs
- touches_running_service: no
- backup_required: no
- backup_plan: `python3 scripts/sync_desktop_break_ideas.py`、`python3 scripts/sync_desktop_locales.py`、`npm run typecheck`、`npm test`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`python3 scripts/validate_workflow_docs.py --mode manual`
- rollback_plan: 回退 break-ideas 目录、`messages/*.json` 的 ideas 拆分、`lib/break-ideas.ts`、`i18n.ts`、sync scripts 与 docs
- destructive_operations: 从 `messages/*.json` 删除 `miniBreakIdeas / longBreakIdeas`
- operator_approval_required: no
- rationale: 纯本地资产迁移，无线上服务或外部副作用；用户已在当前对话显式批准“先做迁移”

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 建立独立 break ideas 资产目录与 registry
  - DoD: `break-ideas/messages/*.json`、`registry.json`、`registry.generated.json` 与 `sync_desktop_break_ideas.py` 落地
- [x] Task-2: 迁移现有 ideas 内容并移除旧 message bundle 重复 key
  - DoD: 50 语言 ideas 从 `messages/*.json` 抽离完成，`sync_desktop_locales.py` 会阻止旧 key 回流
- [x] Task-3: 调整读取链路、测试与当前 docs
  - DoD: runtime/test/docs 全部指向新资产路径并完成验证

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260424-break-ideas-asset-migration/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs:
  - `sync_desktop_break_ideas.py` 输出语言数 / bundle 数 / registry 路径
  - `sync_desktop_locales.py` 在 message bundle 残留 ideas key 时直接失败
- Error codes:
  - N/A
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - N/A

## Risks & Rollback
- Risks:
  - 运行时仍读旧 `messages` 路径
  - source registry 与 generated registry 漂移
  - 未来删 legacy bundle 时 fallback 行为未被注意
- Rollback plan:
  - 回退本任务对 ideas 目录、message bundle、helper、sync script、tests 与 docs 的改动
  - 重新运行 `python3 scripts/sync_desktop_locales.py`

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 增加 break ideas 独立目录、source registry 与生成脚本
  2. 批量抽离 50 语言 ideas 到新目录，并从 messages 删掉对应 key
  3. 调整 runtime/helper/test/docs，重新生成 registry 并完成回归

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户已在当前对话明确批准“先做迁移”
