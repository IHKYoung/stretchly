# Task-ID: TID-20260404-fullscreen-break-close-fix

## Summary
- Title: 修复全屏休息跳过后黑屏
- Date: 2026-04-04
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复 fullscreen break 点击 `跳过` 后残留黑屏的问题，让 break 关闭链路正确退出 fullscreen 并清理窗口实例。
- In-scope:
  - 修复 `shell.rs::close_break_window()` 的 fullscreen 关闭路径。
  - 保持 break action command 和 state 逻辑不变。
  - 补齐本任务 docs。
- Out-of-scope:
  - 不改 React break 页面。
  - 不改 settings schema。
  - 不改 Electron legacy。
- Assumptions:
  - 黑屏来自 fullscreen break 窗口只被 `hide()`，没有完整 teardown。
  - 旧 Electron 的 `hide() + destroy()` 关闭策略可作为 source basis。
  - 即使本地无法完整复现 UI，代码路径修复依然成立。
- Risks:
  - fullscreen 退出时机在个别平台上可能仍有短暂过渡动画。
  - 由于本轮没有真实 fullscreen 交互证据，最终现实确认仍依赖用户本机。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户在 fullscreen break 点击 `跳过` 后，应直接退出 break，不再留黑屏。
- Fallback / secondary flow: `完成` / `稍后` 共用同一 `close_break_window()` 链路，也应受益。
- User-visible boundary: 仅 break fullscreen 关闭行为。
- Key visible states / transitions:
  - fullscreen break visible
  - 点击跳过
  - 退出 fullscreen
  - break 窗口销毁

## Goal
- 让 fullscreen break 在结束/跳过/延后时真正关闭，而不是留下黑色全屏壳。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `docs/specs/TID-20260404-fullscreen-break-close-fix/*`
  - `docs/plans/2026-04-04.md`
  - `docs/logs/2026-04-04.md`
  - `docs/CHANGELOG.md`
- Out-of-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/state.rs`
  - 新 UI 证据采集自动化

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/commands.rs`
  - `apps/desktop/src-tauri/src/state.rs`
  - `app/main.js`
- Related docs/specs/logs reviewed:
  - `AGENTS.md`
  - `docs/Architecture.md`
  - `docs/logs/2026-04-03.md`
  - 相邻 break surface / break window tasks
- Why these are sufficient:
  - 已覆盖当前 Tauri break 关闭链路、旧 Electron 关闭策略和当前用户报告的 bug 边界。

## Acceptance Criteria (AC)
- AC1: `close_break_window()` 不再只 `hide()` break windows。
- AC2: fullscreen break 在关闭前会先退出 fullscreen。
- AC3: break window 会被 `destroy()`，避免复用残留黑壳实例。
- AC4: `cargo check`、`typecheck`、`build` 通过。
- AC5: docs 明确记录本轮为代码路径修复，最终现实确认仍需用户本机点击 fullscreen break 验证。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen
- Primary flow: fullscreen break 内点击 `跳过`，break 正常退出，不残留黑屏。
- Fallback / secondary flow: `完成` 和 `稍后` 共用相同关闭链路。
- Interaction authority / ownership boundary: 本任务只改宿主层关闭策略，不改按钮语义和 scheduler。
- Visible entrypoints / handoff cues: break CTA `跳过 / 完成 / 稍后`。
- In-scope interactions:
  - fullscreen break 结束
  - fullscreen break 跳过
  - fullscreen break 延后
- Out-of-scope interactions:
  - break window 打开策略
  - 主窗口布局
  - settings page
- Interaction acceptance criteria:
  - close path 先退出 fullscreen，再销毁窗口。
  - 不再保留仅 `hide()` 的 fullscreen close 行为。
- Validator expectation: direct interaction 字段已补齐。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅改 Tauri break window 关闭路径。
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 VCS、cargo/typecheck/build 与旧 Electron 关闭策略旁证。
- rollback_plan: 回退 `shell.rs` 与本任务 docs。
- destructive_operations: 替换当前 break close 行为。
- operator_approval_required: no
- rationale: 用户明确指出宿主 bug，本轮不涉及数据和外部副作用。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 修复 `close_break_window()` 的 fullscreen teardown
  - DoD: 先退出 fullscreen，再 hide + destroy。
- [x] Task-2: 运行编译链并补 docs
  - DoD: cargo/typecheck/build 通过，spec/logs/plans/changelog 收口。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260404-fullscreen-break-close-fix/evidence/
- Interaction validation note: 本轮没有新鲜 fullscreen 交互截图，证据以代码路径修复与编译链为主，并显式记录现实确认缺口。
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: 无新增编译错误
  - disabled: N/A
  - success: `shell.rs` 关闭链路已变更，编译链通过

## Observability / Debug Plan
- Logs: 无新增日志；依赖用户现实复核 fullscreen skip 行为。
- Error codes: 无新增错误码。
- Trace/metrics (optional): 无。
- Debug flags (optional): 无。

## Risks & Rollback
- Risks:
  - 部分平台 fullscreen 退出仍可能有系统级过渡动画。
  - 若 `destroy()` 带来新的窗口生命周期副作用，需要再细调。
- Rollback plan:
  - 回退 `shell.rs` 与 docs。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 对照旧 Electron 窗口关闭策略，确认 Tauri 回归点。
  2. 修复 `close_break_window()`，对 fullscreen break 做退出 fullscreen + destroy。
  3. 跑 `cargo check`、`typecheck`、`build`，补 docs。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户在当前线程明确报告 fullscreen break 点击跳过后直接黑屏。
