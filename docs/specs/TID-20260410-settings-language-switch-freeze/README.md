# Task-ID: TID-20260410-settings-language-switch-freeze

## Meta
- Title: 修复设置页切换语言卡死
- Date: 2026-04-10
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-10.md
- Log (daily): ../../logs/2026-04-10.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 设置页显示语言不再直接读取 `form.language`；若 snapshot 已存在，则以 `snapshot.settings.language` 作为 UI 真实生效语言。
- `get_snapshot` 轮询 effect 不再因为草稿语言变化而重建，避免在语言选择期间叠加额外宿主调用。
- 保留语言下拉中的草稿值，让用户仍能看到自己刚选中的 locale；只是 labels / `dir/lang` 延迟到保存确认后再切换。

## Governance Notes
- Requirement Brief: 修复设置页切换语言卡死，范围只含前台语言生效时机、轮询依赖和回归测试，不改 locale 内容或 Rust host。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限 desktop 设置页语言生效时机与轮询；touches_running_service=no；backup_required=no；backup_plan=前台测试/typecheck/build；rollback_plan=回退 `App.tsx`、`i18n.ts`、测试与文档；destructive_operations=none；operator_approval_required=no；rationale=纯前台行为修复
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked（当前 session 未获用户显式 delegation 授权，遵循上层工具策略不调用 `spawn_agent`）
- Escalation Summary: 无；本轮在前台边界内闭环解决
- Retention Decision: keep

## Notes
- 自动化验证：`npm test`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`
- 关联文件：`apps/desktop/src/App.tsx`、`apps/desktop/src/i18n.ts`、`test/desktopSettingsControls.js`
