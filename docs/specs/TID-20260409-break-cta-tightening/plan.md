# Task-ID: TID-20260409-break-cta-tightening

## Summary
- Title: 收紧 break 完成与延后按钮时机
- Date: 2026-04-09
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: break 不再允许提前完成，`Later` 只允许在倒计时开始后的前 10 秒。
- In-scope: `state.rs` 中的 `finish_current_break` / `can_postpone`，`App.tsx` 的 break CTA 显示，以及相关测试和文档。
- Out-of-scope: skip 策略、manual finish 设置项本身、break 调度模型。
- Assumptions: 当前用户只要求收紧 break CTA 时机，不要求改动 break 页面其他交互。
- Risks: 若只改前端可能被命令绕过；若 10 秒边界实现错误，延后窗口会和预期不一致。
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让 break CTA 的可用性与用户期望一致：无提前完成，延后窗口固定为前 10 秒。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/App.tsx`
  - `docs/{UI,SettingsInventory,CHANGELOG}.md`
  - 本任务 specs 与当日 plans/logs
- Out-of-scope:
  - `apps/desktop/src/locales/messages/*.json`
  - `apps/desktop/src-tauri/src/shell.rs`
  - skip / reminder / schedule 逻辑

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src/App.tsx`
- Related docs/specs/logs reviewed:
  - `docs/UI.md`
  - `docs/SettingsInventory.md`
  - `docs/CHANGELOG.md`
  - 当日相邻 break / smart reminder 任务记录
- Why these are sufficient:
  - 当前规则集中在 CTA 可用性；后端状态判断和前端按钮显示都集中在这两处。

## Acceptance Criteria (AC)
- AC1: 活跃 break 倒计时阶段调用 `finish_current_break` 不会成功。
- AC2: `can_postpone` 只在 break 开始后的前 `10s` 为真。
- AC3: 前端普通倒计时阶段不显示完成按钮，只在 `manualAwaiting` 显示 `Resume work`。
- AC4: `cargo test`、`npm test`、desktop build 与 docs validator 通过。

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
- service_impact: 仅限 break CTA 时机规则、对应测试与文档
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 Rust/前端测试、desktop build 与 docs validator
- rollback_plan: 回退 `state.rs`、`App.tsx`、测试与文档
- destructive_operations: none
- operator_approval_required: no
- rationale: 不涉及线上服务、权限、数据迁移或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收紧 host 侧 finish / postpone 判定
  - DoD: `finish_current_break` 只能在 `manualAwaiting` 成功，`can_postpone` 固定为前 10 秒窗口
- [x] Task-2: 收紧 break 页 CTA 显示并补验证/文档
  - DoD: 前端不再显示提前完成按钮，测试和 docs gate 通过

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-break-cta-tightening/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: 沿用现有 `last_action`
- Error codes: `postpone_current_break` 继续使用现有 disabled 文案
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - CTA 规则前后不一致
  - 10 秒窗口边界写错
- Rollback plan:
  - 回退 `state.rs` / `App.tsx` / 测试与文档
  - 重新验证 Rust/前端链路

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 修改 host 侧 finish / postpone 判定
  2. 修改前端 CTA 显示
  3. 补测试并更新 UI/Inventory/CHANGELOG 与 task docs

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
