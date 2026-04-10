# Task-ID: TID-20260409-settings-next-cycle-apply-fix

## Summary
- Title: 修复设置更新只在下一轮休息生效
- Date: 2026-04-09
- Level: moderate
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复设置更新不应中断当前 break 或重排已排队 break，而应该从下一轮休息开始生效。
- In-scope:
  - `update_settings()` 的 schedule 生效边界
  - Rust 单测
- Out-of-scope:
  - 显式 reset/pause/focus 动作
  - 设置页 UI 结构
- Assumptions:
  - 当前 break 的运行参数以 `CurrentBreak` 存储值为准
- Risks:
  - 更新 settings 后若没有 active schedule，也不能让未来 schedule 丢失
- Interaction impact: none
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 去掉设置保存对当前 break 状态机的破坏性 reset。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - Rust tests
- Out-of-scope:
  - 前端设置页布局
  - 其它显式调度动作

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/engine.rs`
  - `apps/desktop/src-tauri/src/commands.rs`
- Related docs/specs/logs reviewed:
  - 当日用户反馈
  - `docs/Architecture.md`
- Why these are sufficient:
  - 已覆盖 settings 更新入口、后台 tick 调度和运行时状态真源。

## Acceptance Criteria (AC)
- AC1: 正在进行中的 break 不因保存设置而被清空。
- AC2: 已排队的 next break 不因保存设置而重置 due time/kind。
- AC3: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm test`、`npm --prefix apps/desktop run build` 通过。

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
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 本地 break 调度状态机
- touches_running_service: no
- backup_required: no
- backup_plan: 以 Rust tests、前端 tests/build 为边界
- rollback_plan: 回退 `state.rs` 与新增测试
- destructive_operations: none
- operator_approval_required: no
- rationale: 调度逻辑修复，不涉及外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 审查 `update_settings()` 与 runtime state reset 路径
  - DoD: 明确 bug 根因为无条件 `reset_schedule(now)`。
- [x] Task-2: 修复 settings 更新生效边界
  - DoD: active break flow 保存后保持不变。
- [x] Task-3: 增加 Rust 单测
  - DoD: 当前 break 和已排队 next break 都有自动化覆盖。

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-settings-next-cycle-apply-fix/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: 沿用现有 `runtime.actions.settingsUpdated`
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 若无 active break flow 时不重排 schedule，可能导致 future schedule 丢失
- Rollback plan:
  - 回退 `update_settings()` 与新增单测

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审查 `update_settings()`、`reset_schedule()` 与 runtime state。
  2. 仅在没有 active break flow 且未被阻塞时才重排 schedule。
  3. 补 Rust 单测并验证整体构建。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes
- Approved: yes（orchestrator 已按当前 session 授权路由批准执行）
