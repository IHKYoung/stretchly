# Task-ID: TID-20260409-break-cta-tightening

## Meta
- Title: 收紧 break 完成与延后按钮时机
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
- break 倒计时进行中不再允许提前完成。
- `Later` 从“前 30% 时间窗口”收紧为“倒计时开始后的前 10 秒”。
- 后端 command 也要同步收紧，不能只改前端按钮显示。

## Governance Notes
- Requirement Brief: 用户要求 break 不再允许提前完成，同时把 `Later` 收紧为只在倒计时开始后的前 10 秒可用；本任务同时收紧前端 CTA 显示和 host 侧 command/状态判断。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限 break CTA 时机规则、对应测试与相关文档；touches_running_service=no；backup_required=no；backup_plan=依赖 cargo test、npm test、desktop build 与 docs validator；rollback_plan=回退 `state.rs`、`App.tsx`、测试与文档；destructive_operations=none；operator_approval_required=no；rationale=不涉及线上服务、权限、数据迁移或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无
- Retention Decision: keep

## Notes
- 当前规则不改 skip，只收紧 finish 与 postpone 的时机。
