# Task-ID: TID-20260406-settings-inventory-doc

## Meta
- Title: 整理当前设置范围并沉淀设置清单文档
- Date: 2026-04-06
- Level: trivial  <!-- trivial | moderate | complex -->
- Lane: fast    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-06.md
- Log (daily): ../../logs/2026-04-06.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 将设置清单拆为四层：当前前端已露出、Tauri 已支持但未露出、旧版有但当前未实现、不该放进设置页的运行时动作。
- 新增独立文档 `docs/SettingsInventory.md`，避免后续重做设置页时反复回源码核对。

## Governance Notes
- Requirement Brief: 用户要求先把“现在设置里有哪些、还能加哪些”整理清楚，并单独落一份 docs 文档；本轮只做代码核对和文档沉淀，不改产品行为。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=none；touches_running_service=no；backup_required=no；backup_plan=以 Git 工作区差异为边界；rollback_plan=回滚 `docs/SettingsInventory.md`、`docs/CodeMap.md` 与本任务 docs；destructive_operations=none；operator_approval_required=no；rationale=仅整理文档与设置边界，不触碰运行时逻辑。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: trivial-scribe-only
- Escalation Summary: 无。
- Retention Decision: keep

## Notes
- 文档主文件：`docs/SettingsInventory.md`
- 同步更新：`docs/CodeMap.md`
