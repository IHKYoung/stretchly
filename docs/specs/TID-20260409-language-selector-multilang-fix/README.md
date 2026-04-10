# Task-ID: TID-20260409-language-selector-multilang-fix

## Meta
- Title: 修复设置页多语言选择
- Date: 2026-04-09
- Level: moderate
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Links
- Plan (daily): ../../plans/2026-04-09.md
- Log (daily): ../../logs/2026-04-09.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 前端语言列表不再按 `desktopReady` 过滤，只保留排序职责。
- 前后端 `normalize_language` 都不再把 `tr/ja/fr/...` 等语言强制回退到 `en/zh-CN`。
- 设置页语言选择从 `SegmentedControl` 改为 `Select` 下拉，适配完整多语言列表。

## Governance Notes
- Requirement Brief: 用户指出设置页虽然接入了 locale registry，但实际上不能选择多语言；本任务只修复语言列表和语言归一化链路，不改 locale 文件内容本身。
- Interaction Impact: none
- Interaction Freeze: N/A
- Execution Safety Block: service_impact=仅限设置页语言切换与前后端 i18n 语言归一化；touches_running_service=no；backup_required=no；backup_plan=以 Vitest、TypeScript、Rust tests 与 build 为边界；rollback_plan=回退 `App.tsx`、前后端 `i18n` 与测试；destructive_operations=none；operator_approval_required=no；rationale=本地设置行为修复，不涉及数据迁移、权限或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；问题边界清晰，修复集中在设置页与 i18n 层。
- Retention Decision: keep

## Notes
- 新增回归测试 `test/desktopSettingsControls.js`，覆盖完整语言列表与 `normalizeLanguage()` 不再回退。
