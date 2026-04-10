# Task-ID: TID-20260409-janh-key-rename

## Meta
- Title: 统一开发者 about key 为 clarkeY
- Date: 2026-04-09
- Level: trivial  <!-- trivial | moderate | complex -->
- Lane: fast    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-09.md
- Log (daily): ../../logs/2026-04-09.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- `preferences.about.clarkeY` 成为当前唯一有效的开发者姓名 locale key。
- desktop locale、archived locale、legacy `app/preferences.html` 与生成后的 desktop registry 一起完成 key 重命名，避免仓库内继续双轨。
- 本轮不改显示值、链接或元数据字段，只改 key 名与对应引用。

## Governance Notes
- Requirement Brief: 用户要求把仓库里所有 `janH` key 统一替换为 `clarkeY`；本任务覆盖 desktop locale、archived locale、legacy HTML 引用、生成产物与同日相关任务文档，确保后续 grep 不再看到旧 key。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限 locale key 名、desktop registry、legacy preferences HTML 与相关 docs；touches_running_service=no；backup_required=no；backup_plan=依赖 Git diff、`python3 scripts/sync_desktop_locales.py`、`npm test`、`python3 scripts/validate_workflow_docs.py --mode manual` 与 `rg -n "janH"` 审计；rollback_plan=回退 locale JSON、`app/preferences.html`、registry 与相关 docs；destructive_operations=none；operator_approval_required=no；rationale=无线上服务、提权、外部副作用或数据迁移。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无
- Retention Decision: keep

## Notes
- 当前仓库里精确匹配 `janH` 已清零；剩余仅保留包含小写 `janh` 的 Task-ID / 目录名，不属于产品 key。
