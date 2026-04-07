# Task-ID: TID-20260406-settings-inventory-doc

## Summary
- Title: 整理当前设置范围并沉淀设置清单文档
- Date: 2026-04-06
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 把当前设置边界整理清楚，明确“已经在设置里的项”“可以直接加回设置页的项”“需要先恢复后端能力才能加的项”，并沉淀为 docs 文档。
- In-scope: 核对 `App.tsx` 当前露出的设置；核对 `PauzaSettings` 与 `toggle_autostart` 支持的隐藏项；对照旧版 `defaultSettings.js` 整理候选设置；新增 `docs/SettingsInventory.md` 并更新 `docs/CodeMap.md`；补齐当日 plans/logs/specs。
- Out-of-scope: 不修改设置页 UI；不改 Tauri / Electron 运行时代码；不新增任何设置功能。
- Assumptions: 用户当前需要的是边界清单和信息架构输入，而不是立刻实现新设置项。
- Risks: 唯一风险是把“旧版有过”和“当前真支持”混淆，因此本轮明确按三层能力边界分开记录。
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 形成一份可直接支撑下一版设置页重构的设置清单文档。

## Scope
- In-scope:
  - `docs/SettingsInventory.md`
  - `docs/CodeMap.md`
  - 当日 `docs/plans/2026-04-06.md`
  - 当日 `docs/logs/2026-04-06.md`
  - 本任务 `docs/specs/TID-20260406-settings-inventory-doc/*`
- Out-of-scope:
  - `apps/desktop/**`
  - `app/**`
  - `apps/desktop/src-tauri/**`

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/commands.rs`
  - `app/utils/defaultSettings.js`
- Related docs/specs/logs reviewed:
  - `docs/CodeMap.md`
- Why these are sufficient: 本轮目标就是整理设置边界；前端设置页、Tauri 真源、命令层和旧版默认配置已经覆盖了这个问题的全部事实来源。

## Acceptance Criteria (AC)
- AC1: 新增一份独立文档，明确列出当前设置页已有设置、Tauri 已支持但前端未露出的设置、旧版候选设置和不应放进设置页的运行时动作。
- AC2: `docs/CodeMap.md` 能索引这份新文档。
- AC3: 本任务 docs / plans / logs / specs 不保留 `TBD/INIT` 占位。
- AC4: `python3 scripts/validate_workflow_docs.py --mode manual` 通过。

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
- service_impact: none
- touches_running_service: no
- backup_required: no
- backup_plan: 以 Git 工作区差异为边界，仅新增/修改 docs 文件。
- rollback_plan: 回滚 `docs/SettingsInventory.md`、`docs/CodeMap.md` 与本任务 docs。
- destructive_operations: none
- operator_approval_required: no
- rationale: 仅做文档沉淀和信息整理，不触碰运行时逻辑。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 核对当前设置边界
  - DoD: 明确前端已露出、Tauri 已支持未露出、旧版候选三层清单。
- [x] Task-2: 落盘设置清单文档
  - DoD: `docs/SettingsInventory.md` 完整可读，并在 `docs/CodeMap.md` 中登记。
- [x] Task-3: 补齐 workflow 文档门禁
  - DoD: 当日 plans/logs/specs 完整，validator 通过。

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260406-settings-inventory-doc/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: 文档生成并通过 validator

## Observability / Debug Plan
- Logs: 依赖当日 plans/logs 与 validator 输出。
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 旧版默认配置里有些字段名与 Tauri 现状不同，需要明确标为“旧版候选”而不是“直接可加”。
- Rollback plan:
  - 回滚本任务新增/修改的 docs 文件。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 核对 `App.tsx` 当前设置页已露出的项。
  2. 核对 `state.rs` / `commands.rs` 中当前真正支持的设置真源。
  3. 对照 `defaultSettings.js` 整理旧版候选设置。
  4. 写入 `docs/SettingsInventory.md` 并更新 `docs/CodeMap.md`。
  5. 补齐本任务 workflow 文档并跑 validator。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
