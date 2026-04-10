# Task-ID: TID-20260409-break-window-layout-and-controls-refresh

## Meta
- Title: 重做 break 界面布局并移除跳过按钮
- Date: 2026-04-09
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
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
- break prompt 已从展示型双栏布局收口为居中的单列内容区，不再保留独立 cue card / 装饰性双区块。
- 主视觉改为单条提示语、数字倒计时和细条形进度，保持休息界面更安静、更像桌面工具而不是演示页。
- 主界面不再直接暴露 `Skip`；当前只保留与运行时状态一致的动作入口，如 `Later` 和 `Resume work`。

## Governance Notes
- Requirement Brief: 用户要求重做 break 界面布局并移除主界面的 `Skip` 操作；本任务只改 `App.tsx` / `break-prompt.ts` 的 break 页面结构、视觉层次和按钮出口，不改状态机与宿主命令面。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: frozen；break 页面只保留单列主视觉、数字倒计时、细进度条以及与当前运行态一致的 CTA，不扩展到设置页、调度模型或新的 break 动作。
- Execution Safety Block: service_impact=仅限 break prompt 布局、CTA 出口、相关文档与验证；touches_running_service=no；backup_required=no；backup_plan=依赖 `npm test`、desktop build 与 workflow docs validator；rollback_plan=回退 `App.tsx`、`break-prompt.ts` 与相关 docs；destructive_operations=none；operator_approval_required=no；rationale=仅前台界面收敛，无运行中服务、权限或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: BLOCKED: 当前 session 未获用户显式 delegation 授权，遵循上层工具策略不调用 `spawn_agent`；由单 agent 在限定范围内完成 break 页面重构、验证与文档收口。
- Retention Decision: keep

## Notes
- 证据路径见 `docs/specs/TID-20260409-break-window-layout-and-controls-refresh/evidence/README.md`。当前保留的证据以代码路径、build/test 与文档闭环为主，没有额外录屏资产。
