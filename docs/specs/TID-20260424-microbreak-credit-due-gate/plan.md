# Task-ID: TID-20260424-microbreak-credit-due-gate

## Summary
- Title: 修复恢复结算误提前抵扣微休息
- Date: 2026-04-24
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复 smart reminder v2 中 recovery credit 的触发条件，使其只在“待投递 break 已到点且用户从 45s+ idle 返回”时生效，避免微休息在未到点时被提前自动抵扣。
- In-scope:
  - 收紧 `state.rs` 中 recovery credit 的 due gate。
  - 补充回归测试，覆盖“未到点返回不应抵扣”和“到点后返回仍可抵扣”。
  - 同步本任务 workflow/spec/log 记录。
- Out-of-scope:
  - 不改 smart reminder 的阈值曲线、UI 交互和设置项。
  - 不引入新的 interruptibility score 或 activity model。
  - 不处理上一任务遗留的 2026-04-23 signed rebuild docs 提交问题。
- Assumptions:
  - 用户当前本机设置中 `microbreakEnabled=true`、`longBreakEvery=3`，所以“只看到长休息”不是配置导致。
  - `recovery credit` 设计意图是“只结算 overdue break”，不是“结算任意 pending break”。
- Risks:
  - 若 due gate 加错位置，可能让真正 overdue 的恢复结算失效。
  - 若只修微休息不修公共 gate，long break 仍可能在未到点时被提前 defer。
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 修复 recovery credit 误结算未到点 break，恢复微休息/长休息的正常节奏。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/state.rs`
  - `docs/specs/TID-20260424-microbreak-credit-due-gate/*`
  - `docs/plans/2026-04-24.md`
  - `docs/logs/2026-04-24.md`
- Out-of-scope:
  - 前端设置页与 break window UI
  - locale 文案
  - 打包 / 提交 / 发布流程

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/engine.rs`
  - `apps/desktop/src/App.tsx`
- Related docs/specs/logs reviewed:
  - `docs/ReminderScheduling.md`
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/logs/2026-04-23.md`
- Why these are sufficient:
  - 问题根因集中在 host 调度状态机；`engine.rs` 仅负责投递动作，`App.tsx` 用于确认用户侧模式/设置语义，`ReminderScheduling.md` 用于核对设计意图与当前实现是否漂移。

## Acceptance Criteria (AC)
- AC1: 用户从 `45s+` idle 返回时，只有在当前 pending break 已到点的前提下才允许执行 recovery credit / defer / full reset。
- AC2: 当前 pending break 未到点时，返回不应自动抵扣微休息，也不应提前顺延长休息。
- AC3: 现有“到点后返回可抵扣 microbreak / defer long break”的行为保持不变。

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
- service_impact: 仅影响本地 desktop runtime 的 reminder 调度逻辑、测试与 workflow docs。
- touches_running_service: no
- backup_required: no
- backup_plan: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`python3 scripts/validate_workflow_docs.py --mode manual`
- rollback_plan: 回退 `apps/desktop/src-tauri/src/state.rs` 与本任务相关 docs/test 改动。
- destructive_operations: none
- operator_approval_required: no
- rationale: 无线上服务、权限提升、数据迁移或外部副作用；属于本地逻辑 bugfix。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 给 recovery credit 增加 due gate
  - DoD: `tick()` 中“从 idle 返回”的恢复结算只在 overdue pending break 存在时触发。
- [x] Task-2: 增加回归测试
  - DoD: 至少覆盖“未到点不抵扣”和“到点后仍抵扣”两类路径。
- [x] Task-3: 校验并更新 workflow docs
  - DoD: 本任务 spec / plans / logs 无 `TBD/INIT` 占位残留进入 DONE。

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260424-microbreak-credit-due-gate/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs:
  - 复用现有 `last_action`，重点核对 `recoveryMicrobreakCredited` / `recoveryLongBreakDeferred` 只在 overdue 场景出现。
- Error codes:
  - N/A
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - N/A

## Risks & Rollback
- Risks:
  - 由于 `recovery credit` 与 `natural break` 共用 idle return 判定，修复时可能误伤 full reset。
  - 若只看 `next_break_kind` 不看 `due_at`，问题会继续存在。
- Rollback plan:
  - 回退本任务在 `state.rs` 的 due gate 与测试新增，恢复到修复前版本。

## Outcome
- 已将 `apply_recovery_credit()` 从“看 `next_break_kind`”收紧为“看 `pending_due_kind(now)`”。
- 新增两个回归测试，确认未到点返回不会提前 credit microbreak 或 defer long break。
- 现有 overdue return 路径保持通过。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 复现并确认根因：recovery credit 在 idle return 时缺少 `due <= now` gate。
  2. 修改 `state.rs`，只对 overdue pending break 执行 recovery credit。
  3. 补单元测试并跑 Rust 测试。
  4. 更新 logs/plans/spec 并做 workflow docs 校验。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户在当前会话中报告该 bug，按默认执行策略直接进入修复闭环）
