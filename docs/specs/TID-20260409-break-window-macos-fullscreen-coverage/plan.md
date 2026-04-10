# Task-ID: TID-20260409-break-window-macos-fullscreen-coverage

## Summary
- Title: 修复 macOS 全屏工作区下 break 窗口覆盖当前工作屏幕
- Date: 2026-04-09
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复 macOS 全屏工作区里 break 窗口没有覆盖当前工作屏幕的问题，让现有 break 提示能在当前 active Space 正常出现。
- In-scope: `shell.rs` 的 macOS break window 原生展示策略、对应测试、Architecture/CHANGELOG 与任务 docs。
- Out-of-scope: React break 页面布局、多屏目标选择模型、休息调度状态机。
- Assumptions: 根因主要在 macOS 的 Space / window level / collection behavior，而不是前端样式或 schedule 逻辑。
- Risks: 原生层级设得过高可能影响普通 windowed break 的观感；原生 selector 调用错误会直接影响 break window 显示。
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让当前 break 窗口在 macOS 全屏工作流中能可靠覆盖当前工作屏幕。

## Scope
- In-scope:
- Out-of-scope:
- In-scope:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `docs/Architecture.md`
  - `docs/CHANGELOG.md`
  - `docs/specs/TID-20260409-break-window-macos-fullscreen-coverage/*`
  - 当日 `docs/plans/2026-04-09.md` 与 `docs/logs/2026-04-09.md`
- Out-of-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/state.rs`
  - locale 文案与 break 页面视觉层

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/commands.rs`
  - `apps/desktop/src-tauri/src/state.rs`
- Related docs/specs/logs reviewed:
  - `docs/Architecture.md`
  - `docs/CHANGELOG.md`
  - 当日相邻 smart reminder / tray 任务记录
  - 本地 Tauri 2 / tao macOS window API 源码（`ns_window`, `collectionBehavior`, `setLevel`）
- Why these are sufficient:
  - 当前问题集中在 break window 的宿主显示与平台行为；`shell.rs` 已覆盖创建、定位、显示和 fullscreen 切换，结合本地 Tauri / tao 的 macOS API 即可定位和修复。

## Acceptance Criteria (AC)
- AC1: macOS break 窗口会显式带上 `CanJoinAllSpaces | FullScreenAuxiliary`。
- AC2: macOS fullscreen / windowed break 都会在显示时使用更合适的原生窗口层级，并主动 front 到当前 Space。
- AC3: `cargo test` 与 `npm --prefix apps/desktop run build` 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: N/A
- Primary flow: N/A
- Fallback / secondary flow: N/A
- Interaction authority / ownership boundary: N/A
- Visible entrypoints / handoff cues: N/A
- In-scope interactions: N/A
- Out-of-scope interactions: N/A
- Interaction acceptance criteria: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节与 Requirement Brief 中的交互字段不得继续保留 `N/A/TBD`

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 macOS break 宿主窗口展示层与相关文档
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 `cargo test`、desktop build 与 docs validator 做回归保护
- rollback_plan: 回退 `shell.rs` 中的 native helper、对应测试与文档
- destructive_operations: none
- operator_approval_required: no
- rationale: 不涉及线上服务、权限、数据迁移、付费或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 在 `shell.rs` 为 macOS break 窗口补齐 native collection behavior、window level 与 front-order 行为
  - DoD: break window helper 编译通过，且有对应单测守住关键策略常量
- [x] Task-2: 完成 Architecture / CHANGELOG / task docs 闭环
  - DoD: 不保留 `TBD/INIT` 占位，docs validator 通过

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-break-window-macos-fullscreen-coverage/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: 暂不新增；当前平台行为通过单测和构建验证
- Error codes: 沿用现有 `String` 错误上抛
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 高层级窗口在普通场景下可能显得过于强势
  - 原生 selector 调用若不兼容，会影响 break window 出现
- Rollback plan:
  - 回退 `shell.rs` 中新增的 macOS helper 与单测
  - 重新执行 `cargo test` 和 desktop build

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审计 `shell.rs` 当前 break window 的定位、fullscreen 与 workspace 行为
  2. 为 macOS 增加 `NSWindow` collection behavior / level / front-order patch
  3. 补单测并更新 Architecture / CHANGELOG / specs / daily docs

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
