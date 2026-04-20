# Task-ID: TID-20260413-fullscreen-break-current-space-fix

## Meta
- Title: 修复全屏工作区下 break 未覆盖当前界面
- Date: 2026-04-13
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-13.md
- Log (daily): ../../logs/2026-04-13.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Evidence Report: ./evidence/README.md

## Decision Log
- 根因优先假设为 `shell.rs` 当前工作树把 macOS break window 的 `MoveToActiveSpace` 与 `FullScreenAuxiliary` 从 native `collectionBehavior` 中移除，导致全屏 Space 覆盖能力回退。
- 本轮不重写调度状态机；`engine.rs` 仍通过 `show_break_window()` 触发 break，修复面只落在宿主浮出策略与回归测试。

## Governance Notes
- Requirement Brief: 用户明确报告“当我正在其它软件全屏工作时，到点后的 Pauza 没覆盖当前界面，也没有弹窗提示，只在另一个屏幕自己开始休息”；本任务据此只修复 macOS break window 对当前 fullscreen Space 的可见性回归，不改 scheduler、前端 UI 或设置 schema。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 到点后的 break prompt 必须重新出现在用户当前正在看的 fullscreen Space；本轮只改宿主浮出策略，不改 React break 页面内容、CTA 或调度决策。
- Execution Safety Block: service_impact=仅限本地 desktop shell 的 macOS break window overlay 行为、测试与 docs；touches_running_service=no；backup_required=no；backup_plan=`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` + `npm --prefix apps/desktop run build` + `python3 scripts/validate_workflow_docs.py --mode manual`；rollback_plan=回退 `shell.rs`、相关测试和 docs；destructive_operations=none；operator_approval_required=no；rationale=纯本地桌面端 bug 修复，无数据、提权或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 当前 session 未获用户显式授权调用 `spawn_agent`，且上层工具策略要求仅在用户明确要求子 agent 时才可 delegation；因此在限定范围内采用单 agent 完成本轮 root cause 修复、验证与文档闭环。
- Retention Decision: keep

## Notes
- 当前 `apps/desktop/src-tauri/src/shell.rs` 已存在未提交改动；本任务会在其基础上继续修复，不覆盖已有通知权限与其他工作树修改。
- 真实 fullscreen Space 视觉确认仍需你本机手动 spot-check；本轮 evidence 以代码路径与构建验证为主。
