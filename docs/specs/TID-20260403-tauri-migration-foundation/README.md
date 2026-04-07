# Task-ID: TID-20260403-tauri-migration-foundation

## Meta
- Title: Tauri 2 迁移基础骨架
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
- 明确采用 `Tauri 2 + React + TypeScript + Rust host` 作为跨平台重构的新桌面壳层。
- 旧 `app/**` 保留为领域逻辑参考，不再作为新架构模板或 UI 复制源。
- 新增 `apps/desktop` 作为独立迁移落点，先交付可运行宿主层，再分阶段迁移 planner / idle / DND / app exclusions。
- 为了让设计和证据采集更顺滑，`apps/desktop/src/App.tsx` 同时支持 Tauri runtime 与浏览器 preview。

## Governance Notes
- Requirement Brief: 在不继续加码 Electron 的前提下，新建一套可运行的 Tauri 2 桌面基础壳，覆盖新宿主层、最小可用 dashboard、tray/shortcut/autostart 等核心宿主能力，并保留旧仓库作为领域迁移参考。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 新增一个独立的 Tauri 桌面 dashboard。primary flow 为打开主窗口后查看宿主能力和迁移队列，并可直接开启 focus session、切换 launch on login、隐藏窗口；fallback flow 为浏览器 preview，仅用于 UI 预览与证据采集，不宣称具备原生宿主能力。
- Execution Safety Block: service_impact=新增平行 Tauri 壳层、不替换现有 Electron 运行时；touches_running_service=no；backup_required=no；backup_plan=保留旧 `app/**` 作为迁移真源并通过 VCS 管理新增目录；rollback_plan=删除 `apps/desktop/**` 与根脚本入口、回退 docs；destructive_operations=删除 Tauri 脚手架残留文件与此前第一轮清理的外围资产；operator_approval_required=no；rationale=用户已明确同意跨平台路线切换到 Tauri 2，本轮不触碰现有线上主体逻辑。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked（developer policy 限制：未获用户显式授权不得 spawn_agent）
- Escalation Summary: 无；本轮骨架搭建、预览兜底与验证已收敛。
- Retention Decision: keep

## Notes
- 开发态已通过 `npm run tauri:dev` 启动并进入 `target/debug/pauza-desktop`。
- 证据与验证见 `./plan.md`、`./testplan.md` 与 `./evidence/README.md`。
