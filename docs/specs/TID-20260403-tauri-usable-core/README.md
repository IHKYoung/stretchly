# Task-ID: TID-20260403-tauri-usable-core

## Meta
- Title: Tauri 可用核心闭环
- Date: 2026-04-03
- Level: complex  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-03.md
- Log (daily): ../../logs/2026-04-03.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 不再继续扩写迁移展示页，而是直接把 `apps/desktop` 收敛为真实产品工作台。
- Tauri host 现在承担可运行的调度真源：设置持久化、休息调度、pause/focus、自然休息、DND、应用排除、tray/shortcut、break prompt。
- 浏览器 preview 继续保留，但只作为同一套设置页与 break prompt 的静态预览，不再承载“迁移叙事”。

## Governance Notes
- Requirement Brief: 将 Tauri 2 从基础壳推进到可用的久坐干预闭环，迁入最小可用核心行为，前台只保留极简设置页与 break prompt。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: primary flow 为打开设置页、查看状态、调整节奏与暂停/心流；secondary flow 为 break prompt 上完成/推迟/跳过；浏览器 preview 仅保留同构 UI 预览。
- Execution Safety Block: service_impact=新增可运行的 Tauri 调度闭环、但不替换 Electron 正式入口；touches_running_service=no；backup_required=no；backup_plan=保留旧 Electron 代码与前一阶段 Tauri foundation 作为 source basis；rollback_plan=回退 `apps/desktop/**` 与本次 docs 更新；destructive_operations=删除旧展示型前台实现；operator_approval_required=no；rationale=用户明确要求“继续改，直到能用为止”，且本轮不触碰线上 Electron 主体。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked（developer policy 限制：未获用户显式授权不得 spawn_agent）
- Escalation Summary: 无；编译、构建与 dev runtime 均已收敛。
- Retention Decision: keep

## Notes
- `npm run desktop:dev` 已启动到 `target/debug/pauza-desktop`，当前 dev 进程保持运行。
- 证据见 `./evidence/README.md`、`settings-snapshot.md`、`break-snapshot.md` 与 `browser-console.log`。
