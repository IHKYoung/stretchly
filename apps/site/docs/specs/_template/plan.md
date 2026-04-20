# Task-ID: {{TASK_ID}}

## Summary
- Title: {{TITLE}}
- Date: {{DATE}}
- Level: {{LEVEL}}
- Lane: {{LANE}}
- Execution Profile: {{EXECUTION_PROFILE}}
- Status: INIT

## Requirement Brief
- Goal restatement:
- In-scope:
- Out-of-scope:
- Assumptions:
- Risks:
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- (TBD)

## Scope
- In-scope:
- Out-of-scope:

## Source Basis (Read Before Code)
- Related code/files reviewed:
- Related docs/specs/logs reviewed:
- Why these are sufficient:

## Acceptance Criteria (AC)
- AC1: (TBD)
- AC2: (TBD)

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
- Execution Profile: {{EXECUTION_PROFILE}}
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: {{REQUIRED_ROLES}}
- Delegation Policy: after the user grants orchestration authority, `orchestrator` may autonomously `spawn_agent`
- Automatic Delegation Triggers: `moderate/complex` | long-running task | independent sidecar tasks
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact:
- touches_running_service:
- backup_required:
- backup_plan:
- rollback_plan:
- destructive_operations:
- operator_approval_required:
- rationale:

## Task Breakdown & Definition of Done (DoD)
- [ ] Task-1: (TBD)
  - DoD: (TBD)
- [ ] Task-2: (TBD)
  - DoD: (TBD)

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/{{TASK_ID}}/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs:
- Error codes:
- Trace/metrics (optional):
- Debug flags (optional):

## Risks & Rollback
- Risks:
- Rollback plan:

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. (TBD)

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: {{APPROVAL_NEEDED}}  <!-- yes | no；仅高风险动作写 yes -->
- Approved: {{APPROVED_STATE}}
