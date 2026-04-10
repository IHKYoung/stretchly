# Task-ID: TID-20260410-pre-break-heads-up-restoration

## Meta
- Title: 恢复并重构提前提示为可见 heads-up 状态
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
- `microbreak_notification_*` / `long_break_notification_*`、`next_notification_due_ms` 与 `engine.rs` 的 `notification().show()` 接线仍然存在，提前提示并非彻底失效，而是只作为一次性系统通知存在，产品体感上近似“没用”。
- heads-up 阶段现在改为从 `next_break_due_ms + notification lead time` 派生，不再依赖一次性通知发送后立即清空的内部时间戳，因此设置页运行时状态、tray 菜单与 tooltip 能在 break 到点前持续可见。
- 系统通知保留为辅助提示，但失败不再静默；`engine.rs` 现在会输出 `failed to show desktop notification`，便于后续定位系统层抑制或平台兼容问题。
- 设置文案从“提前提醒 / Heads-up notifications”收敛为“提前提示 / Heads-up cues”，避免继续把功能语义错误绑定为“必须出现系统通知”。

## Governance Notes
- Requirement Brief: 用户质疑“提前提醒”目前看起来完全没用；本任务先核对设置、调度与投递链路，再将其恢复为真正可见的 heads-up 状态。范围仅限 `state.rs` / `engine.rs` / locale 与对应文档，不引入新提醒通道、不改 break 核心节奏和 CTA。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: frozen；仅重构 break 前的 heads-up 可见状态和设置文案，不新增新的 break 页面弹窗、不改变 due 后 smart/forced 的核心 break 进入策略。
- Execution Safety Block: service_impact=仅限 desktop runtime 的 heads-up 状态呈现、通知错误日志与文案；touches_running_service=no；backup_required=no；backup_plan=以 `cargo test`、desktop build、locale registry 重建和 docs 变更为回归边界；rollback_plan=回退 `apps/desktop/src-tauri/src/state.rs`、`apps/desktop/src-tauri/src/engine.rs`、locale 文案及本任务 docs；destructive_operations=none；operator_approval_required=no；rationale=不涉及运行中服务、数据迁移、权限提升或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: BLOCKED: 当前 session 未获用户显式 delegation 授权，遵循上层工具策略不调用 `spawn_agent`；由单 agent 在限定范围内完成链路核查、实现、验证与文档收口。
- Retention Decision: keep

## Notes
- 证据见 `docs/specs/TID-20260410-pre-break-heads-up-restoration/evidence/README.md`；本轮主要证据是状态机单测、desktop build 和文案/设计文档同步，tray 的系统级真实观感仍建议由用户本机实际观察一次。
