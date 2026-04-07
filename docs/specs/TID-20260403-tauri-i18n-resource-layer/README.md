# Task-ID: TID-20260403-tauri-i18n-resource-layer

## Meta
- Title: Tauri 多语言资源化
- Date: 2026-04-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Links
- Plan (daily): ../../plans/2026-04-03.md
- Log (daily): ../../logs/2026-04-03.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 参考旧 Electron 的 `i18next + locale JSON` 组织方式，但不新增 Tauri 前端依赖，先用共享 locale JSON + 轻量 lookup 实现。
- 前后端共用 `apps/desktop/src/locales/{zh-CN,en}.json`，避免 React 与 Rust 各自维护一套文案。
- 默认语言恢复为中文，设置页只保留一个最小语言选择入口，不把设置页重新做成展示页。

## Governance Notes
- Requirement Brief: 将 Tauri 多语言从组件内联文案重构为共享 locale 资源，覆盖设置页、tray、break prompt 与运行时状态。
- Interaction Impact: none
- Interaction Freeze: 不适用；本轮不改变页面结构与交互路径，只替换文案来源并恢复语言切换。
- Execution Safety Block: service_impact=仅重构 Tauri 端文案管理与默认语言，不改调度语义；touches_running_service=no；backup_required=no；backup_plan=依赖 VCS 回退；rollback_plan=恢复 locale 资源与调用点；destructive_operations=替换临时内联文案方案；operator_approval_required=no；rationale=用户明确要求采用优雅的 i18n 管理方式。
- Approval Owner: orchestrator
- Warmup Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；本轮在单 agent 范围内收敛。
- Retention Decision: keep

## Notes
- 相关运行时文案现统一由 `apps/desktop/src-tauri/src/i18n.rs` 读取前端 locale JSON。
