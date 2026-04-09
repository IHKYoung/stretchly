# Task-ID: TID-20260408-reminder-state-machine-doc

## Summary
- Title: 整理提醒状态机与调度逻辑文档
- Date: 2026-04-08
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 在真正继续改 host 逻辑之前，先把下一轮久坐提醒的目标状态机和整体调度逻辑落成正式文档，作为后续实现真源。
- In-scope:
  - 在 `docs/` 根目录新增一份提醒调度设计文档。
  - 用文字和图示说明 `智能提醒 / 强制提醒 / 自然休息` 的职责边界。
  - 明确 `强制提醒` 已吸收原先 `严格模式` 语义，不再并列暴露。
  - 明确目标状态、状态迁移、tick 优先级、推荐阈值和非目标。
  - 同步本任务 spec、daily plans/logs 与 `docs/CodeMap.md`。
- Out-of-scope:
  - 修改任何 Rust/前端运行时代码。
  - 调整现有设置页、文案资源或 break window 表现。
  - 直接把新模型落地实现。
- Assumptions:
  - 当前用户目标是先确认设计，再开始实现。
  - 现有 Tauri host 与上一版 adaptive reminder 文档足以作为 source basis。
  - 本轮只需要一份面向产品和实现的中文设计文档，不需要额外 UI 证据。
- Risks:
  - 文档若不明确标注“目标模型”，容易与当前已落地的实验性 adaptive 版本混淆。
  - 若不明确“强制提醒 = 严格定时打断”，后续实现仍可能重复保留两个同义设置。
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让下一轮提醒逻辑在继续编码前先有一份清晰、可执行、可审阅的调度真源。

## Scope
- In-scope:
  - 新增 `docs/ReminderScheduling.md`。
  - 用 Mermaid 图描述目标状态机与 tick 调度流程。
  - 写清楚核心原则、状态定义、优先级、阈值建议、运行时字段建议和典型场景。
  - 同步 workflow 文档与索引。
- Out-of-scope:
  - 对当前 adaptive reminder 实现做删改。
  - 修改 `docs/Architecture.md`、`docs/UI.md`、`docs/SettingsInventory.md` 的既有行为说明。
  - 增加 CHANGELOG 条目。

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/platform.rs`
  - `apps/desktop/src-tauri/src/state.rs`（通过关键字段与状态机相关 grep）
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/Architecture.md`
  - `docs/specs/TID-20260408-adaptive-reminder-state-machine/plan.md`
  - `docs/plans/2026-04-08.md`
  - `docs/logs/2026-04-08.md`
- Why these are sufficient:
  - 已覆盖当前 Tauri host 的输入空闲信号来源、上一版 adaptive reminder 的设计边界，以及当前 docs 目录的长期索引约束；本轮不涉及运行时代码落地，因此这些依据足以支撑文档设计。

## Acceptance Criteria (AC)
- AC1: `docs/ReminderScheduling.md` 存在，并清楚定义智能提醒、强制提醒、自然休息各自的职责边界，以及“强制提醒吸收严格模式”的结论。
- AC2: 文档包含至少一份状态图和一份 tick/调度流程图，能直接说明状态迁移和优先级。
- AC3: 文档明确写出“先做减法”的约束，包括 `breakPromptStyle` 不进入调度决策、第一版不包含 `soft nudge`。
- AC4: 本任务 spec、daily plans/logs 与 `docs/CodeMap.md` 完整同步，`python3 scripts/validate_workflow_docs.py --mode manual` 通过。

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
- Required Roles: scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅新增/更新文档，不改变运行时代码或用户实际行为。
- touches_running_service: no
- backup_required: no
- backup_plan: 以 Git diff 与 workflow docs validator 为回滚边界。
- rollback_plan: 回退 `docs/ReminderScheduling.md`、`docs/CodeMap.md` 和本任务相关 workflow 文档。
- destructive_operations: none
- operator_approval_required: no
- rationale: 纯文档任务，不涉及服务、数据、权限、依赖或外部副作用。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 整理目标提醒模型的核心概念与状态机
  - DoD: 文档明确写出状态列表、状态迁移、tick 优先级与伪代码。
- [x] Task-2: 把文档落盘到 `docs/` 并补齐图示
  - DoD: `docs/ReminderScheduling.md` 落盘，且包含 Mermaid 状态图与流程图。
- [x] Task-3: 同步 spec / plans / logs / CodeMap
  - DoD: workflow 文档不保留 `TBD/INIT` 占位进入 DONE，CodeMap 可索引到新文档。

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260408-reminder-state-machine-doc/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: 文档与 workflow 校验通过即可

## Observability / Debug Plan
- Logs: 通过 daily plans/logs 记录本轮文档产物、source basis 与验证结果。
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 若文档没有明确声明这是“目标模型”，后续容易与当前已实现的 adaptive 状态机混淆。
  - 若把自然休息误写成第三种提醒方式，或继续把严格模式和强制提醒并列暴露，会继续污染设置与调度职责边界。
- Rollback plan:
  - 删除/回退 `docs/ReminderScheduling.md` 与相关同步文档即可，不影响代码。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 读取当前 host 的 `idle_ms` 探测方式、上一版 adaptive reminder 方案和 docs 索引约束。
  2. 写出做减法后的目标调度模型，明确状态机、调度优先级、自然休息与严格模式的边界。
  3. 把设计文档落盘到 `docs/ReminderScheduling.md`。
  4. 同步 task spec、daily logs/plans 与 `docs/CodeMap.md`。
  5. 跑 workflow docs validator 并结案。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
