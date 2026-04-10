# Task-ID: TID-20260409-version-unify-010

## Summary
- Title: 统一当前有效版本为 0.1.0
- Date: 2026-04-09
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 将当前仍然生效的产品/仓库版本入口统一为 `0.1.0`，避免根 package 继续停留在旧的 `1.20.0`。
- In-scope:
  - 根 `package.json`
  - 根 `package-lock.json`
  - 本任务 docs / 当日 plans / logs
- Out-of-scope:
  - 历史 release 文本
  - archive 文档
  - 依赖包自身版本号
- Assumptions:
  - `apps/desktop/package.json`、`apps/desktop/src-tauri/tauri.conf.json`、`apps/desktop/src-tauri/Cargo.toml` 已是正确的 `0.1.0`
  - 只有根 package/lock 仍残留 `1.20.0`
- Risks:
  - 根 package 与 lock 不同步
  - 历史文本中的 `1.20.0` 被误当成当前真源
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 收口当前有效版本真源，让仓库级与 desktop / tauri / cargo 的版本字段全部一致为 `0.1.0`。

## Scope
- In-scope:
  - `package.json`
  - `package-lock.json`
  - `docs/specs/TID-20260409-version-unify-010/*`
  - `docs/{plans,logs}/2026-04-09.md`
- Out-of-scope:
  - `apps/desktop/package.json`
  - `apps/desktop/src-tauri/tauri.conf.json`
  - `apps/desktop/src-tauri/Cargo.toml`
  - 历史 release / archive 记录

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `package.json`
  - `package-lock.json`
  - `apps/desktop/package.json`
  - `apps/desktop/src-tauri/tauri.conf.json`
  - `apps/desktop/src-tauri/Cargo.toml`
- Related docs/specs/logs reviewed:
  - 当日 `docs/plans/2026-04-09.md`
  - 当日 `docs/logs/2026-04-09.md`
- Why these are sufficient:
  - 已覆盖当前仍会影响现行产品/仓库版本显示和打包的所有有效版本真源，足以完成统一。

## Acceptance Criteria (AC)
- AC1: 根 `package.json` 与 `package-lock.json` 顶层版本统一为 `0.1.0`。
- AC2: `apps/desktop/package.json`、`apps/desktop/src-tauri/tauri.conf.json`、`apps/desktop/src-tauri/Cargo.toml` 保持 `0.1.0`，无残留有效版本出口为 `1.20.0`。
- AC3: `npm test`、`npm --prefix apps/desktop run build` 与 `python3 scripts/validate_workflow_docs.py --mode manual` 通过。

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
- service_impact: 仅限当前仓库/打包元数据版本号
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 Git diff、版本 grep、`npm test`、build 与 docs validator
- rollback_plan: 回退 `package.json`、`package-lock.json` 与本任务 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 不涉及运行时逻辑、数据迁移、权限或外部副作用。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 统一根 package 版本字段
  - DoD: `package.json` 与 `package-lock.json` 顶层版本均为 `0.1.0`。
- [x] Task-2: 补齐当前任务文档与验证
  - DoD: spec / plans / logs 已无占位，且验证通过。

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-version-unify-010/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs:
  - `rg -n '1\\.20\\.0|0\\.1\\.0' ...`
  - `npm test`
  - `npm --prefix apps/desktop run build`
  - docs validator 输出
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 根 package 与 lock 不同步
  - 误把历史文档中的版本号当成当前有效出口
- Rollback plan:
  - 回退 `package.json`、`package-lock.json` 与本任务 docs，然后重跑验证

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审计当前有效版本真源，确认 only root package/lock 残留 `1.20.0`。
  2. 将根 `package.json` 与 `package-lock.json` 顶层版本改为 `0.1.0`。
  3. 运行 grep / build / test / docs validator，并收口文档。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: N/A（trivial 默认直行；如需审批请手动填写）
