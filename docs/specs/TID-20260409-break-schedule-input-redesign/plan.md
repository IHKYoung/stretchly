# Task-ID: TID-20260409-break-schedule-input-redesign

## Summary
- Title: 重构微休息与休息的预设和手动输入
- Date: 2026-04-09
- Level: moderate
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 将微休息/休息的候选项改得更合理，并修复手动输入不能先清空再输入的问题。
- In-scope:
  - 4 组节奏 preset 值
  - preset + 手动输入的组合交互
  - 对应 helper 与回归测试
- Out-of-scope:
  - 修改设置 schema
  - 修改自动保存策略
- Assumptions:
  - 当前用户主要在节奏页调整这 4 组值
- Risks:
  - 输入草稿与父级数值同步错误
- Interaction impact: none
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让 preset 更均衡，同时恢复可用的自定义输入体验。

## Scope
- In-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/lib/settings-controls.ts`
  - `test/desktopSettingsControls.js`
- Out-of-scope:
  - Rust settings schema
  - 其它非节奏类设置输入

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/components/ui/segmented-control.tsx`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - 当日对话中的用户反馈
- Why these are sufficient:
  - 已覆盖当前 preset 组件、受控输入实现和目标交互。

## Acceptance Criteria (AC)
- AC1: 4 组节奏 preset 全部为 5 个候选项。
- AC2: 自定义输入框允许临时清空，并在 blur/Enter 时提交 clamp 后的值。
- AC3: `npm test`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build` 通过。

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
- service_impact: 设置页节奏输入交互
- touches_running_service: no
- backup_required: no
- backup_plan: 以测试、typecheck、build 为边界
- rollback_plan: 回退 `App.tsx`、helper 与测试
- destructive_operations: none
- operator_approval_required: no
- rationale: 纯前端设置交互修复

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收敛 4 组 preset 为 5 个候选项
  - DoD: 节奏 preset 分布统一且更均衡。
- [x] Task-2: 引入草稿式数字输入提交逻辑
  - DoD: 可先清空再输入，提交时再 clamp。
- [x] Task-3: 补测试
  - DoD: preset 和数字提交 helper 可被自动验证。

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-break-schedule-input-redesign/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: N/A
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 自定义输入与 preset 之间的同步状态不一致
- Rollback plan:
  - 回退 `App.tsx` 和 `settings-controls.ts`

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 调整 4 组 preset 候选项。
  2. 将输入框改为草稿式提交。
  3. 补测试并验证前端构建。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes
- Approved: yes（orchestrator 已按当前 session 授权路由批准执行）
