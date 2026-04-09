# Task-ID: TID-20260408-reminder-mode-simplification

## Meta
- Title: 收敛提醒模式并实现智能强制自然休息模型
- Date: 2026-04-08
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-08.md
- Log (daily): ../../logs/2026-04-08.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 主模型收敛为 `智能提醒 / 强制提醒 / 自然休息` 三个用户概念。
- `强制提醒` 直接吸收原先 `严格模式` 语义，不再并列暴露。
- 第一版去掉 `soft nudge`，智能提醒只保留“到点后等空档”主路径。
- `breakPromptStyle` 从用户设置中移除，不再参与提醒策略讨论。

## Governance Notes
- Requirement Brief: 用户已确认把当前实现进一步做减法，真正落地成“智能提醒 / 强制提醒 / 自然休息”模型；本轮要同步 host 状态机、输入空闲探测、设置页、文案、测试与长期文档。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: frozen for implementation；设置页只保留 `智能提醒 / 强制提醒 / 自然休息`，强制提醒吸收原 strict 语义，break style 从用户设置移除。
- Execution Safety Block: service_impact=仅限本地提醒投递时机、break 可绕过性与设置页入口；touches_running_service=no；backup_required=no；backup_plan=以 Git diff、cargo test/check、前端构建与 workflow docs validator 为边界；rollback_plan=回退 Rust host、设置页和文档改动；destructive_operations=替换当前 adaptive/strict 设置模型；operator_approval_required=no；rationale=不涉及数据、提权、外部服务或新增依赖。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked（仅 fallback 时填写）
- Escalation Summary: N/A
- Retention Decision: keep

## Notes
- Primary runtime targets: `apps/desktop/src-tauri/src/{state,platform,shell}.rs`
- Primary UI target: `apps/desktop/src/App.tsx`
