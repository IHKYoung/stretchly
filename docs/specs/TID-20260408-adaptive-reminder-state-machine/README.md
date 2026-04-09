# Task-ID: TID-20260408-adaptive-reminder-state-machine

## Meta
- Title: 设计并实现低打断久坐提醒状态机
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
- 决定将“到点即打断”改为“到点后择机投递”，显式引入 `Scheduled -> DueButProtected -> SoftNudge -> BreakRunning` 的运行时状态机。
- 优先在 Tauri `state.rs` 落地，因为根入口默认已切到 `apps/desktop`，仅修 Electron 参考实现无法解决当前产品体验。
- 不引入全局键盘监听或新依赖，只复用现有 `idle / DND / app exclusion / focus session` 信号判断是否适合投递。

## Governance Notes
- Requirement Brief: 用户希望久坐提醒不要再用固定时间强行打断当前输入或深度工作，而是在不烦人的前提下提醒起身活动；本轮聚焦于 Tauri host 的提醒投递状态机与对应状态文案，不重做设置页布局或 break window 视觉。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 已冻结为“到点先等空档、等待过久只 soft nudge 一次、检测到短暂停顿后再开 break window”；不改 break CTA、strict/manual finish、tray 菜单结构和设置页信息架构。
- Execution Safety Block: service_impact=仅限本地提醒节奏与状态文案；touches_running_service=no；backup_required=no；backup_plan=以 Git diff、Rust tests、cargo check 与 docs 变更为回滚边界；rollback_plan=回退 `state.rs`、locale 文案与任务 docs；destructive_operations=替换默认提醒投递逻辑；operator_approval_required=no；rationale=不涉及权限、数据迁移、外部服务、副作用或新增依赖。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked（当前 session 未获用户显式授权，运行策略禁止直接 `spawn_agent`）
- Escalation Summary: 当前无阻塞；若仅靠 idle 信号无法得到足够稳定的“专注中”判断，再升级讨论是否需要 OS 级输入探测。
- Retention Decision: keep

## Notes
- 现有痛点根因不是“间隔值不对”，而是调度域缺失“该提醒已经到点，但用户仍在高活跃输入中”的中间态。
- 本任务预计会同步更新 `docs/Architecture.md`、`docs/UI.md`、`docs/SettingsInventory.md` 和 `docs/CHANGELOG.md`。
- 已完成 `state.rs`、中英文 locale、evidence README 与长期文档同步；`cargo fmt` 因本机缺少 `rustfmt` 组件未执行。
