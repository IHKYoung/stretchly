# Task-ID: TID-20260403-tauri-core-parity-docs

## Summary
- Title: Tauri core parity 文档收口
- Date: 2026-04-03
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 仅基于现有仓库事实，把 Tauri core parity 的 specs/logs/plans 和总览文档补齐到可追溯状态。
- In-scope:
  - `docs/specs/TID-20260403-tauri-core-parity/*`
  - `docs/plans/2026-04-03.md`
  - `docs/logs/2026-04-03.md`
  - `docs/CHANGELOG.md`
  - `docs/CodeMap.md`
  - `docs/Architecture.md`
  - `docs/UI.md`
  - 本 docs task 自身的 spec 收口
- Out-of-scope:
  - 任意生产代码
  - 任意测试代码
  - 新截图/录屏采集
- Assumptions:
  - 已完成的 Tauri parity 代码事实以当前仓库为准。
  - 若缺真实可视化证据，需要诚实记录为 gap，而不是伪造“已完成”。
- Risks:
  - 当天其他占位 task 会导致 workflow validator 失败。
  - 文档若写得比代码更激进，会造成后续误导。
- Interaction impact: none
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让与 `TID-20260403-tauri-core-parity` 相关的文档链条完整、可审计、可追溯。

## Scope
- In-scope:
  - 仅 docs 写入
- Out-of-scope:
  - 任何源代码、测试、脚本行为修改

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `package.json`
  - `apps/desktop/src-tauri/src/{state,shell,commands,engine,lib,platform}.rs`
  - `apps/desktop/src/{App.tsx,styles.css}`
- Related docs/specs/logs reviewed:
  - `docs/specs/TID-20260403-tauri-core-parity/*`
  - `docs/specs/TID-20260403-tauri-migration-foundation/*`
  - `docs/specs/TID-20260403-tauri-usable-core/*`
  - `docs/specs/TID-20260403-tauri-i18n-resource-layer/*`
  - `docs/plans/2026-04-03.md`
  - `docs/logs/2026-04-03.md`
  - `docs/CHANGELOG.md`
  - `docs/CodeMap.md`
  - `docs/Architecture.md`
  - `docs/UI.md`
- Why these are sufficient:
  - 它们覆盖了本次文档收口所需的全部事实来源与需同步索引。

## Acceptance Criteria (AC)
- AC1: `TID-20260403-tauri-core-parity` 五件套 spec + evidence report 不再保留占位。
- AC2: 当日 `logs/plans` 对 core parity 与 docs 收口任务都具备完整 warmup / execution / verification 字段。
- AC3: `CHANGELOG`、`CodeMap`、`Architecture`、`UI` 与当前仓库事实一致。
- AC4: role scope 校验通过；workflow docs validator 若仍失败，失败原因必须被明确记录。

## Interaction Freeze
- Freeze status: N/A
- Primary flow: N/A
- Fallback / secondary flow: N/A
- Interaction authority / ownership boundary: N/A
- Visible entrypoints / handoff cues: N/A
- In-scope interactions: N/A
- Out-of-scope interactions: N/A
- Interaction acceptance criteria: N/A

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: scribe
- Execution Mode Policy: `single-agent-fallback` allowed because task is trivial and Required Roles 仅 `scribe`
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: scribe -> Orchestrator
- Safe-local Command Route: repo-local docs validation only
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: docs only
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 VCS
- rollback_plan: 回退本次文档修改
- destructive_operations: none
- operator_approval_required: no
- rationale: 本任务只处理文档落盘与门禁资产

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 补全 core parity spec 与 evidence report
  - DoD: 五件套齐全，无 `TBD/INIT`
- [x] Task-2: 同步 daily logs/plans 与总览文档
  - DoD: `CHANGELOG`、`CodeMap`、`Architecture`、`UI` 与当天 logs/plans 更新完成
- [x] Task-3: 跑文档门禁校验
  - DoD: role scope 校验通过；workflow validator 结果被记录

## Evidence Plan (UI / E2E)
- Evidence required: no
- Owner: scribe
- Artifact path: N/A
- Interaction validation note: 本任务自身不涉及 UI 变更，只记录别的任务尚缺哪些证据。
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs: 当日 `docs/logs/2026-04-03.md`
- Error codes: validator 输出
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 写错代码事实
  - 漏填当天其他占位 task 导致 validator 失败
- Rollback plan:
  - 回退本次 docs 变更

## Sequential Phases
- phase_execution: N/A
- phase_confirmation_policy: N/A
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 读取 parity 相关代码与现有文档。
  2. 补全 core parity spec / logs / plans / 总览文档。
  3. 跑 role scope 和 workflow docs validator。
  4. 记录仍缺的证据项。

## Definition of Done (DoD)
- 文档中不再保留本任务相关的 `TBD/INIT` 占位。
- 校验结果与剩余缺口都被记录。

## Approval
- Approval needed: no
- Approved: N/A（trivial docs-only task）
