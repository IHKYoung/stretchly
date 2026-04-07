# Task-ID: TID-20260407-finish-break-no-exit

## Summary
- Title: 修复完成休息后应用退出
- Date: 2026-04-07
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复 Tauri break prompt 点击“完成休息”后应用直接退出的问题，让 break 关闭后应用继续在后台存活。
- In-scope:
  - 调整 break CTA 对应命令的 break window close timing。
  - 保留既有 fullscreen break close 修复。
  - 补齐本任务 docs / evidence / changelog。
- Out-of-scope:
  - 不改 React break prompt 结构和文案。
  - 不改 `PauzaState` 调度逻辑。
  - 不改 Electron legacy。
- Assumptions:
  - 退出更像是当前 break webview 在 `invoke` 回包前被同步 destroy 引起的生命周期竞态。
  - 现有 `finish_current_break()` 状态转移本身是正确的，问题集中在 command -> shell close 链路。
- Risks:
  - 极小延迟 close 可能带来轻微窗口过渡差异。
  - 当前仍缺少自动化的真实原生 UI 交互证据，最终现实确认依赖用户本机点击。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: break prompt 可见时点击“完成休息”，窗口关闭但应用不退出。
- Fallback / secondary flow: `稍后` / `跳过` 共用相同 deferred close 策略，也应保持后台存活。
- User-visible boundary: 仅 Tauri break prompt 关闭后的宿主行为。
- Key visible states / transitions:
  - break prompt visible
  - 用户点击 CTA
  - 命令返回 snapshot
  - break window teardown
  - 应用继续后台运行

## Goal
- 让 break 完成/跳过/延后都能安全关闭 break prompt，而不是把整个应用一并带退出。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/commands.rs`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `docs/specs/TID-20260407-finish-break-no-exit/*`
  - `docs/plans/2026-04-07.md`
  - `docs/logs/2026-04-07.md`
  - `docs/CHANGELOG.md`
- Out-of-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/state.rs`
  - `app/**`

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/commands.rs`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/App.tsx`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/specs/TID-20260404-fullscreen-break-close-fix/plan.md`
  - `docs/specs/TID-20260404-fullscreen-break-close-fix/testplan.md`
  - `docs/specs/TID-20260404-fullscreen-break-close-fix/evidence/README.md`
- Why these are sufficient:
  - 已覆盖 break CTA 前端入口、Tauri command/state/shell 生命周期边界，以及相邻 fullscreen close 修复的约束。

## Acceptance Criteria (AC)
- AC1: break CTA 命令不再在 `invoke` 回包前同步 destroy 当前 break webview。
- AC2: 点击“完成休息”后，break prompt 正常关闭，应用继续保持后台存活。
- AC3: `稍后` / `跳过` 共用同一 deferred close 策略，不引入回归。
- AC4: fullscreen break close 仍保留 `set_fullscreen(false)` + destroy 的 teardown 语义。
- AC5: `cargo check`、`cargo test`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build` 与 workflow docs validator 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen
- Primary flow: break prompt 中点击“完成休息”后，窗口关闭但应用继续后台运行。
- Fallback / secondary flow: `稍后` / `跳过` 复用同一 close timing 修复。
- Interaction authority / ownership boundary: 只改宿主关闭时序，不改 CTA 语义和可见布局。
- Visible entrypoints / handoff cues: break prompt 底部三个 CTA。
- In-scope interactions:
  - 完成休息
  - 稍后
  - 跳过
- Out-of-scope interactions:
  - settings page
  - break prompt 视觉样式
  - scheduler 触发时机
- Interaction acceptance criteria:
  - 命令先回包，窗口后 teardown。
  - 应用不应表现为退出。
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
- service_impact: 仅限 break CTA 对应的宿主关闭时序。
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖当前 Git 工作区差异、编译链和相邻 fullscreen close fix 作为回退参考。
- rollback_plan: 回退 `commands.rs`、`shell.rs` 与本任务 docs。
- destructive_operations: 替换 break CTA 的同步 close 为 deferred close。
- operator_approval_required: no
- rationale: 用户报告的是本地宿主 bug，本轮不涉及数据、权限、网络和外部副作用。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 调整 break CTA 的宿主关闭时序
  - DoD: `finish/skip/postpone` 先返回 snapshot，再异步调用真实 break window teardown。
- [x] Task-2: 保留 close path 兼容 fullscreen 修复并完成验证
  - DoD: `close_break_window()` 仍负责 fullscreen 退出 + destroy，编译链与 docs validator 通过。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260407-finish-break-no-exit/evidence/
- Interaction validation note: 以代码路径修复和编译链为主，显式保留“需用户本机点一次完成休息”的现实确认缺口。
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: 无新增编译错误
  - disabled: N/A
  - success: break CTA 命令与 `close_break_window()` 已分离为“先回包，后 teardown”

## Observability / Debug Plan
- Logs: 继续依赖 `DesktopSnapshot.lastAction`、tray 状态和命令执行结果；本轮不新增日志字段。
- Error codes: 无新增错误码。
- Trace/metrics (optional): `cargo check` / `cargo test` / `typecheck` / `build` 输出作为实现自证。
- Debug flags (optional): 无。

## Risks & Rollback
- Risks:
- 极小延迟 teardown 可能带来短暂但可接受的窗口过渡。
- 若竞态仍存在，后续可能需要升级为前端事件驱动自关闭。
- Rollback plan:
  - 回退 `commands.rs` / `shell.rs` 到同步 close 版本，并保留 docs 说明。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 复盘 break CTA -> command -> shell close 的当前时序与相邻 fullscreen 修复约束。
  2. 将 break CTA 命令改为 deferred close，避免在 `invoke` 回包前 destroy 当前窗口。
  3. 运行 Rust / 前端构建链与 docs validator，补齐 evidence / logs / changelog。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户已在当前线程直接报告“点击完成休息后程序会直接退出”，并要求修复该 bug。
