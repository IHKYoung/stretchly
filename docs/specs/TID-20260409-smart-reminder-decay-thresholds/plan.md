# Task-ID: TID-20260409-smart-reminder-decay-thresholds

## Summary
- Title: 将智能提醒改为递减阈值曲线
- Date: 2026-04-09
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把 smart reminder 从“单一 idle 阈值”收敛成“分阶段递减阈值 + 最终 deadline”的默认策略，并把这套设计沉淀到长期文档。
- In-scope:
  - `state.rs` 的 smart reminder 判定逻辑
  - zh-CN / en 相关文案
  - `docs/ReminderScheduling.md` 与当前架构/UI 文档
- Out-of-scope:
  - 新设置项
  - 新 reminder mode
  - pause/focus/DND/natural break 规则改写
- Assumptions:
  - “少配置、强默认”优先级高于继续开放阈值设置
  - 用户认可微休息和休息使用不同的等待节奏
- Risks:
  - 阶段切换时机写错
  - 文档只写概念，没把具体曲线记录下来
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让 smart reminder 更像一个会“逐步找时机”的助手，而不是单一阈值判断器。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/locales/messages/{en,zh-CN}.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `docs/{Architecture,UI,SettingsInventory,ReminderScheduling,CHANGELOG}.md`
  - 本任务 specs/logs/plans
- Out-of-scope:
  - `App.tsx`
  - 新设置项
  - 额外 runtime signal

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/locales/messages/{en,zh-CN}.json`
- Related docs/specs/logs reviewed:
  - `docs/{Architecture,UI,SettingsInventory,ReminderScheduling,CHANGELOG}.md`
  - `docs/{plans,logs}/2026-04-09.md`
  - `docs/specs/TID-20260409-smart-reminder-wait-guard/*`
  - `docs/specs/TID-20260409-smart-reminder-ui-simplification/*`
- Why these are sufficient:
  - 已覆盖当前状态机、语言真源、当日 reminder 决策演进和需要同步的长期文档。

## Acceptance Criteria (AC)
- AC1: smart reminder 改为 per-kind 递减阈值曲线，不再用单一 idle 阈值判定。
- AC2: Rust 单测覆盖微休息阈值放宽、长休息更宽首段阈值和最终 deadline 触发。
- AC3: 文档明确记录微休息 `6/3/1/45s` 与休息 `8/4/1/90s` 的设计。

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
- service_impact: 仅限 smart reminder host 状态机、locale 文案与 reminder 设计文档
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 locale sync、cargo test、npm test、typecheck、build 与 docs validator
- rollback_plan: 回退 `state.rs`、locale、文档与测试
- destructive_operations: none
- operator_approval_required: no
- rationale: 不涉及线上服务、提权、数据迁移或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 将 host 判定逻辑改为递减阈值曲线
  - DoD: 微休息和休息各自使用阶段式空档要求，并保留最终 deadline
- [x] Task-2: 补回归测试并更新设计文档
  - DoD: Rust 单测通过，`ReminderScheduling`/`Architecture`/`UI` 写明具体曲线

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-smart-reminder-decay-thresholds/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs: 无新增结构化日志；调试优先看 `next_break_wait_started_ms`、`idle_ms` 与当前阶段要求。
- Error codes: 无
- Trace/metrics (optional): 无
- Debug flags (optional): 无

## Risks & Rollback
- Risks:
  - 某阶段阈值过低，导致过早打断
  - 某阶段切换条件写错，导致迟迟不开始
- Rollback plan:
  - 回退 `state.rs`、locale、文档与测试
  - 重新同步 registry，并重跑验证

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 把单一阈值逻辑改成 per-kind 阶段曲线。
  2. 为三条关键路径补 Rust 单测。
  3. 更新 locale 和长期设计文档。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: 用户在当前线程中明确认可“我觉得可以，改一下”
