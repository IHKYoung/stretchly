# Task-ID: TID-20260409-smart-reminder-wait-guard

## Summary
- Title: 缩短智能提醒阈值并阻止无限等待
- Date: 2026-04-09
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把 smart reminder 从“默认阈值偏长且可能无限等待”的状态收紧为“默认更短、用户可调、等待有上限”的可控行为。
- In-scope:
  - `idle_opportunity_seconds` 默认值调整
  - smart wait cap 的 host 侧判定
  - 设置页智能暂停分组暴露 idle threshold
  - zh-CN / en 文案与回归测试
- Out-of-scope:
  - 新 reminder mode
  - per-break 独立 idle threshold 设置
  - tray / break window 的额外 UI 重构
- Assumptions:
  - 当前用户投诉的阈值主要来自 hidden default，而不是手动配置
  - 1s tick 的状态机粒度足够支撑 bounded wait
- Risks:
  - 状态快照与 tick 节拍对 waiting 态显示不一致
  - 设置页新增控件后 locale 漏配导致显示 raw key
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让 smart reminder 更像“稍等一下再打断”，而不是“可能永远不打断”。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/locales/messages/{en,zh-CN}.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `test/desktopSettingsControls.js`
  - `docs/{UI,Architecture,SettingsInventory,CHANGELOG}.md` 与本任务 specs/logs/plans
- Out-of-scope:
  - 不调整 reminder mode 枚举
  - 不新增新的持久化字段
  - 不补 UI evidence 采集

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/{state.rs,engine.rs,platform.rs}`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src/locales/messages/{en,zh-CN}.json`
  - `test/desktopSettingsControls.js`
- Related docs/specs/logs reviewed:
  - `docs/{UI,Architecture,SettingsInventory}.md`
  - `docs/logs/2026-04-08.md` 与当日 plans/logs
- Why these are sufficient:
  - 已覆盖智能提醒状态机、设置页现状、locale fallback、既有 hidden setting 口径与历史提醒简化记录。

## Acceptance Criteria (AC)
- AC1: `idle_opportunity_seconds` 默认值收紧到 `6s`，并且设置页“智能暂停”分组直接暴露该秒数控件。
- AC2: smart mode 在 due 后会先等待短空档，但在持续输入下不会无限等待；达到 bounded wait cap 后自动开始。
- AC3: zh-CN / en 的相关文案不再表达“等更合适的时候”，而是明确为“短空档、不会无限等待”。

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
- Required Roles: orchestrator,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 smart reminder host 状态机、设置页智能暂停控件、locale 与相关文档
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 locale sync、npm test、cargo test、typecheck、build 与 docs validator
- rollback_plan: 回退 `state.rs`、`App.tsx`、locale、测试与 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 不涉及线上服务、权限、数据迁移或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收紧 smart reminder host 策略
  - DoD: 默认阈值更短，且 due break 在持续操作下也会在 bounded wait cap 后自动开始
- [x] Task-2: 暴露设置入口并同步文案/测试
  - DoD: 设置页出现阈值控件，locale registry 重建，前后端测试通过

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-smart-reminder-wait-guard/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs: 无新增结构化日志；排查优先观察 `next_break_wait_started_ms`、`idle_ms`、`idle_opportunity_seconds` 与 `smart_wait_cap_ms(kind)`。
- Error codes: 无
- Trace/metrics (optional): 无
- Debug flags (optional): 无

## Risks & Rollback
- Risks:
  - waiting status 在 snapshot/tick 边界上显示不一致
  - 新设置项缺少 locale key 或 registry 未同步
- Rollback plan:
  - 回退 `state.rs`、`App.tsx`、locale、测试与 docs
  - 重新生成 registry，并重跑全量验证

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 核对现有 smart reminder 的 due / waiting 判定。
  2. 在 `state.rs` 增加 bounded wait cap，并收紧默认阈值。
  3. 在设置页暴露 `idleOpportunitySeconds`。
  4. 同步 locale、测试与文档。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
