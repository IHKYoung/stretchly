# Task-ID: TID-20260411-settings-freeze-root-cause-fix

## Meta
- Title: 修复设置点击与时间修改卡死
- Date: 2026-04-11 ~ 2026-04-12
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (init): ../../plans/2026-04-11.md
- Plan (completion): ../../plans/2026-04-12.md
- Log (init): ../../logs/2026-04-11.md
- Log (completion): ../../logs/2026-04-12.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 这次不再把问题定义成“某个输入框还没修干净”，而是明确收敛为“autosave 并发 + 宿主无差别全量刷新”两层根因。
- 设置页保存链路必须串行化，避免多个 `update_settings` 在前端连续点击/改时间时并发飞向宿主。
- Rust `update_settings` 不再每次都无条件重绑快捷键和整棵 tray `set_menu()`；只有真的受影响时才做对应刷新。

## Governance Notes
- Requirement Brief: 用户报告设置页依然容易卡死，具体表现为点击设置项会卡死、调整时间后也会卡死；本任务聚焦 `App.tsx` autosave 链路与 Rust `update_settings` 的宿主刷新策略，不扩到新的设置 schema 或调度规则。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: frozen before code；本轮只修设置点击/改时间的保存与宿主刷新链路，不改设置页布局、文案或 break 调度语义。
- Execution Safety Block: service_impact=仅限桌面端设置 autosave 与 Rust host 设置应用链路；touches_running_service=no；backup_required=no；backup_plan=`npm test` + `npm run typecheck` + `npm --prefix apps/desktop run build` + `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` + `python3 scripts/validate_workflow_docs.py --mode manual`；rollback_plan=回退 `apps/desktop/src/App.tsx`、`apps/desktop/src-tauri/src/{commands,state}.rs` 与对应 docs；destructive_operations=none；operator_approval_required=no；rationale=纯本地桌面端交互/宿主修复，无外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked（当前 session 未获用户显式 delegation 授权，遵循上层工具策略不调用 `spawn_agent`）
- Escalation Summary: 无；已能在当前代码路径内找到并收敛根因。
- Retention Decision: keep

## Notes
- 本任务的重点是减少宿主侧“设置一次就做全量刷新”的主线程压力，而不是继续给单个控件打补丁。
- 任务在 `2026-04-11` 建立 Task-ID 并完成实现，最终验证与结案发生在 `2026-04-12 00:07 CST` 之后，因此 daily plan/log 同时落在两天切片。
- 交互证据以测试/构建/命令链和根因分析为主，见 `evidence/README.md`。
